mod app;
mod auth;
mod prelude;
mod root;

use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::{
    databases::MainDb,
    UserManager,
    services::*
};

fn main() {
    gtk::init().expect("Could not initialize GTK");


    let db: Rc<dyn DbService> = Rc::new(MainDb::new().expect("Database not initialized."));
    let us: Rc<dyn UserService> = Rc::new(UserManager::new().database(&db));

    let tm = Box::new(app::Application::new().database(&db).user_service(&us));

    tm.run();
}
