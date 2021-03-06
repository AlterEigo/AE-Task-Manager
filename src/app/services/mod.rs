use crate::app::forms::SignUpForm;
use crate::app::models::{SessionId, User};
use crate::prelude::Result;

mod user;

pub use user::UserManager;

pub trait DbService {
    fn connection(&self) -> Result<sqlite::Connection>;
}

pub trait UserService {
    fn authenticate(&self, u: String, p: String) -> Result<SessionId>;

    fn info(&self, t: SessionId) -> Result<User>;

    fn sign_up(&self) -> Result<SignUpForm>;
}

pub trait TaskService {}

pub trait BoardService {}
