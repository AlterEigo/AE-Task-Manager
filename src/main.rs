mod ui;
mod core;

use ui::{application::*};

fn main() {
    let app = Application::new();

    app.run();
}
