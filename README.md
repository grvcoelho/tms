# tms 

A CLI tool for managing tmux sessions with git projects.

## What is tms?

tms is a command-line interface tool that works as a tmux session manager. It's based on [ThePrimeagen's tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/602019e902634188ab06ea31251c01c1a43d1621/bin/.local/scripts/tmux-sessionizer) but extends the functionality and is tailored to a very specific workflow with [ghq](https://github.com/x-motemen/ghq) for github project management.

Features:
- **Quick Session Management**: Create or switch tmux sessions for git projects
- **Fuzzy Finding**: Select projects quickly with fzf
- **Auto-cloning**: Clone non-existent repositories automatically
- **Consistent Naming**: Generate uniform session names from repo names
- **Smart Session Handling**: Switch to existing sessions or create new ones

## Installation

You can install tms using Homebrew:

```bash
brew tap grvcoelho/tms https://github.com/grvcoelho/tms.git
brew install tms
```

### Requirements

- [ghq](https://github.com/x-motemen/ghq): A Git repository management tool for organizing and accessing your repositories
- [fzf](https://github.com/junegunn/fzf): A command-line fuzzy finder for quickly searching and selecting items from a list
- [tmux](https://github.com/tmux/tmux): A terminal multiplexer

## Usage

```bash
# Open fzf to select a repository from your ghq list
$ tms

# Clone the repo if it doesn't exist, then open a tmux session for it
$ tms username/repo

# Navigate to a specific directory and create/switch to a tmux session
$ tms ~/path/to/directory

# Fuzzy search for a repository (just like z or fzf)
$ tms partial_repo_name
```

## Command-line Options

```
USAGE:
    tms [TERM]

ARGS:
    <TERM>    Optional search term or path to project

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Acknowledgments

- [ThePrimeagen's tmux-sessionizer](https://github.com/ThePrimeagen/.dotfiles/blob/602019e902634188ab06ea31251c01c1a43d1621/bin/.local/scripts/tmux-sessionizer): This is the original bash script this tool is based on.
- [tmux-sessionizer](https://github.com/jrmoulton/tmux-sessionizer): This is a project that implements some of the features with a different approach. Check it out to see the differences.
