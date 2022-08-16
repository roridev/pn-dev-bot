
pub trait Queryable {
    fn to_query(&self) -> String;
}

impl Queryable for String {
    fn to_query(&self) -> String {
        (*self).clone()
    }
}
