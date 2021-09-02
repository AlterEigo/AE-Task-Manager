use gtk::prelude::*;
use crate::prelude::{
    View
};

pub struct AuthView {

}

impl AuthView {
    pub fn new() -> Self {
        AuthView {}
    }
}

impl View for AuthView {
    fn assemble(&self) -> gtk::Widget {
        let grid = gtk::Grid::builder().build();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
