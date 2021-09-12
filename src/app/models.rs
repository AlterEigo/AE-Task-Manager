#[derive(Default, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
}

pub struct Task;
pub struct Board;

#[derive(Clone,Debug)]
pub struct SessionId(pub String);
