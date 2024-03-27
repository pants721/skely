# How to contribute

Thanks for wanting to help with Skely!

There aren't many rules as of now, but here's what's important.

Skely uses [semantic versioning](https://semver.org/).

## Issues

Filing issues is contribution! Issue titles should have brief, but descriptive 
summaries of the problem.

An example of a bad issue title (too long):
`My computer won't work when I'm using skely to generate my C++ cmake project`

An example of a bad issue title (too short):
`Skely C++ issue`

An example of a good issue title (just right):
`Can't generate C++ project with CMake`

In your issue description, briefly describe the problem and include steps to 
reproduce if necessary.

Use the appropriate Github tag for your issue.

## PR's

Pull requests are always welcome as long as they follow contribution guidelines.

Make sure your code abides by the [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/),
which is easily achievable using `cargo fmt`.

Make sure to at least check your code with [clippy](https://github.com/rust-lang/rust-clippy).
99% of clippy's suggestions should probably be followed, but sometimes I disagree with it too.

Make sure your commit messages are resonably descriptive and broken up into specific
things (not just one mega commit). Inversely, if your pr makes the working tree 
look more like a working garden, then a maintainer might just squash your commits. 
The main purpose of breaking up your commits is so that maintainers and other 
contributors can understand the changes better.

Use one of the following prefixes for your pr title:
- `feat:` features
- `fix:` for bug fixes (try to file and issue and mention that issue in the pr)
- `chore:` boring stuff that doesn't actually fix or introduce anything (version bumps, etc.)
- `docs:` documentation updates
- other tags can be made up, just try to use these first

Just like issues, make your titles concise.

Bad:
`feat: this adds a feature that implements logging for dogs`

Good:
`feat: logging for dogs`
