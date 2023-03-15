use anyhow::{Context, Result};
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use home::home_dir;
use predicates::prelude::*; // Used for writing assertions
use std::{
    fs::{read_dir, read_to_string, remove_file, File},
    path::PathBuf,
    process::Command,
}; // Run programs

#[test]
fn file_doesnt_exist() -> Result<()> {
    // sk edit ajfklsdjfkadsf
    let mut cmd = Command::cargo_bin("sk")?;

    cmd.arg("edit")
        .arg("dlsdjlasdasdlasdlasldlasasdlasdlldlasldasd");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Skeleton not found"));

    Ok(())
}

#[test]
/// Creates empty skeleton at TEST_FILE.sk and removes it
fn add_touch() -> Result<()> {
    let mut add_cmd = Command::cargo_bin("sk")?;
    add_cmd.arg("add").arg("TEST_FILE").arg("-t");
    add_cmd.assert().success();

    let sk_file: PathBuf = PathBuf::from(format!(
        "{}/.config/sk/skeletons/TEST_FILE.sk",
        home_dir().unwrap().display()
    ));

    assert!(sk_file.exists());
    assert!(sk_file.as_os_str().is_empty());

    // let mut rm_cmd = Command::cargo_bin("sk")?;
    // rm_cmd.arg("remove").arg("TEST_FILE").arg("-n");
    // rm_cmd.assert()
    //     .success();

    Ok(())
}

#[test]
fn add_from_source() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let sk_file: PathBuf = PathBuf::from(format!(
        "{}/.config/sk/skeletons/TEST_FILE.sk",
        home_dir().unwrap().display()
    ));

    if sk_file.exists() {
        remove_file(&sk_file)?;
    }

    let mut add_cmd = Command::cargo_bin("sk")?;
    add_cmd
        .arg("add")
        .arg("-s")
        .arg(file.path())
        .arg("TEST_FILE");
    add_cmd.assert().success();

    assert!(file_eq(file.to_path_buf(), sk_file.clone())?);

    remove_file(&sk_file)?;

    Ok(())
}

//
// UTILS
//

fn file_eq(file1: PathBuf, file2: PathBuf) -> Result<bool> {
    let file1_string: String = read_to_string(file1)?;
    let file2_string: String = read_to_string(file2)?;

    Ok(file1_string == file2_string)
}
