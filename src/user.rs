use libunftp::auth::UserDetail;
use unftp_sbe_rooter::UserWithRoot;
use std::fmt::Formatter;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct CCUser {
    username: String,
    root: Option<PathBuf>
}

impl CCUser {
    pub fn new(username: String, root: Option<PathBuf>) -> Self {
        CCUser {
            username,
            root
        }
    }
}

impl UserDetail for CCUser {
    fn account_enabled(&self) -> bool {
        true
    }
    fn home(&self) -> Option<&std::path::Path> {
        self.root.as_deref()
    }
}

impl std::fmt::Display for CCUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}@{:?}", self.username,self.root)
    }
}

impl UserWithRoot for CCUser {
    fn user_root(&self) -> Option<PathBuf> {
        self.root.clone()
    }
}