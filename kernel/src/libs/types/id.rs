use std::marker::PhantomData;
pub type RawId = String;
pub struct Id<T> {
    raw_id: RawId,
    _phantom: PhantomData<T>,
}

impl<T> From<RawId> for Id<T> {
    fn from(raw_id: RawId) -> Id<T> {
        Id {
            raw_id,
            _phantom: PhantomData,
        }
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, r: &Self) -> bool {
        self.raw_id == r.raw_id
    }
}

impl<T> Id<T> {
    pub fn raw_id(&self) -> &RawId {
        &self.raw_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge".to_string()=>"hoge")]
    fn from_into_works(raw_id: RawId) -> RawId {
        struct Tag;
        let id = Id::<Tag>::from(raw_id);
        id.raw_id().clone()
    }

    #[test_case("hoge".to_string(),"fuga".to_string()=>false)]
    fn eq_works(a: RawId, b: RawId) -> bool {
        struct Tag;
        let ida = Id::<Tag>::from(a);
        let idb = Id::<Tag>::from(b);
        ida == idb
    }
}
