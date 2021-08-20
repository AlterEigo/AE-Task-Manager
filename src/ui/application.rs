use gtk::prelude::*;

pub struct Application {
    parent: gtk::Application,
}

impl Application {
    pub fn new() -> Application {
        let app: Application = Application {
            parent: gtk::Application::builder()
                .application_id("org.altereigo.task-manager")
                .build()
        };

        app.parent.connect_activate(|app| {
            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .title("Æ Task Manager")
                .default_width(800)
                .default_height(600)
                .build();

            window.show();
        });

        return app;
    }

    pub fn run(&self) -> i32 {
        self.parent.run()
    }
}
