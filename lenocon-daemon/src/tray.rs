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

struct StandardItemBuilder<T> {
    label: String,
    enabled: bool,
    icon_name: String,
    activate: Box<dyn Fn(&mut T) + Send>,
}

impl<T> StandardItemBuilder<T> {
    fn new() -> Self {
        StandardItemBuilder::<T> {
            label: String::new(),
            enabled: false,
            icon_name: String::new(),
            activate: Box::new(|_this| {}),
        }
    }

    fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    fn icon_name(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = icon_name.into();
        self
    }

    fn activate(mut self, activate: Box<dyn Fn(&mut T) + Send>) -> Self {
        self.activate = activate;
        self
    }

    fn build(self) -> StandardItem<T> {
        StandardItem {
            label: self.label,
            enabled: self.enabled,
            icon_name: self.icon_name,
            activate: self.activate,
            ..Default::default()
        }
    }
}

fn toggle_status(tray: &mut LenoconTray) {
    let result = std::process::Command::new("pkexec")
        .args(["/usr/bin/lenocon", "toggle"])
        .output();

    match result {
        Ok(out) if out.status.success() => match lenocon_core::read_status() {
            Ok(new) => tray.enabled = new,
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
            StandardItemBuilder::new()
                .label(format!("Conservation: {}", self.status_str()))
                .enabled(false)
                .icon_name(self.icon_name())
                .build()
                .into(),
            MenuItem::Separator,
            StandardItemBuilder::new()
                .label("Toggle")
                .activate(Box::new(toggle_status))
                .build()
                .into(),
            StandardItemBuilder::new()
                .label("Quit")
                .activate(Box::new(|_| std::process::exit(0)))
                .build()
                .into(),
        ]
    }
}
