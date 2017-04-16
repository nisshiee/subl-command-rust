use subl::*;

use std::ffi::OsStr;
use std::process::Command;

const SUBL_PATH: &'static str = include_str!("../../config/subl_path");

pub fn run<S: AsRef<OsStr>>(target: S) -> Result<()> {
    Command::new(SUBL_PATH)
        .arg(target)
        .spawn()
        .and_then(|ref mut child| child.wait())?;
    Ok(())
}
