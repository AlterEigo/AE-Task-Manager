use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod ui;

use crate::ui::Frame;

fn main() {
    let app = Application::builder()
        .application_id("org.altereigo.tmanager")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .title("Task Manager")
            .build();

        window.show();
    });

    app.run();
}
