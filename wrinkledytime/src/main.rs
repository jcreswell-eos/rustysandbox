use tokio::time::{Duration, timeout};
use wrinkledytime::concurrent_test_sleeper;

#[tokio::main(flavor = "current_thread")]
async fn main() {
   let res = match timeout(Duration::from_secs(5), concurrent_test_sleeper(Duration::from_secs(10))).await {
       Ok(_) => String::from("uh oh, time is disobeying me!"),
       Err(e) => format!("timed out as expected with {}",e),
   };
   dbg!(res);
}
