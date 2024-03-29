# Skely 💀
> A simple command line tool for managing and using skeleton projects

## About

Skely is a command line tool for managing and using project templates. Skely is especially useful when working with languages like C that don't have the luxury of a cargo-esque project initializer.

## Installation 
To install Skely run
```bash
cargo install skely
```

## Configuration

### Settings

Skely uses the `SK_PLACEHOLDER` enviroment variable to determine a placeholder string to replace with a project name when using a skeleton. If this variable is unset, it will default to `PLACEHOLDER`.

## Usage

### New
To create a new project using skeleton do `sk new <PROJECT NAME>`. `sk new foo` will create project at `/foo`. Additional options can be found using `sk new --help`.

### Add
Skely can add new skeletons by passing Skely a directory or file to copy. `sk add foo/` will add `foo/` as a skeleton, found at `~/.config/sk/skeletons/foo/`.

### Remove
To remove a skeleton do `sk remove <PROJECT NAME>`.

### List
To list all configured skeletons do `sk list`.

### Completion
To generate shell completion for Skelly do `sk completion <SHELL>`. To see supported shells do `sk completion --help`. Installing completion scripts is dependent on your shell. With zsh and [oh-my-zsh](https://github.com/ohmyzsh/ohmyzsh) it would look something like this:
```zsh
mkdir ~/.oh-my-zsh/completions
sk completion zsh > ~/.oh-my-zsh/completions/_sk
omz reload
```

If you've read this far, thank you for taking interest in my software, it is much appreciated :).
