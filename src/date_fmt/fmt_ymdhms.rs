
use chrono::NaiveDateTime;
use serde::{self, Serializer};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

// 指定序列化方式 to string
pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    // match date {
    //     Some(d) => {
    //         let s = format!("{}", d.format(FORMAT));
    //         return serializer.serialize_str(&s)
    //     },
    //     None => Error::custom("nonoe")
    // };

    let s = format!("{}", date.unwrap().format(FORMAT));
    serializer.serialize_str(&s)
}

// 反序列化方式
// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<D>(D) -> Result<T, D::Error> where D: Deserializer
//
// although it may also be generic over the output types T.
// pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
// }

