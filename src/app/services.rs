use crate::prelude::Result;
use crate::app::models::{SessionId, User};
use crate::app::forms::SignUpForm;

pub trait DbService {
    fn connection(&self) -> &sqlite::Connection;
}

pub trait UserService {
    fn authenticate(&self, u: String, p: String) -> Result<SessionId>;

    fn info(&self, t: SessionId) -> Result<User>;

    fn sign_up(&self) -> SignUpForm;
}

pub trait TaskService {}

pub trait BoardService {}
