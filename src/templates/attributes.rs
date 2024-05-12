use crate::auth::cookies_and_jwt::LoggedInUser;

pub enum Attrs<'a, V> {
    LoggedInUser(&'a Option<LoggedInUser>),
    Varient(V),
    Class(&'a str),
    Placeholder(&'a str),
    Required(&'a str),
    Name(&'a str),
    Id(&'a str),
    Type(&'a str),
    Value(&'a str),
    #[allow(dead_code)]
    Form(&'a str),
    #[allow(unused)]
    HxPost(&'a str),
    HxGet(&'a str),
    HxSwap(&'a str),
    HxTarget(&'a str),
    HxPushUrl(&'a str),
    HxExt(&'a str),
    HxSelect(&'a str),
    HxTrigger(&'a str),
}
