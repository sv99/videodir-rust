//! Working with Apache .htpasswd files using bcrypt.
//!
//! # Examples
//!
//! Add and verify BCrypt hash
//!
//! ```rust
//! use htpasswd::Htpasswd;
//!
//! fn setup_credentials() -> Result<(), Box<dyn Error>> {
//!     let mut htpasswd = Htpasswd::new();
//!
//!     htpasswd.add("John Doe", "Don't hardcode")?;
//!     htpasswd.verify("John Doe", "Don't hardcode");
//!     htpasswd.write_to_path("www/.htpasswd")?;
//!
//!     Ok(())
//! }
//!
//! ```

use std::collections::HashMap;
use std::path::Path;

use bcrypt::{Version, DEFAULT_COST};

#[derive(Clone, Debug, Default)]
pub struct Htpasswd {
    // Username and encrypted password
    entries: HashMap<String, String>,
}

impl Htpasswd {
    pub fn load(bytes: &str) -> Htpasswd {
        let lines = bytes.split('\n');
        let hashes = lines
            .filter_map(parse_hash_entry)
            .collect::<HashMap<String, String>>();
        Htpasswd { entries: hashes }
    }

    pub fn check(&self, username: &str, password: &str) -> bool {
        match self.entries.get(username) {
            Some(val) => match bcrypt::verify(password, val) {
                Ok(true) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn add(&mut self, username: &str, password: &str) {
        let pass = bcrypt::hash_with_result(password, DEFAULT_COST)
            .expect("REASON")
            .format_for_version(Version::TwoA);
        self.entries.insert(username.to_string(), pass);
    }

    pub fn list(&self) {
        for user in self.entries.keys() {
            println!("{user}");
        }
    }

    pub fn remove(&mut self, username: &str) {
        self.entries.remove(username);
    }

    pub fn write_to_path<P>(&self, path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
    {
        std::fs::write(path, self.to_string())
    }
}

impl ToString for Htpasswd {
    fn to_string(&self) -> String {
        self.entries
            .iter()
            .map(|(u, p)| format!("{}:{}\n", u, p))
            .collect()
    }
}

fn parse_hash_entry(entry: &str) -> Option<(String, String)> {
    let semicolon = match entry.find(':') {
        Some(idx) => idx,
        None => return None,
    };
    let username = &entry[..semicolon];

    let hash_id = &entry[(semicolon + 1)..];
    Some((username.to_string(), hash_id.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "user:$2y$05$EdYOrCBLxXsZXZp0UbK8leUlXT23agbGfTSEr3GKmvlUb7RS.1rF2";

    #[test]
    fn bcrypt_verify_htpasswd() {
        let htpasswd = Htpasswd::load(DATA);
        assert_eq!(htpasswd.check("user", "password"), true);
    }

    #[test]
    fn bcrypt_add_verify_htpasswd() {
        let mut htpasswd = Htpasswd::new();
        htpasswd.add("user1", "pass");
        assert_eq!(htpasswd.check("user1", "pass"), true);
    }

    #[test]
    fn user_not_found() {
        let htpasswd = Htpasswd::load(DATA);
        assert_eq!(htpasswd.check("user_does_not_exist", "password"), false);
    }

    #[test]
    fn user_remove() {
        let mut htpasswd = Htpasswd::new();
        htpasswd.add("user1", "pass");
        assert_eq!(htpasswd.check("user1", "pass"), true);
        htpasswd.remove("user1");
        assert_eq!(htpasswd.check("user1", "pass"), false);
    }

    // #[test]
    // fn to_str() {
    //     let htpasswd = Htpasswd::load(DATA);
    //     println!("{}", htpasswd.to_string());
    // }
}
