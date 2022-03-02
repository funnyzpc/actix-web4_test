
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::date_fmt;



#[derive(Deserialize, Serialize)]
pub struct SysMenu {
    #[serde(with = "rust_decimal::serde::str")]
    pub id: Decimal,
    pub name: String,
    // #[serde(with = "rust_decimal::serde::str")]
    // show_flag: Decimal,
    // show_flag: i8,
    pub show_flag: Option<i16>,
    // TIMESTAMP WITH TIME ZONE type to interact with DateTime<Utc>.
    // NaiveDateTime type interacts with TIMESTAMP
    // create_date: Option<DateTime<Utc>>,
    // create_date: NaiveDateTime,
    #[serde(serialize_with = "date_fmt::fmt_ymdhms::serialize",skip_serializing_if="Option::is_none")]
    // create_date: NaiveDateTime,
    pub create_date: Option<NaiveDateTime>,
    // create_date: Option<DateTime<Utc>>,
    pub code:String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_code:Option<String>,
}
