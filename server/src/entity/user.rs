#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub age: i8,
}

impl User {
    pub fn new(username: &str, age: i8) -> Self {
        Self {
            username: username.to_owned(),
            age,
        }
    }
}
