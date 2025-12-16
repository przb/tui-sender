use std::path::PathBuf;

pub mod client;
pub mod msgs;
pub mod people_info;
pub mod server;

/// A struct that holds metadata bout the json messages
struct MsgMetaData<T> {
    msg: T,

    path: PathBuf,
}

impl<T> MsgMetaData<T> {
    pub fn new(msg: T, path: PathBuf) -> Self {
        Self { path, msg }
    }
}
