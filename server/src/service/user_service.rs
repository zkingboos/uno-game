use std::collections::HashMap;
use crate::entity::user::User;

#[derive(Debug, Clone)]
pub struct UserService {
    registry: HashMap<String, User>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn add(&mut self, user: &User) -> Result<(), ()> {
        match self.registry.insert(user.username.clone(), user.to_owned()) {
            Some(_) => Err(()),
            None => Ok(()),
        }
    }

    pub fn search(&self, username: &str) -> Option<User> {
        self.registry.get(username).cloned()
    }

    pub fn remove(&mut self, user: User) {
        self.registry.remove(&user.username);
    }
}