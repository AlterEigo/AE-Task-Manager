use crate::app::Error;
use crate::prelude::Form;

#[derive(Debug)]
pub struct SignUpForm {
    first_name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
    on_submit: fn() -> Result<User, Error>,
}

impl SignUpForm {
    pub fn new(submit_action: fn() -> Result<User, Error>) -> Self {
        SignUpForm {
            on_submit: submit_action,
            first_name: Default::default(),
            last_name: Default::default(),
            email: Default::default(),
            username: Default::default(),
            password: Default::default()
        }
    }

    pub fn first_name(self, value: String) -> Self {
        SignUpForm {
            first_name: value,
            ..self
        }
    }

    pub fn last_name(self, value: String) -> Self {
        SignUpForm {
            last_name: value,
            ..self
        }
    }

    pub fn email(self, value: String) -> Self {
        SignUpForm {
            email: value,
            ..self
        }
    }

    pub fn username(self, value: String) -> Self {
        SignUpForm {
            username: value,
            ..self
        }
    }

    pub fn password(self, value: String) -> Self {
        SignUpForm {
            password: value,
            ..self
        }
    }
}

impl Form<User> for SignUpForm {
    fn submit(self) -> Result<User, Error> {
        (self.on_submit)()
    }
}

#[derive(Default,Debug)]
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
