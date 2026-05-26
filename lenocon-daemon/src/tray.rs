use ksni::menu::{MenuItem, StandardItem};
use log::error;

pub struct LenoconTray {
    enabled: bool,
}

impl LenoconTray {
    pub fn new(status: bool) -> Self {
        LenoconTray { enabled: status }
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_enabled(&mut self, status: bool) {
        self.enabled = status;
    }
    fn status_str(&self) -> &'static str {
        if self.enabled { "ON" } else { "OFF" }
    }
}

impl ksni::Tray for LenoconTray {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn icon_name(&self) -> String {
        match self.enabled {
            true => "battery-good-charging-symbolic",
            false => "battery-caution-symbolic",
        }
        .into()
    }

    fn title(&self) -> String {
        format!("Conservation mode: {}", self.status_str())
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![
            StandardItem {
                label: format!("Conservation: {}", self.status_str()),
                enabled: false,
                icon_name: self.icon_name(),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Toggle".into(),
                activate: Box::new(|this: &mut Self| {
                    let result = std::process::Command::new("pkexec")
                        .args(["/usr/bin/lenocon", "toggle"])
                        .output();

                    match result {
                        Ok(out) if out.status.success() => match lenocon_core::read_status() {
                            Ok(new) => this.enabled = new,
                            Err(e) => error!(
                                "lenocon: file {}: {e}",
                                lenocon_core::CONSERVATION_FILE_PATH
                            ),
                        },
                        Ok(out) => error!(
                            "pkexec lenocon failed ({}): {}",
                            out.status,
                            String::from_utf8_lossy(&out.stderr).trim_end_matches('\n')
                        ),
                        Err(e) => error!("Failed to spawn pkexec: {e}"),
                    }
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
