use crate::{
    db::base_gateway::BaseGateway,
    types::{plant::Plant, plant_input::PlantInput, user::User},
};

pub async fn create_plant(
    user: &User,
    plant_input: &PlantInput,
    gateway: &impl BaseGateway,
) -> Result<Plant, String> {
    let new_plant = Plant::new(&plant_input.name, &plant_input.species, &user.id);
    let result = gateway.create_plant(&new_plant).await;

    match result {
        Ok(_) => Ok(new_plant),
        Err(_) => Err(String::from("Error creating plant for user")),
    }
}
