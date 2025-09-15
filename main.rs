use notify_rust::Notification;
use std::process::Command;

enum VolumeState {
    Muted,
    Active(u8),
}

impl VolumeState {
    fn from_output(output: &str) -> Self {
        if output.contains("MUTED") {
            return Self::Muted;
        }
        let volume: f32 = output
            .split_whitespace()
            .nth(1)
            .unwrap_or("0")
            .parse()
            .unwrap_or(0.0);
        Self::Active((volume * 100.0) as u8)
    }

    fn notify(&self) {
        let mut notification = Notification::new();
        notification.appname("vol-notify").timeout(1000);

        match self {
            Self::Muted => {
                notification
                    .summary("Volume: Muted")
                    .show()
                    .expect("Notification error:");
            }
            Self::Active(level) => {
                notification
                    .summary(&format!("Volume: {level}%"))
                    .hint(
                        notify_rust::Hint::CustomInt("value".to_owned(), (*level).into()), // CustomInt requires i32...
                    )
                    .show()
                    .expect("Notification error:");
            }
        }
    }
}

fn main() {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .unwrap();
    let text = std::str::from_utf8(&output.stdout).unwrap_or("");
    VolumeState::from_output(text).notify();
}
