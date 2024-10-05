use crate::error::Result;
use log::info;
use std::process::Command;

pub struct Tmux;

impl Tmux {
    pub fn new() -> Tmux {
        Tmux {}
    }

    pub fn is_running(&self) -> Result<bool> {
        let output = Command::new("pgrep").arg("tmux").output()?;
        Ok(!output.stdout.is_empty())
    }

    pub fn is_inside_tmux(&self) -> bool {
        std::env::var("TMUX").is_ok()
    }

    pub fn session_exists(&self, session_name: &str) -> Result<bool> {
        let output = Command::new("tmux")
            .args(&["has-session", "-t", session_name])
            .output()?;
        Ok(output.status.success())
    }

    pub fn create_and_attach_session(&self, session_name: &str, path: &str) -> Result<()> {
        info!("Creating and attaching tmux session: {}", session_name);

        Command::new("tmux")
            .args(&["new-session", "-s", session_name, "-c", path])
            .status()?;

        Ok(())
    }

    pub fn create_or_switch_session(&self, session_name: &str, path: &str) -> Result<()> {
        if !self.session_exists(session_name)? {
            info!("Creating new tmux session: {}", session_name);
            Command::new("tmux")
                .args(&["new-session", "-ds", session_name, "-c", path])
                .status()?;
        }

        if self.is_inside_tmux() {
            info!("Switching to tmux session: {}", session_name);
            Command::new("tmux")
                .args(&["switch-client", "-t", session_name])
                .status()?;
        } else {
            info!("Attaching to tmux session: {}", session_name);
            Command::new("tmux")
                .args(&["attach-session", "-t", session_name])
                .status()?;
        }
        Ok(())
    }
}
