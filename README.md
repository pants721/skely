# Skely 💀
> A simple command line tool for managing and using skeleton projects

**DISCLAIMER: Skely was created for personal use. Contributions are appreciated, but this is a personal project at heart, so don't take it to seriously.**

## About

Skely is very simple. All it does is copy a given template file or directory from `~.config/sk` to a specified directory.

## Installation 
To install Skely run
```bash
cargo install skely
```

## Configuration

### Settings

As of right now, Skely's configuration is very simple. It is one file at `~/.config/config.toml`. The default configuration looks like this:
```toml
# Skely config

[config]
# Replace with your editor of choice!
editor = ""
```
As you many have guessed, the only option is the editor you would like Skely to use. The string should contain the proper command to call the editor. For example, if I wanted to use Neovim, I **wouldn't** use `editor = "neovim"`, I **would** use `editor = "nvim` so that Skely can properly execute it.

### Configuring New Skeletons

To configure a new skeleton, you can either configure it manually by creating it in the `~/.config/skeletons` directory as a directory or file under the name you would like to identify it by, or you can run the `sk add foo` command to open `~/.config/skeletons/foo.sk` in your preferred text editor. Interactive directory creation is currently under development.

## Usage

### Directory

We have a skeleton structured like this:
```
~/.config/sk/skeletons/
├─ c/
│  ├─ src/
│  │  ├─ main.c
│  ├─ CMakeLists.txt
```
The pattern for creating a new project using a template is
```bash
sk new <ID> <PATH>
```
To create a new project using this template in directory `foo` you would use the command
```bash
sk new c foo
```

### Single File

We have a template for a `CMakeLists.txt`:
```
~/.config/sk/skeletons/
├─ cmake.sk
```
To copy this file to our project as `CMakeLists.txt` you would use the command
```bash
sk new cmake CMakeLists.txt
```
If you were to run
```bash
sk new cmake .
```
It would copy `cmake.sk` to your current project with the name `cmake.sk`

### Remove

To remove a configured skeleton `foo`, you would run
```bash
sk remove foo
```
Pretty simple

### List

Equally as simple, to list configured skeletons, run
```bash
sk list
```
Shocking!

If you've read this far, thank you for taking interest in my software, it is much appreciated :).
