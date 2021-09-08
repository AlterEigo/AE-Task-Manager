use crate::prelude::View;
use gtk::prelude::*;

use crate::app::services::UserService;

pub struct AuthView<'a> {
    user_srv: &'a dyn UserService
}

impl<'a> AuthView<'a> {
    pub fn new(user_srv: &'a dyn UserService) -> Self
    {
        AuthView {
            user_srv: user_srv
        }
    }

    fn action_sign_in(btn: &gtk::Button, username: &str, password: &str) {
        println!(
            "Submitted username: '{}' password: '{}'",
            username, password
        )
    }

    fn bind_inputs(builder: &gtk::Builder) {
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

    fn bind_buttons(builder: &gtk::Builder) {
        let (e_login, e_password, b_signin) = (
            builder.object::<gtk::Entry>("e_username").unwrap(),
            builder.object::<gtk::Entry>("e_password").unwrap(),
            builder.object::<gtk::Button>("b_signin").unwrap(),
        );

        b_signin.connect_clicked(move |btn| {
            let username = String::from(e_login.text());
            let password = String::from(e_password.text());
            AuthView::action_sign_in(btn, &username, &password);
        });
    }
}

impl<'a> View for AuthView<'a> {
    fn assemble(&self) -> gtk::Widget {
        let builder = gtk::Builder::from_resource("/org/altereigo/ae-task-manager/AuthFrame.glade");

        AuthView::bind_inputs(&builder);
        AuthView::bind_buttons(&builder);
        let grid: gtk::Grid = builder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
