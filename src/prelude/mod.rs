pub enum Error {
    NotImplemented,
    Unauthorized
}

impl Error {
    pub fn msg(&self) -> &'static str {
        match &self {
            NotImplemented => "Not implemented.",
            Unauthorized => "Did not pass authentication."
        }
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
