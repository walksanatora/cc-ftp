use std::path::PathBuf;

use libunftp::auth::{AuthenticationError, Authenticator, Credentials};
use regex::Regex;
use async_trait::async_trait;

use crate::user::CCUser;

#[derive(Debug)]
pub struct Auth {
    re: Regex,
    root: PathBuf
}

impl Auth {
    pub fn new(root: PathBuf) -> Auth {
        Auth { 
            re: Regex::new(r"([a-z]+)([0-9]+)").unwrap(),
            root
        }
    }
}


#[async_trait]
impl Authenticator<CCUser> for Auth {
    #[doc = " Authenticate the given user with the given credentials."]
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    async fn authenticate(&self,username: &str,creds: &Credentials) -> Result<CCUser,AuthenticationError> {
        let captures = self.re.captures(username);
        if captures.is_none() {
            println!("Username Regex failed. got username '{}'",username);
            return Err(AuthenticationError::BadUser)
        };
        let mut cloned = self.root.clone();
        let captured = captures.unwrap();
        let device = captured.get(1).unwrap().as_str();
        let id = captured.get(2).unwrap().as_str();
        cloned.push(device);
        cloned.push(id);
        println!("checking for user home {:?}", cloned);
        if !cloned.exists() {
            println!("User Rejected (computer does not exists)");
            return Err(AuthenticationError::BadUser)
        }

        let mut password = cloned.clone();
        password.push(".ftppasswd");
        if let Ok(meta) = password.metadata() {
            if !meta.is_file() {
                println!("User Rejected (password file is not password)");
                return Err(AuthenticationError::BadPassword)
            }
        } else {
            println!("User Rejected (passowrd file does not exists)");
            return Err(AuthenticationError::BadPassword)
        }
        let lock = std::fs::read_to_string(password).unwrap();
        if lock.as_str().trim() != creds.password.as_ref().unwrap().trim() {
            println!("User Rejected (invalid password)");
            return Err(AuthenticationError::BadPassword)
        }
        println!("User Accepted to {:?}",cloned);
        return Ok(CCUser::new(username.to_string(),Some(cloned)))
    }
}