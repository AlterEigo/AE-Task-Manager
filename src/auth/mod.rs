use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::services::UserService;

pub struct AuthView {
    user_srv: Option<Rc<dyn UserService>>,
}

impl AuthView {
    pub fn new() -> Self {
        AuthView {
            user_srv: None
        }
    }

    pub fn user_service(self, value: Rc<dyn UserService>) -> Self {
        AuthView {
            user_srv: Some(value),
            ..self
        }
    }

    fn bind_inputs(&self, builder: &gtk::Builder) {
        let (e_login, e_password, b_signin) = (
            builder.object::<gtk::Entry>("e_username").unwrap(),
            builder.object::<gtk::Entry>("e_password").unwrap(),
            builder.object::<gtk::Button>("b_signin").unwrap(),
        );

        let btn = b_signin.clone();
        e_login.connect_activate(move |_| btn.emit_clicked());

        let btn = b_signin.clone();
        e_password.connect_activate(move |_| btn.emit_clicked());
    }

    fn bind_buttons(&self, builder: &gtk::Builder) {
        let (e_login, e_password, b_signin)= (
            builder.object::<gtk::Entry>("e_username").unwrap(),
            builder.object::<gtk::Entry>("e_password").unwrap(),
            builder.object::<gtk::Button>("b_signin").unwrap(),
        );

        if let Some(srv) = self.user_srv {
            let user_srv = Rc::clone(&srv);
            b_signin.connect_clicked(move |btn| {
                let username = String::from(e_login.text());
                let password = String::from(e_password.text());
                if let Ok(form) = user_srv.sign_up() {
                    form.username(username).password(password).submit();
                }
                // self.action_sign_in(btn, &username, &password);
            });
        }
    }
}

impl View for AuthView {
    fn assemble(&self) -> gtk::Widget {
        let builder = gtk::Builder::from_resource("/org/altereigo/ae-task-manager/AuthFrame.glade");

        self.bind_inputs(&builder);
        self.bind_buttons(&builder);
        let grid: gtk::Grid = builder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
