use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tarsum")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open input file"));

    Ok(())
}

#[test]
fn check_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tarsum")?;
    let output = r#"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  tarsum-test/empty
ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  tarsum-test/abc
\e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  tarsum-test/new\nline
"#;

    cmd.args(["-c", "sha256", "tests/tarsum-test.tar"]);
    cmd.assert()
        .success()
        .stdout(output);

    Ok(())
}
