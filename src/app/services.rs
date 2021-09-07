use crate::app::models::{SessionId, SignUpForm, User};

use crate::prelude::Form;

pub trait DbService {
    fn connection(&self) -> &sqlite::Connection;
}

pub trait UserService {
    type SignUpForm: Form<User>;

    fn authenticate(&self, u: String, p: String) -> Option<SessionId>;

    fn info(&self, t: SessionId) -> User;

    fn sign_up(&self) -> Self::SignUpForm;
}

pub trait TaskService {}

pub trait BoardService {}
