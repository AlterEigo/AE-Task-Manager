mod app;
mod auth;
mod prelude;
mod root;

use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::{MainDb, UserManager};

fn main() {
    gtk::init().expect("Could not initialize GTK");

    let db = Rc::new(MainDb::new().unwrap());
    let us = Rc::new(UserManager::new().database(db.clone()));
    let tm = Box::new(app::Application::new().database(db.clone()).user_service(us.clone()));

    tm.run();
}
