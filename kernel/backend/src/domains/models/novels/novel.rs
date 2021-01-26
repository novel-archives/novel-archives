use super::*;

#[derive(PartialEq)]
pub struct NovelTitle(String);

#[derive(Entity, new, Getters)]
pub struct Novel {
    id: Id<Self>,
    title: NovelTitle,
    order: Order,
    parts: Vec<Id<Part>>,
}
