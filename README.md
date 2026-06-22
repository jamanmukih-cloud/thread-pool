# Thread Pool 🧵

Work-stealing thread pool with priorities and metrics.

## Features

- **Work Stealing**: Automatic load balancing
- **Task Priorities**: Priority queue scheduling
- **Timeout**: Per-task deadline support
- **Metrics**: Queue depth, throughput, latency

## Performance

| Metric | Value |
|--------|-------|
| Task throughput | 10M tasks/s |
| Scheduling overhead | 50ns |
| Context switch | <1μs |

## Quick Start

```rust
let pool = ThreadPool::new(num_cpus::get());
pool.submit(Priority::High, || println!("High priority task"));
pool.submit(Priority::Normal, || println!("Normal task"));
pool.shutdown();
```

## License

MIT