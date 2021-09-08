mod app;
mod auth;
mod prelude;
mod root;

use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::services::*;
use crate::app::{MainDb, UserManager};

fn main() {
    gtk::init().expect("Could not initialize GTK");

    let db: Option<Rc<dyn DbService>> = match MainDb::new() {
        Ok(created) => Some(Rc::new(created)),
        _ => None,
    };

    let us: Option<Rc<dyn UserService>> = Some(Rc::new(UserManager::new().database(&db)));
    let tm = Box::new(app::Application::new().database(db).user_service(us));

    tm.run();
}
