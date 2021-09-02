use gtk::prelude::*;

use crate::auth::{AuthView};
use crate::prelude::{
    View
};

pub struct RootView {
    auth: AuthView
}

impl View for RootView {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        grid.attach(&self.auth.assemble(), 0, 0, 1, 1);
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}

impl RootView {
    pub fn new() -> Self {
        RootView {
            auth: AuthView::new()
        }
    }
}
