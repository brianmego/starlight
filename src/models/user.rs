use serde::Serialize;

#[derive(Serialize)]
pub struct User<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> User<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Self {
        Self { username, password }
    }
}


