use loopy_async::wait_randomly;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let before_wait = Instant::now();
    wait_randomly().await;
    let after_wait = Instant::now();
    println!(
        "Hello, world! We waited {}",
        after_wait
            .checked_duration_since(before_wait)
            .expect("Failed measuring wait time")
            .as_secs()
    );
    Ok(())
}
