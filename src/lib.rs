#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
extern crate std;

use core::alloc::GlobalAlloc;

use derive_more::{Add, AddAssign, Sub, SubAssign, Neg};

#[cfg(test)]
mod test;

#[cfg(feature = "thread")]
std::thread_local! {
    static THREAD_METRICS: core::cell::Cell<Metrics> = core::cell::Cell::new(Metrics { allocated_bytes: 0, allocations: 0 });
}
#[cfg(feature = "global")]
static GLOBAL_METRICS: spin::Mutex<Metrics> = spin::Mutex::new(Metrics { allocated_bytes: 0, allocations: 0 });

fn add_assign_metrics(_metrics: Metrics) {
    #[cfg(feature = "thread")]
    { THREAD_METRICS.set(THREAD_METRICS.get() + _metrics) }
    #[cfg(feature = "global")]
    { *GLOBAL_METRICS.lock() += _metrics }
}

/// Holds metrics on memory allocations.
/// 
/// This type implements several arithmetic operations, which allows for computing metric deltas and aggregating them.
#[cfg_attr(feature = "thread", doc = r#"
```rust
let before = alloc_metrics::thread_metrics();
// ... do some work ...
let after = alloc_metrics::thread_metrics();
let delta = after - before;
```
"#)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Add, AddAssign, Sub, SubAssign, Neg)]
pub struct Metrics {
    /// The number of bytes that were allocated.
    pub allocated_bytes: isize,
    /// The number of allocations made.
    pub allocations: isize,
}

/// A global allocator type that tracks allocation metrics.
/// 
/// This type makes use of shared memory in order to aggregate metrics while still supporting arbitrary global allocator composition.
/// Because of this, there should only ever be at most one instance of it (hence being a global allocator).
pub struct MetricAlloc<A: GlobalAlloc> {
    wrapped: A,
}
impl<A: GlobalAlloc> MetricAlloc<A> {
    /// Wraps an existing global allocator into a metric allocator.
    pub const fn new(wrapped: A) -> Self {
        Self { wrapped }
    }
}
unsafe impl<A: GlobalAlloc> GlobalAlloc for MetricAlloc<A> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let res = self.wrapped.alloc(layout);
        if !res.is_null() {
            add_assign_metrics(Metrics {
                allocated_bytes: layout.size() as isize,
                allocations: 1,
            });
        }
        res
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.wrapped.dealloc(ptr, layout);
        add_assign_metrics(Metrics {
            allocated_bytes: -(layout.size() as isize),
            allocations: -1,
        });
    }
}

/// Get the current allocation metrics for the current thread.
/// 
/// Allocations in other threads will not affect this value.
#[cfg(feature = "thread")]
pub fn thread_metrics() -> Metrics {
    THREAD_METRICS.get()
}

/// Get the current allocation metrics for the entire program.
/// 
/// Allocations in other threads will affect this value, but each modification is guaranteed to be atomic.
#[cfg(feature = "global")]
pub fn global_metrics() -> Metrics {
    *GLOBAL_METRICS.lock()
}
