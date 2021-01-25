use crate::domains::models::prelude::*;
pub trait Entity: PartialEq {
    fn id(&self) -> &Id<Self>;
}
