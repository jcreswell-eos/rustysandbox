use tokio::time::{Duration, timeout};
use wrinkledytime::{concurrent_test_sleeper, sync_test_busy_driver, sync_test_sleeper_driver};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let res_async_sleep = match timeout(
        Duration::from_secs(5),
        concurrent_test_sleeper(Duration::from_secs(10)),
    )
    .await
    {
        Ok(_) => String::from("uh oh, async time is disobeying me!"),
        Err(e) => format!("async sleep timed out as expected with {}", e),
    };
    dbg!(res_async_sleep);

    let res_sync_sleep = match timeout(
        Duration::from_secs(5),
        sync_test_sleeper_driver(Duration::from_secs(10)),
    )
    .await
    {
        Ok(_) => String::from("uh oh, sync sleep time is disobeying me!"),
        Err(e) => format!("sync sleep timed out as expected with {}", e),
    };
    dbg!(res_sync_sleep);

    let res_sync_busy = match timeout(
        Duration::from_secs(5),
        sync_test_busy_driver(Duration::from_secs(10)),
    )
    .await
    {
        Ok(_) => String::from("uh oh, sync busy time is disobeying me!"),
        Err(e) => format!("sync busy timed out as expected with {}", e),
    };
    dbg!(res_sync_busy);
}
