use libunftp::auth::UserDetail;
use unftp_sbe_rooter::UserWithRoot;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct CCUser {
    pub username: String,
    pub root: Option<PathBuf>,
}

impl UserDetail for CCUser {
    fn account_enabled(&self) -> bool {
        true
    }
}

impl std::fmt::Display for CCUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(username: {:?}", self.username,)
    }
}

impl UserWithRoot for CCUser {
    fn user_root(&self) -> Option<PathBuf> {
        self.root.clone()
    }
}