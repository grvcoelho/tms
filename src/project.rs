use crate::error::{Error, Result};
use log::info;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct Project {}

impl Project {
    pub fn new() -> Project {
        Project {}
    }

    pub fn get_ghq_root(&self) -> Result<String> {
        let output = Command::new("ghq").arg("root").output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    pub fn select_with_fzf(&self) -> Result<String> {
        info!("Selecting project with fzf");

        let output = Command::new("sh")
            .arg("-c")
            .arg("ghq list -p | fzf")
            .output()?;

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    pub fn search_with_fzf(&self, query: &str) -> Result<String> {
        info!("Searching project with fzf: {}", query);

        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("ghq list -p | fzf -q \"{}\" -1", query))
            .output()?;

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    pub fn get_or_clone_repo(&self, repo_path: &str) -> Result<String> {
        info!("Getting or cloning repo: {}", repo_path);

        let ghq_root = self.get_ghq_root()?;
        let full_path = format!("{}/github.com/{}", ghq_root, repo_path);

        if !Path::new(&full_path).is_dir() {
            info!("Repository not found. Attempting to clone with ghq...");
            let output = Command::new("ghq").arg("get").arg(repo_path).output()?;
            if !output.status.success() {
                return Err(Error::ProjectError("Failed to clone repository".into()));
            }
        }

        Ok(full_path)
    }

    pub fn create_session_name(&self, path: &str) -> String {
        PathBuf::from(path)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .replace(".", "_")
    }
}
