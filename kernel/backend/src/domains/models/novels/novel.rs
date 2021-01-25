use super::*;

#[derive(Entity, new, Getters)]
pub struct Novel {
    id: Id<Self>,
    title: String,
    order: Order,
    parts: Vec<Id<Part>>,
}
