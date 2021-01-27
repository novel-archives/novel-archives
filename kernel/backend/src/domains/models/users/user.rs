use super::*;

#[derive(Entity, new, Getters)]
pub struct User {
    id: Id<User>,
    name: UserName,
}
