use crate::{
    db::dynamo::{get_dynamo_client::get_dynamo_client, get_user::get_user},
    types::{plant::Plant, plant_input::PlantInput},
};
use aws_sdk_dynamodb::{
    operation::put_item::builders::PutItemFluentBuilder, types::AttributeValue, Client,
};

pub async fn create_plant(username: &str, plant_input: &PlantInput) -> Result<Plant, String> {
    let user = get_user(username.to_string()).await?;
    let new_plant = Plant::new(&plant_input.name, &plant_input.species, &user.id);

    let client: Client = get_dynamo_client().await;

    let request: PutItemFluentBuilder = client
        .put_item()
        .table_name(String::from("plants"))
        .item("id", AttributeValue::S(new_plant.id.clone()))
        .item("user_id", AttributeValue::S(user.id.clone()))
        .item("name", AttributeValue::S(new_plant.name.clone()))
        .item("species", AttributeValue::S(new_plant.species.clone()))
        .item("created", AttributeValue::S(new_plant.created.to_string()));

    let res = request.send().await;

    match res {
        Ok(_) => Ok(new_plant),
        Err(e) => {
            println!("{e}");
            Err(String::from("Error creating plant for user"))
        }
    }
}
