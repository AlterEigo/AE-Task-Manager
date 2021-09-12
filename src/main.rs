mod app;
mod auth;
mod prelude;
mod root;

use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::{
    databases::MainDb,
    services::*
};

fn main() {
    gtk::init().expect("Could not initialize GTK");

    let db = MainDb::new();
    let db: Rc<dyn DbService> = Rc::new(db);

    let us = UserManager::new().database(&db);
    let us: Rc<dyn UserService> = Rc::new(us);

    let tm = app::Application::builder()
        .database(&db)
        .user_service(&us)
        .build().unwrap();

    tm.run();
}
