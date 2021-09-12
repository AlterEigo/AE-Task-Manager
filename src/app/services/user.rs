use sha2::Digest;
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
                WHERE user_id = :uid;",
            )
            .expect("Could not prepare DB statement.");
        loop {
            let uid = nanoid::nanoid!();
            stmt.reset().unwrap();
            stmt.bind_by_name(":uid", uid.as_str())
                .expect("Unsuccessful statement parameter binding.");
            stmt.next()
                .expect("Unsuccessful statement advance.");
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
            stmt.next()
                .expect("Unsuccessful statement advance");
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
        let pass_hash = format!("{pass}{salt}", pass=form.password, salt=salt);
        let pass_hash = sha2::Sha256::digest(pass_hash.as_bytes());
        let values = vec![
                (":uid", sqlite::Value::String(new_uid.clone())),
                (":fname", sqlite::Value::String(form.first_name)),
                (":lname", sqlite::Value::String(form.last_name)),
                (":email", sqlite::Value::String(form.email)),
                (":uname", sqlite::Value::String(form.username)),
                (":password", sqlite::Value::String(format!("{:x}", pass_hash))),
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
        let conn = self.db.as_ref().unwrap().connection()?;
        let mut stmt = conn.prepare("
            SELECT user_id, salt, password FROM users WHERE username=:uname;
            ")?;
        stmt.bind_by_name(":uname", &sqlite::Value::String(u))?;
        if let sqlite::State::Done = stmt.next()? {
            return Err(Error::Unauthorized)
        }
        let user: (String, String, String) = (
            stmt.read::<String>(0)?,
            stmt.read::<String>(1)?,
            stmt.read::<String>(2)?,
        );
        let hash = format!("{pass}{salt}", pass=p, salt=user.1);
        let hash = sha2::Sha256::digest(hash.as_bytes());
        let hash = format!("{:x}", hash);
        if hash == user.2 {
            Ok(SessionId(user.0))
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn info(&self, id: SessionId) -> Result<User> {
        Err(Error::NotImplemented)
    }

    fn sign_up(&self) -> Result<SignUpForm> {
        if let Some(db) = &self.db {
            let conn = db.connection().unwrap();
            let action =
                move |form: SignUpForm| -> Result<SessionId> {
                    UserManager::register_user(form, &conn)
                };
            Ok(SignUpForm::new(action))
        } else {
            Err(Error::ServiceNotBound("Database service"))
        }
    }
}
