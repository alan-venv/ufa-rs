use std::thread;
use std::time::Duration;

pub fn sleep(seconds: u64) {
    let duration = Duration::from_secs(seconds);
    thread::sleep(duration)
}
