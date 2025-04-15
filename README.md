# Scheduler

Run async function at a planned schedule

## Usage

For example I want to run this **add** function

```rust
async fn add(num: i32) -> String {
  format!("result, {}!", num + 1)
}
```

### Create and configure a scheduler

#### Run now and only once
```rust
let mut scheduler = Scheduler::default();
scheduler.plan.date_time = Some(Local::now());
scheduler.plan.count = Some(1);

```
#### Run every 3 seconds for up to 3 times

```rust
let mut scheduler = Scheduler::default();
scheduler.plan.interval = Some(Duration::from_secs(3));
scheduler.plan.count = Some(3);
```

### Run

```rust
// Get result vec
let result = scheduler.run(add, 1).await;
println!("{:?}", result)
```

The result should be a Vec containing results from every run.

