pub enum Attrs<'a, V> {
    Varient(V),
    Class(&'a str),
    Placeholder(&'a str),
    Name(&'a str),
    Id(&'a str),
    Type(&'a str),
    #[allow(unused)]
    HxPost(&'a str),
    HxGet(&'a str),
    HxSwap(&'a str),
    HxTarget(&'a str),
    HxPushUrl(&'a str),
}
