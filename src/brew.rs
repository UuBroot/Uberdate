use std::process::Command;

pub(crate) fn check_brew_installed() -> bool {
    let result = Command::new("brew")
        .arg("--version")
        .output();

    match result {
        Ok(output) => output.status.success(),
        Err(_) => false, // Command failed to execute, so brew is not installed
    }
}
