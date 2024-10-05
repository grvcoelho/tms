use crate::{
    error::{Error, Result},
    project::Project,
    tmux::Tmux,
};
use std::{path::PathBuf, process::Command};

pub struct Program;

impl Program {
    pub fn new() -> Program {
        Program {}
    }

    fn command_exists(&self, cmd: &str) -> bool {
        Command::new("which").arg(cmd).output().is_ok()
    }

    pub fn ensure_required_dependencies(&self, dependencies: &[&str]) -> Result<()> {
        for cmd in dependencies.iter() {
            if !self.command_exists(cmd) {
                return Err(Error::MissingDependency(cmd.to_string()));
            }
        }

        Ok(())
    }

    pub fn execute(&self, term: Option<String>) -> Result<()> {
        let project = Project::new();
        let tmux = Tmux::new();

        let selected = match term {
            None => project.select_with_fzf()?,
            Some(path) => {
                if PathBuf::from(&path).is_dir() {
                    path
                } else if path.contains("/") {
                    project.get_or_clone_repo(&path)?
                } else {
                    project.search_with_fzf(&path)?
                }
            }
        };

        if selected.is_empty() {
            return Ok(());
        }

        let session_name = project.create_session_name(&selected);

        if !tmux.is_running()? {
            tmux.create_and_attach_session(&session_name, &selected)?;
        } else {
            tmux.create_or_switch_session(&session_name, &selected)?;
        }

        Ok(())
    }
}
