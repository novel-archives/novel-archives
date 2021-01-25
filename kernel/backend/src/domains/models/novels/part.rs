use super::*;

#[derive(Entity, new, Getters)]
pub struct Part {
    id: Id<Self>,
    title: String,
    order: Order,
    chapters: Vec<Id<Chapter>>,
}
