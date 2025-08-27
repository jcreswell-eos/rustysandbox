use rand::Rng;
use tokio::time::Instant;

pub async fn wait_randomly() {
    let random = {
        let mut rng = rand::rng();
        rng.random_range(1..=5)
    };
    tokio::time::sleep(tokio::time::Duration::from_secs(random)).await;
}

async fn wait_randomly_inside_loop(iteration: usize) {
    let before_sleep = Instant::now();
    wait_randomly().await;
    let after_sleep = Instant::now();
    dbg!(
        "Sleepy time for {} seconds on itr {}.",
        after_sleep
            .checked_duration_since(before_sleep)
            .expect("Failed measuring time elapsed from before to after sleep")
            .as_secs(),
        iteration
    );
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
            wait_randomly_inside_loop(iteration).await;
            dbg!("After sleep for iteration {}", iteration);
        }
    }
}
