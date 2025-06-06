use tokio::time::{sleep, Duration, timeout};

pub async fn concurrent_test_sleeper(duration: Duration) -> std::result::Result<(), &'static str> {
    sleep(duration).await;
    Ok(())
}