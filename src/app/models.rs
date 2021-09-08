#[derive(Default, Debug)]
pub struct User {
    first_name: String,
    last_name: String,
    id: String,
    email: String,
    password: String,
    salt: String,
}

pub struct Task;
pub struct Board;
pub struct SessionId(String);
