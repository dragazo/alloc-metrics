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

    let p = Box::new(567);
    assert_eq!(thread_metrics() - prev, Metrics { allocated_bytes: 4, allocations: 1 });
    assert_eq!(prev - thread_metrics(), Metrics { allocated_bytes: -4, allocations: -1 });
    assert_eq!(delta(|| drop(p)), Metrics { allocated_bytes: -4, allocations: -1 });

    assert_eq!(delta(|| unsafe {
        let p = alloc::alloc::alloc(alloc::alloc::Layout::array::<u16>(32).unwrap());
        assert!(!p.is_null());
        core::ptr::write_bytes(p, 46, 32);
        Box::<[u16; 32]>::from_raw(p as _)
    }), Metrics { allocated_bytes: 64, allocations: 1 });

    assert_eq!(delta(|| unsafe {
        let p = alloc::alloc::alloc_zeroed(alloc::alloc::Layout::from_size_align(4, 4).unwrap());
        assert!(!p.is_null());
        Box::<f32>::from_raw(p as _)
    }), Metrics { allocated_bytes: 4, allocations: 1 });

    assert_eq!(delta(|| unsafe {
        let p1 = alloc::alloc::alloc(alloc::alloc::Layout::array::<i32>(7).unwrap());
        assert!(!p1.is_null());
        let p2 = alloc::alloc::realloc(p1, alloc::alloc::Layout::array::<i32>(7).unwrap(), 60);
        assert!(!p2.is_null());
        Box::<[i32; 15]>::from_raw(p2 as _)
    }), Metrics { allocated_bytes: 60, allocations: 1 });

    assert_eq!(delta(|| unsafe {
        let p1 = alloc::alloc::alloc(alloc::alloc::Layout::array::<i32>(15).unwrap());
        assert!(!p1.is_null());
        let p2 = alloc::alloc::realloc(p1, alloc::alloc::Layout::array::<i32>(15).unwrap(), 28);
        assert!(!p2.is_null());
        Box::<[i32; 7]>::from_raw(p2 as _)
    }), Metrics { allocated_bytes: 28, allocations: 1 });

    assert_eq!(thread_metrics(), prev);
}

#[cfg(feature = "global")]
#[test]
fn test_global() {
    // global is really hard to test since unit tests are all one unsynchronized process - at least make sure we're getting alloc data
    let _v = Box::new(123);
    assert_ne!(global_metrics(), Metrics { allocated_bytes: 0, allocations: 0 });
}
