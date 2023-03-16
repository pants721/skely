use anyhow::{Result, Context};
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use home::home_dir;
use predicates::prelude::*; // Used for writing assertions
use std::{
    fs::{read_to_string, remove_file, read_dir},
    path::{PathBuf, Path},
    process::Command, collections::HashSet,
}; // Run programs

// - List                       - List all configured skeletons
// - Edit <Skeleton>            - Edit a skeleton
// - Add <Name>                 - Configure new skeleton
// - Add <Name> --source <Path> - Configure new skeleton from path
// - New <Path>                 - Copy skeleton to specified directory
// - Remove <Skeleton>          - Remove configured skeleton and its files

mod list_tests {
    use super::*;

    #[test]
    fn list_basic() -> Result<()> {
        let mut list_cmd = Command::cargo_bin("sk")?;
        list_cmd.arg("list");
        list_cmd.assert().success();

        Ok(())
    }

    #[test]
    fn list_verbose() -> Result<()> {
        let mut list_cmd = Command::cargo_bin("sk")?;
        list_cmd.arg("list").arg("-v");
        list_cmd.assert().success();

        Ok(())
    }
}

mod edit_tests {
    use super::*;

    #[test]
    fn edit_file_doesnt_exist() -> Result<()> {
        let skeleton_dir: PathBuf = PathBuf::from(format!(
            "{}/.config/sk/skeletons",
            home_dir().unwrap().display()
        ));

        let filename = find_shortest_nonfilenames(&skeleton_dir)?;

        let mut cmd = Command::cargo_bin("sk")?;

        cmd.arg("edit")
            .arg(filename);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Error: Skeleton not found"));

        Ok(())
    }
}

mod add_tests {
    use super::*;
    use anyhow::Context;

    #[test]
    /// Creates empty skeleton at TEST_FILE.sk and removes it
    fn add_touch() -> Result<()> {
        let sk_file: PathBuf = PathBuf::from(format!(
            "{}/.config/sk/skeletons/TEST_FILE.sk",
            home_dir().unwrap().display()
        ));

        if sk_file.exists() {
            remove_file(&sk_file)?;
        }

        let mut add_cmd = Command::cargo_bin("sk")?;
        add_cmd.arg("add").arg("TEST_FILE").arg("-t");
        add_cmd.assert().success();

        assert!(sk_file.exists());

        remove_file(&sk_file)?;

        Ok(())
    }
}

mod new_tests {
    use super::*;

    // for these there has to be some complicated skeleton created uniquely for testing in code so ill implement that later
}

mod remove_tests {
    use super::*;
}

//
// UTILS
//

fn file_eq(file1: PathBuf, file2: PathBuf) -> Result<bool> {
    assert!(file1.exists());
    assert!(file2.exists());
    let file1_string: String = read_to_string(file1).context("file1 error")?;
    let file2_string: String = read_to_string(file2).context("file2 error")?;

    Ok(file1_string == file2_string)
}

// I'm proud of this algorithm
fn find_shortest_nonfilenames(dir: &Path) -> Result<String> {
    let mut filenames = HashSet::new();

    // Iterate through all entries in the directory
    for entry in read_dir(dir)? {
        let entry = entry?;
        // If it's a file, add the file name to the set of filenames
        let filename = entry.file_name().to_string_lossy().into_owned();
        filenames.insert(filename);
    }

    // Iterate through all possible strings of increasing length until
    // we find a string that is not a filename
    for len in 1.. {
        for name in generate_strings(len) {
            if !filenames.contains(&name) {
                return Ok(name);
            }
        }
    }

    // We should never get here
    unreachable!()
}

fn generate_strings(length: usize) -> Vec<String> {
    let mut names = Vec::new();
    let chars = (b'!'..=b'~').map(char::from).collect::<Vec<_>>();
    generate_strings_rec(&chars, length, &mut names, String::new());
    names
}

fn generate_strings_rec(
    chars: &[char],
    length: usize,
    names: &mut Vec<String>,
    current_name: String,
) {
    if current_name.len() == length {
        names.push(current_name);
        return;
    }

    for c in chars {
        let mut new_prefix = current_name.clone();
        new_prefix.push(*c);
        generate_strings_rec(chars, length, names, new_prefix);
    }
}









