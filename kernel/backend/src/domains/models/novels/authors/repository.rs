use super::*;
use crate::domains::models::users::*;
use crate::prelude::*;

trait AuthorRepository {
    fn find_by_user_id(&self, id: Id<User>) -> result::Result<Author>;
}
