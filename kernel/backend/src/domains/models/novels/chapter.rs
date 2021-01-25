use super::*;

#[derive(new, Entity, Getters)]
pub struct Chapter {
    id: Id<Chapter>,
    title: String,
    order: Order,
    body: Body,
}
