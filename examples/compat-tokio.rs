use std::time::{Instant, Duration};

use tokio::time::delay_for;

fn main() {
    smol::run(async {
        let start = Instant::now();
        println!("Sleeping...");
        delay_for(Duration::from_secs(1)).await;
        println!("Woke up after {:?}", start.elapsed());
    })
}
