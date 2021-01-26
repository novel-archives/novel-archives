use super::*;

#[derive(PartialEq)]
pub struct PartTitle(String);

#[derive(Entity, new, Getters)]
pub struct Part {
    id: Id<Self>,
    title: PartTitle,
    order: Order,
    chapters: Vec<Id<Chapter>>,
}
