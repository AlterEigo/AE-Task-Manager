mod app;
mod auth;
mod prelude;
mod root;

use gtk::prelude::*;
use crate::prelude::*;

fn main() {
    let tm = Box::new(app::Application::new());
    
    tm.run();
}
