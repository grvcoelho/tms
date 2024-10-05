use crate::utils;
use std::path::PathBuf;

pub fn execute(path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let selected = match path {
        None => utils::select_project_with_fzf()?,
        Some(p) => {
            if PathBuf::from(&p).is_dir() {
                p
            } else if p.contains('/') {
                utils::clone_or_get_repo(&p)?
            } else {
                utils::search_project_with_fzf(&p)?
            }
        }
    };

    if selected.is_empty() {
        return Ok(());
    }

    let session_name = utils::create_session_name(&selected);

    if !utils::is_tmux_running()? {
        utils::create_and_attach_tmux_session(&session_name, &selected)?;
    } else {
        utils::create_or_switch_tmux_session(&session_name, &selected)?;
    }

    Ok(())
}
