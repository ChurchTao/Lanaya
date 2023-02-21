use chrono::Duration;
use std::thread::sleep;

pub async fn set_time_out(f: impl Fn(), d: Duration) {
    sleep(d.to_std().unwrap());
    f();
}
