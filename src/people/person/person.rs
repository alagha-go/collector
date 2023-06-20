use super::*;
use serde_json::{Value, Map, Number};

impl Person {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn from_imdb_json(id: &u32) -> Result<String> {
        let url = PERSONURL.replace("ID", &id.to_string());
        let url = url.replace("APIKEY", &APIKEY);
        Ok(CLIENT.get(url).send().await?.text().await?)
    }

    pub async fn from_imdb(id: &u32) -> Result<Self> {
        let value: Value = serde_json::from_str(&Self::from_imdb_json(id).await?)?;
        Ok(value.into())
    }

    pub fn from_json(json: &str) -> Result<Self> {
        let value: Value = serde_json::from_str(json)?;
        Ok(value.into())
    }
}

impl From<Value> for Person {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => map.into(),
            _ => Self::new()
        }
    }
}


impl From<Map<String, Value>> for Person {
    fn from(mut map: Map<String, Value>) -> Self {
        let id = map.remove("id").unwrap_or_default().as_i64().unwrap_or_default() as u32;
        let name = map.remove("name").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let gender: Gender = map.remove("gender").unwrap_or_default().into();
        let adult = map.remove("adult").unwrap_or_default().as_bool().unwrap_or_default();
        let department = map.remove("known_for_department").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let imdb_id = map.remove("imdb_id").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let profile = map.remove("profile_path").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let birthday = NaiveDate::parse_from_str(map.remove("birthday").unwrap_or_default().as_str().unwrap_or_default(), "%Y-%m-%d").ok();
        let place_of_birth = map.remove("place_of_birth").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let popularity = map.remove("popularity").unwrap_or_default().as_f64().unwrap_or_default() as f32;
        let biography = map.remove("biography").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let homepage = map.remove("homepage").unwrap_or_default().as_str().unwrap_or_default().to_string();
        let deathday = NaiveDate::parse_from_str(map.remove("birthday").unwrap_or_default().as_str().unwrap_or_default(), "%Y-%m-%d").ok();
        Self{id, name, gender, adult, department, imdb_id, profile, birthday, place_of_birth, popularity, biography, homepage, deathday}
    }
}