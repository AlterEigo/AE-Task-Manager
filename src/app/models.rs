use crate::prelude::{Form, Result, Error};

pub struct SignUpForm {
    first_name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
    on_submit: Box<dyn Fn(Self) -> Result<User>>
}

impl SignUpForm {
    pub fn new<F>(submit_action: F) -> Self
        where F: Fn(Self) -> Result<User> + 'static
    {
        SignUpForm {
            on_submit: Box::new(submit_action),
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
    fn submit(self) -> Result<User> {
        (self.on_submit)(self)
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
