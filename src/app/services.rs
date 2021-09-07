pub trait DbService {
    fn connection(&self) -> &sqlite::Connection;
}

pub trait UserService {
    fn authenticate(&self, u: String, p: String) -> Option<SessionId>;

    fn info(&self, t: SessionId) -> UserModel;

    fn register(&self, f: SignUpForm) -> Option<SessionId>;
}

pub trait TaskService {}

pub trait BoardService {}
