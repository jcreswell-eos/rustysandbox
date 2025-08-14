use tokio::*;
use rand::Rng;

async fn wait_randomly(iteration: usize) {
    let random = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=5)
    };
    dbg!("Sleepy time for {} seconds on itr {}.", random, iteration);
    tokio::time::sleep(tokio::time::Duration::from_secs(random)).await;
}

#[cfg(test)]
mod loopy_async_test {
    use super::*;

    /**
     * Q: Do loops operate synchronously within async functions and with nested awaits?
     * A: seems so, thank the stars.
     */
    #[tokio::test]
    async fn test_async_loop() {
        for iteration in 1..=3 {
            dbg!("About to sleep for iteration {}", iteration);
            wait_randomly(iteration).await;
            dbg!("After sleep for iteration {}", iteration);
        }
    }
}