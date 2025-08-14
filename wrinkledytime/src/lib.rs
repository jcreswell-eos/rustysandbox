use tokio::time::{Duration, sleep};

pub async fn concurrent_test_sleeper(duration: Duration) -> std::result::Result<(), &'static str> {
    sleep(duration).await;
    Ok(())
}

pub async fn sync_test_sleeper_driver(duration: Duration) -> std::result::Result<(), &'static str> {
    sync_test_sleeper(duration)
}

pub async fn sync_test_busy_driver(duration: Duration) -> std::result::Result<(), &'static str> {
    sync_test_busy(duration)
}

pub fn sync_test_sleeper(duration: Duration) -> std::result::Result<(), &'static str> {
    std::thread::sleep(duration);
    Ok(())
}

pub fn sync_test_busy(duration: Duration) -> std::result::Result<(), &'static str> {
    let clock = std::time::SystemTime::now();
    loop {
        if clock.elapsed().unwrap() >= duration {
            break;
        }
    }
    dbg!("busy for {:?} ms", clock.elapsed().unwrap());
    Ok(())
}
