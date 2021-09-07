pub struct Error(Option<i32>, Option<String>);
impl Error {
    pub fn code(&self) -> Option<i32> {
        self.0
    }

    pub fn msg(&self) -> Option<String> {
        self.1.clone()
    }
}

pub type Result<Data> = std::result::Result<Data, Error>;

pub trait Builder<T> {
    fn build(&self) -> T;
}

pub trait View {
    fn assemble(&self) -> gtk::Widget;
}

pub trait Form<T> {
    fn submit(self) -> Result<T>;
}
