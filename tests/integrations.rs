use assert_cmd::Command;
use color_eyre::eyre::Result;

#[test]
/// Make sure help runs. This indicates the binary works
fn test_help() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  let assert = cmd.arg("--help").assert();
  assert.success().stderr("");
  Ok(())
}

#[test]
/// Make sure help runs. This indicates the binary works
fn test_write_help() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  let assert = cmd.arg("write").arg("--help").assert();
  assert.success().stderr("");
  Ok(())
}

#[test]
/// Make sure help runs. This indicates the binary works
fn test_write() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  let assert = cmd.arg("write").assert();
  assert.success().stderr("");
  Ok(())
}
