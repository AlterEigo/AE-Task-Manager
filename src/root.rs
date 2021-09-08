use gtk::prelude::*;

use crate::app::services::UserService;
use crate::auth::AuthView;
use crate::prelude::View;

pub struct RootView<'a> {
    user_srv: &'a dyn UserService
}

impl<'a> RootView<'a> {
    pub fn new(user_srv: &'a dyn UserService) -> Self {
        RootView {
            user_srv: user_srv
        }
    }
}

impl<'a> View for RootView<'a> {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        let auth = AuthView::new(self.user_srv);
        grid.attach(&auth, 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
