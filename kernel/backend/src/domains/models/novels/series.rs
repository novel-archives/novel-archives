use super::*;

#[derive(PartialEq)]
pub struct SeriesTitle(String);

#[derive(new, Entity, Getters)]
pub struct Series {
    id: Id<Self>,
    author_id: Id<Author>,
    title: SeriesTitle,
    novels: Vec<Id<Novel>>,
}
