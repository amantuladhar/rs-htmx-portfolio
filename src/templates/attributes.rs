pub enum Attrs<'a, V> {
    Varient(V),
    Class(&'a str),
    HxPost(&'a str),
    HxGet(&'a str),
    HxSwap(&'a str),
    HxTarget(&'a str),
}
