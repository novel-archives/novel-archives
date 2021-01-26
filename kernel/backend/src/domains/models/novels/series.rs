use super::*;

#[derive(PartialEq)]
pub struct SeriesTitle(String);

#[derive(new, Entity, Getters)]
pub struct Series {
    id: Id<Self>,
    title: SeriesTitle,
    novels: Vec<Id<Novel>>,
}
