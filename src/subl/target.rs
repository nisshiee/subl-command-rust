use subl::*;

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn target() -> Result<OsString> {
    let base_dir = base_dir()?;
    let project = find_project(base_dir)?;
    Ok(project.into_os_string())
}

fn base_dir() -> Result<PathBuf> {
    let mut args = env::args_os();

    if let Some(arg) = args.nth(1) {
        base_dir_from(&arg)
    } else {
        base_dir_from(".")
    }
}

fn base_dir_from<S: AsRef<OsStr>>(arg: S) -> Result<PathBuf> {
    let mut pb = canonicalize(arg.as_ref())?;
    let metadata = pb.metadata()?;

    if metadata.is_dir() {
        Ok(pb)
    } else {
        pb.pop();
        Ok(pb)
    }
}

fn canonicalize(arg: &OsStr) -> Result<PathBuf> {
    Path::new(&arg)
        .canonicalize()
        .map_err(|e| {
            match e.kind() {
                io::ErrorKind::NotFound => Error::InvalidArgument(arg.to_os_string()),
                _                       => Error::UnexpectedIoError(e),
            }
        })
}

fn find_project(pb: PathBuf) -> Result<PathBuf> {
    let dir = fs::read_dir(&pb)?;
    let mut dir = dir.filter_map(|entry| {
        entry.map(|entry| entry.path()).ok()
    });

    if let Some(project) = dir.find(|ref p| is_project(p)) {
        Ok(project)
    } else {
        Ok(pb)
    }
}

fn is_project(pb: &PathBuf) -> bool {
    pb.to_string_lossy().ends_with(".sublime-project")
}
