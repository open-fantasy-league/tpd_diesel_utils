// TODO commonise
// https://github.com/serde-rs/serde/issues/1444 kind of sucks
use crate::{my_timespan_format::deserialize as des, DieselTimespan};
use serde::{self, Deserialize, Deserializer};

// Only need a deserializer as cant have options out. Only options in in changesets
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DieselTimespan>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "des")] DieselTimespan);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}
