# Refernces
- [Clap cookbook git derive](https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html)
- [Clap cookbook cargo derive](https://docs.rs/clap/latest/clap/_derive/_cookbook/cargo_example_derive/index.html)
- [Copy directory recursivley function (common.rs)](https://nick.groenen.me/notes/recursively-copy-files-in-rust/)
- [Capitalize word function (common.rs)](https://nick.groenen.me/notes/capitalize-a-string-in-rust/)
- []


































!TODO
[ ] Add default skeleton text
[x] Add correct error handling (get rid of the 100 std::io::Error's)
[ ] Add --touch tag for add
[x] Choose unwrap_or_else() or Result<>
[ ] Interactive dir created for `sk add --dir`

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
