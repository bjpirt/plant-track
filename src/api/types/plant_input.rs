use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlantInput {
    pub name: String,
    pub species: String,
}
