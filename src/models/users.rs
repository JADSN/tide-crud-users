use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(u16);
impl UserId {
    pub fn new(data: u16) -> Self {
        Self(data)
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserName(String);
impl UserName {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn get(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmail(String);
impl UserEmail {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn get(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDepartment(u16);
impl UserDepartment {
    pub fn new(data: u16) -> Self {
        Self(data)
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermission(u16);
impl UserPermission {
    pub fn new(data: u16) -> Self {
        Self(data)
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus(u16);

impl Default for UserStatus {
    fn default() -> Self {
        // * Status: Disabled
        UserStatus(2)
    }
}

impl UserStatus {
    pub fn new(data: u16) -> Self {
        Self(data)
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}
