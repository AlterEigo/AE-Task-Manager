use crate::app::models::User;
use crate::prelude::{Error, Form, Result};

pub struct SignUpForm<'a> {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    on_submit: Option<Box<dyn FnOnce(Self) -> Result<User> + 'a>>,
}

impl<'a> SignUpForm<'a> {
    pub fn new<F>(submit_action: F) -> Self
    where
        F: 'a + FnOnce(Self) -> Result<User>,
    {
        SignUpForm {
            on_submit: Some(Box::new(submit_action)),
            first_name: Default::default(),
            last_name: Default::default(),
            email: Default::default(),
            username: Default::default(),
            password: Default::default(),
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

impl<'a> Form<User> for SignUpForm<'a> {
    fn submit(self) -> Result<User> {
        let cpy = SignUpForm {
            on_submit: None,
            ..self
        };
        match self.on_submit {
            Some(action) => action(cpy),
            _ => Err(Error::AlreadySubmitted),
        }
    }
}
