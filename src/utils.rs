use std::env;
use std::path::Path;
use std::process::Command;

pub fn command_exists(cmd: &str) -> bool {
    Command::new("which").arg(cmd).output().is_ok()
}

pub fn check_required_commands() -> Result<(), Box<dyn std::error::Error>> {
    let required_commands = ["ghq", "fzf", "tmux"];

    for cmd in required_commands.iter() {
        if !command_exists(cmd) {
            return Err(format!("{} was not found. Please install it first.", cmd).into());
        }
    }

    Ok(())
}

pub fn select_project_with_fzf() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ghq list -p | fzf")
        .output()?;

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn search_project_with_fzf(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("ghq list -p | fzf -q \"{}\" -1", query))
        .output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn clone_or_get_repo(repo_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ghq_root = get_ghq_root()?;
    let full_path = format!("{}/github.com/{}", ghq_root, repo_path);

    if !Path::new(&full_path).is_dir() {
        println!("Repository not found. Attempting to clone with ghq...");
        let output = Command::new("ghq").arg("get").arg(repo_path).output()?;
        if !output.status.success() {
            return Err("Failed to clone repository".into());
        }
    }

    Ok(full_path)
}

fn get_ghq_root() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ghq").arg("root").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn create_session_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .replace(".", "_")
}

pub fn is_tmux_running() -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("pgrep").arg("tmux").output()?;
    Ok(!output.stdout.is_empty())
}

pub fn create_and_attach_tmux_session(
    session_name: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("tmux")
        .args(&["new-session", "-s", session_name, "-c", path])
        .status()?;
    Ok(())
}

pub fn create_or_switch_tmux_session(
    session_name: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !tmux_session_exists(session_name)? {
        Command::new("tmux")
            .args(&["new-session", "-ds", session_name, "-c", path])
            .status()?;
    }

    if is_inside_tmux() {
        Command::new("tmux")
            .args(&["switch-client", "-t", session_name])
            .status()?;
    } else {
        Command::new("tmux")
            .args(&["attach-session", "-t", session_name])
            .status()?;
    }
    Ok(())
}

fn tmux_session_exists(session_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("tmux")
        .args(&["has-session", "-t", session_name])
        .output()?;
    Ok(output.status.success())
}

fn is_inside_tmux() -> bool {
    env::var("TMUX").is_ok()
}

