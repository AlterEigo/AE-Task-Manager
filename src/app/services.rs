use crate::app::models::{User};

use crate::prelude::{Form, Result};

pub trait DbService {
    fn connection(&self) -> &sqlite::Connection;
}

pub trait UserService {
    type SignUpForm: Form<User>;
    type SessionId;

    fn authenticate(&self, u: String, p: String) -> Result<Self::SessionId>;

    fn info(&self, t: Self::SessionId) -> User;

    fn sign_up(&self) -> Self::SignUpForm;
}

pub trait TaskService {}

pub trait BoardService {}
