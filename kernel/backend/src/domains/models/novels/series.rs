use super::*;

#[derive(new, Entity, Getters)]
pub struct Series {
    id: Id<Self>,
    title: String,
    novels: Vec<Id<Novel>>,
}
