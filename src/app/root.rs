use gtk::prelude::*;
use crate::prelude::*;

pub struct RootView {
    content: Box<gtk::Grid>,
}

impl RootView {
    pub fn new() -> Self {
        RootView {
            content: Box::new(gtk::Grid::builder().build())
        }
    }
}

impl View for RootView {
    fn assemble(&self) -> Box<gtk::Widget> {
        let cpy: gtk::Grid = *(self.content).clone();
        Box::new(cpy.upcast::<gtk::Widget>())
    }
}

