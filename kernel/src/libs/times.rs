use crate::prelude::*;
use chrono::Utc;

pub fn now() -> DateTime {
    Utc::now().with_timezone(&chrono_tz::Tz::Asia__Tokyo)
}
