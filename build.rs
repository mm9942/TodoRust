use std::process::Command;

fn main() {
    let cli_command = "todo --check";

    #[cfg(target_os = "windows")]
    create_windows_task(cli_command);

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    create_unix_cron_job(cli_command);
}

fn create_windows_task(command: &str) {
    let _ = Command::new("schtasks")
        .args([
            "/CREATE", "/SC", "DAILY", "/TN", "MyDailyTask",
            "/TR", command
        ])
        .output()
        .expect("Failed to create scheduled task on Windows");
}

fn create_unix_cron_job(command: &str) {
    let cron_entry = format!("0 0 * * * {}", command);
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("(crontab -l 2>/dev/null; echo '{}') | crontab -", cron_entry))
        .output()
        .expect("Failed to create cron job on Unix-like OS");
}
