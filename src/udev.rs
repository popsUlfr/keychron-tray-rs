use std::error;

use tokio::process::Command;

const UDEV_RULE_CONTENT_BYTES: &str = include_str!("../assets/70-keychron.rules");
const UDEV_RULE_DIR: &str = "/etc/udev/rules.d";
const UDEV_RULE_FILENAME: &str = "70-keychron.rules";

pub async fn udev_rule_install() -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let pkexec_status = Command::new("pkexec").args(["sh", "-c", format!("mkdir -p \"{}\" && cat > \"{}/{}\" <<'EOF' && udevadm control --reload && udevadm trigger\n{}\nEOF\n", UDEV_RULE_DIR, UDEV_RULE_DIR, UDEV_RULE_FILENAME, UDEV_RULE_CONTENT_BYTES).as_str()]).status().await?;
    if !pkexec_status.success() {
        return Err("pkexec command failed".into());
    }

    Ok(())
}
