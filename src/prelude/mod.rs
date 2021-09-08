pub enum Error {
    NotImplemented,
    Unauthorized,
    NotFound,
}

impl Error {
    pub fn msg(&self) -> &'static str {
        match &self {
            NotImplemented => "NotImplemented: method or function not implemented.",
            Unauthorized => "Unauthorized: did not pass authentication.",
            NotFound => "NotFound: could not found requested data.",
            _ => "Unknown: error type not described",
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
