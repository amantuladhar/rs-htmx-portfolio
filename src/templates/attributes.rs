use crate::auth::cookies_and_jwt::LoggedInUser;

pub enum Attrs<'a, V> {
    LoggedInUser(Option<LoggedInUser>),
    Varient(V),
    Class(&'a str),
    Placeholder(&'a str),
    Required(&'a str),
    Name(&'a str),
    Id(&'a str),
    Type(&'a str),
    #[allow(unused)]
    HxPost(&'a str),
    HxGet(&'a str),
    HxSwap(&'a str),
    HxTarget(&'a str),
    HxPushUrl(&'a str),
    HxExt(&'a str),
    HxSelect(&'a str),
}
