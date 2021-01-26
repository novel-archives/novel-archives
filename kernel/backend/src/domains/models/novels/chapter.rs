use super::*;

#[derive(PartialEq)]
pub struct ChapterTitle(String);

#[derive(new, Entity, Getters)]
pub struct Chapter {
    id: Id<Self>,
    title: ChapterTitle,
    order: Order,
    body: Body,
}
