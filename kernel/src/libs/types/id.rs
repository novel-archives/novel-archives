use super::GenericId;
use uuid::Uuid;

pub type Id<T> = GenericId<T, Uuid>;
