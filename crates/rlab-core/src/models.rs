use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Name must not be empty".into());
        };
        if email.trim().is_empty() {
            return Err("Email must not be empty".into());
        };
        // TODO MB regex later?
        if !email.contains("@") {
            return Err("Invalid email".into());
        };
        Ok(Self { id: 0, name, email })
    }
    
}
