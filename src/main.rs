mod app;
mod auth;
mod prelude;
mod root;

use crate::prelude::*;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Could not initialize GTK");

    let tm = Box::new(app::Application::new());

    tm.run();
}
