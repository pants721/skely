# Refernces
- [Clap cookbook git derive](https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html)
- [Clap cookbook cargo derive](https://docs.rs/clap/latest/clap/_derive/_cookbook/cargo_example_derive/index.html)
- [Copy directory recursivley function (common.rs)](https://nick.groenen.me/notes/recursively-copy-files-in-rust/)
- [Capitalize word function (common.rs)](https://nick.groenen.me/notes/capitalize-a-string-in-rust/)
- [create_dir_all documentation](https://doc.rust-lang.org/std/fs/fn.create_dir_all.html)
- [crates.io Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [toml formatting](https://toml.io/en/)
- [Rust file struct](https://doc.rust-lang.org/std/fs/struct.File.html)
- [Toml crate docs](https://docs.rs/toml/latest/toml/#)

# TODO
- [ ] Add default skeleton text
- [ ] Individual project cfg files
- [x] Template word config
- [ ] Bash completion
- [x] Add correct error handling (get rid of the 100 std::io::Error's)
- [x] Add --touch tag for add
- [x] Choose unwrap_or_else() or Result<>
- [ ] Interactive dir created for `sk add --dir`
- [x] Proper bin instillation of release
- [x] Non vim text editors
- [x] Settings
- [x] Toml serialization for default config file

# Command Line Interface

## Commands

- List                       - List all configured skeletons
- Edit <Skeleton>            - Edit a skeleton
- Add <Name>                 - Configure new skeleton
- Add <Name> --source <Path> - Configure new skeleton from path
- New <Path>                 - Copy skeleton to specified directory
- Remove <Skeleton>          - Remove configured skeleton and its files

### Usage Examples

sk list
sk edit rust (opens vim with the rust sk file/dir)
sk add rust (todo! maybe interactive dir creator)
sk add --source rust_sk/
sk new rust
sk remove javascript
















# TOML Config Format

```toml
# Skely config

[config]
# Default editor is vim
editor = "vim"
```



