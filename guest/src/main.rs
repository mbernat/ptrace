use std::time::Duration;

use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    let pid = std::process::id();
    info!("Started with pid {pid}");
    loop {
        std::thread::sleep(Duration::from_secs(1));
        info!("Yo");
    }
}
