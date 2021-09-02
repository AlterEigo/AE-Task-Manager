use gtk::prelude::*;

pub trait Builder<T> {
    fn build(&self) -> T;
}

pub trait View {
    fn assemble(&self) -> Box<gtk::Widget>;
}

