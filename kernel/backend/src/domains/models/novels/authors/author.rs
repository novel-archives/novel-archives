use super::*;

#[derive(PartialEq)]
pub struct AuthorName(String);

#[derive(Entity, new, Getters)]
pub struct Author {
    id: Id<Author>,
    name: AuthorName,
}

pub trait AuthorIdGenerator {
    fn generate(&self) -> result::Result<Id<Author>>;
}
