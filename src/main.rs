mod app;
mod auth;
mod prelude;
mod root;

use crate::prelude::*;
use gtk::prelude::*;

use crate::app::{MainDb, UserManager};

fn main() {
    gtk::init().expect("Could not initialize GTK");

    let db = MainDb::new().unwrap();
    let us = UserManager::new().database(&db);
    let tm = Box::new(app::Application::new()
        .database(&db)
        .user_service(&us));

    tm.run();
}
