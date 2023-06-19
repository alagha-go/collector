use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub gender: Gender,
    pub adult: bool,
    pub department: String,
    pub imdb_id: String,
    pub profile: String,
    pub birthday: Option<NaiveDate>,
    pub place_of_birth: String,
    pub popularity: f32,
    pub biography: String,
    pub homepage: String,
    pub deathday: Option<NaiveDate>
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum Gender {
    #[default]
    UnSpedified  = 0,
    Female = 1,
    Male = 2,
    NonBinary = 3
}