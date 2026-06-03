use ksni::TrayMethods;
use log::{error, info};
use tokio::time::{Duration, sleep};

use lenocon_daemon::config::Config;
use lenocon_daemon::logging::configure_logger;
use lenocon_daemon::tray::LenoconTray;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    configure_logger().unwrap();
    let config = Config::load().unwrap_or_else(|err| {
        error!("Config error: {err}");
        info!("Using default settings");
        Config::default()
    });
    let tray = LenoconTray::from(config);
    let handle = match tray.spawn().await {
        Ok(h) => h,
        Err(e) => {
            error!("Error while spawning handle: {e}");
            std::process::exit(1);
        }
    };

    let mut previous_status = lenocon_core::read_status()
        .inspect_err(|e| error!("Failed to read initial status: {e}"))
        .unwrap_or(false);
    loop {
        sleep(Duration::from_secs(1)).await;
        if let Ok(is_enabled) = lenocon_core::read_status()
            && is_enabled != previous_status
        {
            handle
                .update(|tray: &mut LenoconTray| tray.update_status())
                .await;
            info!(
                "Conservation mode {}",
                if is_enabled { "ON" } else { "OFF" }
            );
            previous_status = is_enabled;
        }
    }
}
