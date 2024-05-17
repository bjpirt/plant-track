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

#[cfg(test)]
mod tests {
    use crate::{
        db::{base_gateway::BaseGateway, dynamo_gateway::DynamoGateway},
        handlers::create_plant::create_plant,
        lib::random_id::random_id,
        types::{plant_input::PlantInput, user::User},
    };

    #[async_test]
    async fn it_creates_a_new_plant() -> Result<(), String> {
        let dynamo_gateway = DynamoGateway::new().await;
        let test_user = User::new("username", "name", "email", "password");
        let new_plant = PlantInput {
            name: random_id(20),
            species: "species".to_string(),
        };
        //test no plants exist with the id
        let existing_plants = dynamo_gateway.get_plants_by_user(&test_user.id).await?;
        assert_eq!(existing_plants.len(), 0);
        //create plant
        create_plant(&test_user, &new_plant, &dynamo_gateway).await?;
        //test plant exists
        let all_plants = dynamo_gateway.get_plants_by_user(&test_user.id).await?;
        assert_eq!(all_plants.len(), 1);
        assert_eq!(all_plants.get(0).unwrap().name, new_plant.name);
        Ok(())
    }
}
