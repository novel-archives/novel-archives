pub type RawId = String;

pub type Id<T> = phantom_newtype::Id<T, RawId>;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hoge".to_string()=>"hoge")]
    fn from_into_works(raw_id: RawId) -> RawId {
        struct Tag;
        let id = Id::<Tag>::from(raw_id);
        id.get().clone()
    }

    #[test_case("hoge".to_string(),"fuga".to_string()=>false)]
    fn eq_works(a: RawId, b: RawId) -> bool {
        struct Tag;
        let ida = Id::<Tag>::from(a);
        let idb = Id::<Tag>::from(b);
        ida == idb
    }
}
