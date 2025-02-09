use chrono::{DateTime, Local, TimeDelta, TimeZone, Timelike};
use std::future::Future;
use std::time::Duration;
use tokio::time;

pub async fn run<F, Fut>(func: F, hour: u32, minute: u32, cool_down: TimeDelta)
where
    F: Fn() -> Fut,
    Fut: Future<Output=()>,
{
    let mut interval = time::interval(Duration::from_secs(1));
    let mut last_exec_time: DateTime<Local> = Local.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

    loop {
        interval.tick().await;
        let now = Local::now();

        if now.hour() == hour && now.minute() == minute && now.signed_duration_since(last_exec_time) > cool_down {
            last_exec_time = now;
            func().await;
        }
    }
}