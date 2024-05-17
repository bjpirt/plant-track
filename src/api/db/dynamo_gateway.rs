use std::{env, panic};

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    config::Builder,
    operation::{put_item::builders::PutItemFluentBuilder, query::builders::QueryFluentBuilder},
    types::AttributeValue,
    Client,
};
use chrono::DateTime;

use crate::types::{plant::Plant, user::User};

use super::base_gateway::BaseGateway;

pub struct DynamoGateway {
    client: Client,
}

impl DynamoGateway {
    pub async fn new() -> Self {
        let base_config: aws_config::SdkConfig =
            aws_config::load_defaults(BehaviorVersion::latest()).await;

        let endpoint_url: String = match env::var("AWS_ENDPOINT_URL") {
            Ok(val) => val,
            Err(_) => String::from(""),
        };

        let mut builder: Builder = aws_sdk_dynamodb::config::Builder::from(&base_config);

        if endpoint_url != "" {
            builder = builder.endpoint_url(endpoint_url);
        }

        Self {
            client: Client::from_conf(builder.build()),
        }
    }

    async fn get_user_from_index(
        &self,
        index: &str,
        key: &str,
        value: &String,
    ) -> Result<User, String> {
        let request: QueryFluentBuilder = self
            .client
            .query()
            .table_name(String::from("users"))
            .index_name(index)
            .key_condition_expression(format!("{key} = :value"))
            .expression_attribute_values(":value", AttributeValue::S(value.to_string()));

        let res = request.send().await;
        match res {
            Ok(results) => {
                if let Some(items) = results.items {
                    if let Some(user) = items.get(0) {
                        let found_user = panic::catch_unwind(|| User {
                            id: user.get("id").unwrap().as_s().unwrap().clone(),
                            username: user.get("username").unwrap().as_s().unwrap().clone(),
                            name: user.get("name").unwrap().as_s().unwrap().clone(),
                            email: user.get("email").unwrap().as_s().unwrap().clone(),
                            password: user.get("password").unwrap().as_s().unwrap().clone(),
                            created: DateTime::parse_from_rfc3339(
                                user.get("created").unwrap().as_s().unwrap(),
                            )
                            .unwrap()
                            .into(),
                            validated: user.get("validated").unwrap().as_bool().unwrap().clone(),
                            contact: user.get("contact").unwrap().as_bool().unwrap().clone(),
                        });
                        return match found_user {
                            Ok(u) => Ok(u),
                            Err(_) => Err("Could not find user".to_string()),
                        };
                    }
                }
                Err(String::from("Could not find user"))
            }
            Err(_) => Err(String::from("Could not find user")),
        }
    }
}

impl BaseGateway for DynamoGateway {
    async fn create_user(&self, user: &User) -> Result<(), String> {
        let result = self
            .client
            .put_item()
            .table_name(String::from("users"))
            .item("id", AttributeValue::S(user.id.clone()))
            .item("username", AttributeValue::S(user.username.clone()))
            .item("name", AttributeValue::S(user.name.clone()))
            .item("email", AttributeValue::S(user.email.clone()))
            .item("password", AttributeValue::S(user.password.clone()))
            .item("created", AttributeValue::S(user.created.to_rfc3339()))
            .item("validated", AttributeValue::Bool(user.validated))
            .item("contact", AttributeValue::Bool(user.contact))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error creating user")),
        }
    }

    #[cfg(test)]
    async fn get_user(&self, user_id: &str) -> Result<User, String> {
        let res = self
            .client
            .get_item()
            .table_name("users")
            .key("id", AttributeValue::S(user_id.to_string()))
            .send()
            .await;

        if let Ok(res) = res {
            if let Some(item) = res.item {
                let found_user = panic::catch_unwind(|| User {
                    id: item.get("id").unwrap().as_s().unwrap().clone(),
                    username: item.get("username").unwrap().as_s().unwrap().clone(),
                    name: item.get("name").unwrap().as_s().unwrap().clone(),
                    email: item.get("email").unwrap().as_s().unwrap().clone(),
                    password: item.get("password").unwrap().as_s().unwrap().clone(),
                    created: DateTime::parse_from_rfc3339(
                        item.get("created").unwrap().as_s().unwrap(),
                    )
                    .unwrap()
                    .into(),
                    validated: item.get("validated").unwrap().as_bool().unwrap().clone(),
                    contact: item.get("contact").unwrap().as_bool().unwrap().clone(),
                });
                if let Ok(found_user) = found_user {
                    return Ok(found_user);
                }
            }
        }
        Err("Could not find user".to_string())
    }

    async fn get_user_by_username(&self, username: &String) -> Result<User, String> {
        self.get_user_from_index("usernameIndex", "username", username)
            .await
    }

    async fn get_user_by_email(&self, email: &String) -> Result<User, String> {
        self.get_user_from_index("emailIndex", "email", email).await
    }

    async fn create_plant(&self, plant: &Plant) -> Result<(), String> {
        let request: PutItemFluentBuilder = self
            .client
            .put_item()
            .table_name(String::from("plants"))
            .item("id", AttributeValue::S(plant.id.clone()))
            .item("user_id", AttributeValue::S(plant.user_id.clone()))
            .item("name", AttributeValue::S(plant.name.clone()))
            .item("species", AttributeValue::S(plant.species.clone()))
            .item("created", AttributeValue::S(plant.created.to_rfc3339()));

        let res = request.send().await;

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error creating plant for user")),
        }
    }

    #[cfg(test)]
    async fn get_plant(&self, plant_id: &str) -> Result<Plant, String> {
        let res = self
            .client
            .get_item()
            .table_name("plants")
            .key("id", AttributeValue::S(plant_id.to_string()))
            .send()
            .await;

        if let Ok(res) = res {
            if let Some(item) = res.item {
                let plant = panic::catch_unwind(|| Plant {
                    id: item.get("id").unwrap().as_s().unwrap().clone(),
                    user_id: item.get("user_id").unwrap().as_s().unwrap().clone(),
                    name: item.get("name").unwrap().as_s().unwrap().clone(),
                    species: item.get("species").unwrap().as_s().unwrap().clone(),
                    created: DateTime::parse_from_rfc3339(
                        item.get("created").unwrap().as_s().unwrap(),
                    )
                    .unwrap()
                    .into(),
                });
                if let Ok(plant) = plant {
                    return Ok(plant);
                }
            }
        }
        return Err("Could not find plant".to_string());
    }

    #[cfg(test)]
    async fn get_plants_by_user(&self, user_id: &str) -> Result<Vec<Plant>, String> {
        let result = self
            .client
            .query()
            .table_name(String::from("plants"))
            .index_name("useridIndex")
            .key_condition_expression(format!("user_id = :value"))
            .expression_attribute_values(":value", AttributeValue::S(user_id.to_string()))
            .send()
            .await;

        if let Ok(result) = result {
            println!("Result was OK");
            if let Some(items) = result.items {
                println!("Items was OK");
                let plants = panic::catch_unwind(|| {
                    items
                        .iter()
                        .map(|i| Plant {
                            id: i.get("id").unwrap().as_s().unwrap().clone(),
                            user_id: i.get("user_id").unwrap().as_s().unwrap().clone(),
                            name: i.get("name").unwrap().as_s().unwrap().clone(),
                            species: i.get("species").unwrap().as_s().unwrap().clone(),
                            created: DateTime::parse_from_rfc3339(
                                i.get("created").unwrap().as_s().unwrap(),
                            )
                            .unwrap()
                            .into(),
                        })
                        .collect()
                });
                if let Ok(plants) = plants {
                    println!("Plants was OK");
                    return Ok(plants);
                }
            }
        }

        return Err("Error fetching plants for user".to_string());
    }
}
