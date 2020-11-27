use std::path::PathBuf;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
    #[error("The file `{}` is malformed. Try removing it.", .0.display())]
    MalformedFile(PathBuf),
    #[error("Could not find the project directory.")]
    ProjectDir,
    #[error("Lacking {0:?} permissions on `{}`", .1.display())]
    Permissions(PermissionType, PathBuf),
}

#[derive(Debug)]
pub enum PermissionType {
    Read,
    Write,
}
