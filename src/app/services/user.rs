use crate::prelude::*;
use crate::app::{
    services::{DbService, UserService},
    models::{SessionId, User},
    forms::SignUpForm
};

use std::rc::Rc;

pub struct UserManager {
    db: Option<Rc<dyn DbService>>,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager { db: None }
    }

    pub fn database(self, val: &Rc<dyn DbService>) -> Self {
        UserManager {
            db: Some(val.clone()),
            ..self
        }
    }

    fn unique_uid(db_conn: &sqlite::Connection) -> String {
        let mut stmt = db_conn
            .prepare(
                "
                SELECT COUNT(user_id)
                FROM users
                WHERE user_id = :uid",
            )
            .expect("Could not prepare DB statement.");
        loop {
            let uid = nanoid::nanoid!();
            stmt.reset().unwrap();
            stmt.bind_by_name(":uid", uid.as_str())
                .expect("Unsuccessful statement parameter binding.");
            if let sqlite::State::Done = stmt.next().unwrap() {
                break uid;
            }
        }
    }

    fn unique_salt(db_conn: &sqlite::Connection) -> String {
        let mut stmt = db_conn
            .prepare(
                "
                SELECT COUNT(user_id)
                FROM users
                WHERE salt = :salt",
            )
            .expect("Could not prepare DB statement.");
        loop {
            let salt = nanoid::nanoid!();
            stmt.reset().unwrap();
            stmt.bind_by_name(":salt", salt.as_str())
                .expect("Unsuccessful statement parameter binding.");
            if let sqlite::State::Done = stmt.next().unwrap() {
                break salt;
            }
        }
    }

    fn register_user(form: SignUpForm, db_conn: &sqlite::Connection) -> Result<SessionId> {
        let mut stmt = db_conn.prepare("
                INSERT INTO users
                VALUES (:uid, :fname, :lname, :email, :uname, :password, :salt);
            ")?;
        let new_uid = Self::unique_uid(db_conn);
        let salt = Self::unique_salt(db_conn);
        let values = vec![
                (":uid", sqlite::Value::String(new_uid.clone())),
                (":fname", sqlite::Value::String(form.first_name)),
                (":lname", sqlite::Value::String(form.last_name)),
                (":email", sqlite::Value::String(form.email)),
                (":uname", sqlite::Value::String(form.username)),
                (":password", sqlite::Value::String(form.password)),
                (":salt", sqlite::Value::String(salt)),
        ];
        for pair in values.iter() {
            stmt.bind_by_name(pair.0, &pair.1)?;
        };
        stmt.next()?;
        Ok(SessionId(new_uid))
    }
}

impl UserService for UserManager {
    fn authenticate(&self, u: String, p: String) -> Result<SessionId> {
        Err(Error::NotImplemented)
    }

    fn info(&self, id: SessionId) -> Result<User> {
        Err(Error::NotImplemented)
    }

    fn sign_up(&self) -> Result<SignUpForm> {
        if let Some(db) = &self.db {
            let conn = db.connection();
            let action =
                move |form: SignUpForm| -> Result<SessionId> { UserManager::register_user(form, conn) };
            Ok(SignUpForm::new(action))
        } else {
            Err(Error::ServiceNotBound("Database service"))
        }
    }
}
