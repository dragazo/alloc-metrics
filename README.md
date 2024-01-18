## Description

`alloc-metrics` is a simple crate that adds a new global allocator type that tracks the total number of allocations and actual bytes allocated.
In typical programs, this could be useful to get a plot of memory utilization over time for debugging purposes,
or could be used in interpreter runtimes to limit the amount of memory a script is allowed to use.

## Setup

To get started using `alloc-metrics`, you must first set the global allocator for your project:

```rust
use alloc_metrics::MetricAlloc;

#[global_allocator]
static GLOBAL: MetricAlloc<std::alloc::System> = MetricAlloc::new(std::alloc::System);
```

Note that the [`MetricAlloc`] type is able to wrap any existing global allocator type.
Thus, you can compose the features of `alloc-metrics` with any other global allocator you want.

## Features

- `thread`: gives access to thread-local allocation metrics (requires `std`).
- `global`: gives access to global allocation metrics.

## `no-std`

This crate is fully compatible in `no-std` environments by disabling default features:

```toml
alloc-metrics = { version = "...", default-features = false, features = ["global"] }
```

Note that we re-enabled the `global` feature so that we still have access to global allocation metrics (see feature list above).
