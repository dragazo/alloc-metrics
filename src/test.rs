use crate::*;

extern crate std;
extern crate alloc;

#[allow(unused_imports)]
use alloc::boxed::Box;

#[global_allocator]
static GLOBAL: MetricAlloc<std::alloc::System> = MetricAlloc::new(std::alloc::System);

#[cfg(feature = "thread")]
#[test]
fn test_thread() {
    fn delta<R, F: FnOnce() -> R>(f: F) -> Metrics {
        let before = thread_metrics();
        let _v = f();
        thread_metrics() - before
    }

    let _v = Box::new(123);
    let prev = thread_metrics();
    assert_ne!(prev, Metrics { allocated_bytes: 0, allocations: 0 });

    assert_eq!(delta(|| ()), Metrics { allocated_bytes: 0, allocations: 0 });
    assert_eq!(delta(|| { let _ = Box::new([0u8; 128]); 12 }), Metrics { allocated_bytes: 0, allocations: 0 });
    assert_eq!(delta(|| Box::new([0u8; 128])), Metrics { allocated_bytes: 128, allocations: 1 });
    assert_eq!(delta(|| (Box::new([0u8; 128]), Box::new(0f64))), Metrics { allocated_bytes: 136, allocations: 2 });

    assert_eq!(thread_metrics(), prev);
}

#[cfg(feature = "global")]
#[test]
fn test_global() {
    // global is really hard to test since unit tests are all one unsynchronized process - at least make sure we're getting alloc data
    let _v = Box::new(123);
    assert_ne!(global_metrics(), Metrics { allocated_bytes: 0, allocations: 0 });
}
