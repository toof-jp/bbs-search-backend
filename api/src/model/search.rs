use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub main_text: String,
    #[serde(default)]
    pub name_and_trip: String,
    pub cursor: Option<i32>,
    #[serde(default)]
    pub ascending: bool,
    #[serde(default)]
    pub oekaki: bool,
    #[serde(default, deserialize_with = "deserialize_date")]
    pub since: Option<NaiveDate>,
    #[serde(default, deserialize_with = "deserialize_date")]
    pub until: Option<NaiveDate>,
}

fn deserialize_date<'de, D>(de: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    const FORMAT: &str = "%Y-%m-%d";
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => NaiveDate::parse_from_str(s, FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}
