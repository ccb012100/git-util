# git-util

CLI utility for working with the Git CLI. I use it as a wrapper around the `git` CLI by copying the executable to `$HOME/bin/git-util` and then
aliasing `git` to a `fish` shell function in `$HOME/.config/fish/functions/git.fish`:

```fish
function git --wraps=/home/linuxbrew/.linuxbrew/bin/git --description 'wraps the Git CLI with $HOME/bin/git-util application'
    $HOME/bin/git-util $argv
end
```

This project was precipitated by wanting more powerful [Git aliases](https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases) and becoming frustrated
with the limitations and ergonomics of using [Bash functions](https://www.atlassian.com/blog/git/advanced-git-aliases) in `.gitconfig`.

## Usage

Run `git-util` or `git-util help` to get the full list of subcommands.

Run `git-util COMMAND --help` to see details for a specific subcommand.

```plaintext
Usage: git-util [OPTIONS] [FALLBACK]... [COMMAND]

Commands:
  a        Wrapper around `git-add`
  aa       Add updated and untracked files
  aac      Add updated and untracked files and then commit
  aamend   Stage updated and untracked files and amend the previous commit
  alias    List configured aliases
  auc      Commit updated files
  aumend   Stage updated files and amend the previous commit
  author   Reset author to current value of `user.author` and `user.email` for the last n commits
  conf     List config settings (excluding aliases)
  hook     Call a git hook
  files    List the files that changed in the last n commits
  l        Wrapper around `git-log`, formatted to 1 line per commit
  last     List commit message and of changed files for the last n commits; wrapper around `git-log --compact-summary`
  restore  Wrapper around `git-restore`
  show     Wrapper around `git-show`
  undo     Reset the last n commits and keep the undone changes in working directory
  unstage  Move staged files back to staging area; wrapper around `git-restore --staged`
  update   Update the specified local branch from origin without checking it out
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [FALLBACK]...  A catch-all for passing straight through to the native `git` binary; required if [COMMAND] is not specified

Options:
  -v, --verbose...     Set verbosity; adding multiple times increases the verbosity level (>=4, i.e. `-vvvv`, sets maximum verbosity)
  -p, --print-command  Print the `std::process::Command`s that are executed
  -d, --dry-run        Print the `std::process::Command`s that will be executed, but do not run
  -h, --help           Print help
  -V, --version        Print version
```

Any subcommand passed to `git-util` that does not match the above list of subcommands will be passed through to the `git` CLI, e.g. `git-util foo`
will evaluate to `git foo`. This allows me to alias it to `git` and have the subcommands act as git aliases.
