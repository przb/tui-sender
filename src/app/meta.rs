use std::path::{Path, PathBuf};

/// A struct that holds metadata bout the json messages
pub struct MsgMetaData<T> {
    pub msg: T,
    pub path: PathBuf,
}

impl<T> MsgMetaData<T> {
    pub fn new<P: AsRef<Path>>(msg: T, path: P) -> Self {
        Self {
            path: path.as_ref().into(),
            msg,
        }
    }
}
