use std::panic;

use aws_sdk_dynamodb::{operation::query::builders::QueryFluentBuilder, types::AttributeValue};
use chrono::DateTime;

use crate::types::user::User;

use super::get_dynamo_client::get_dynamo_client;

pub async fn get_user(username: String) -> Result<User, String> {
    let client = get_dynamo_client().await;

    let request: QueryFluentBuilder = client
        .query()
        .table_name(String::from("users"))
        .index_name("usernameIndex")
        .key_condition_expression("username = :username")
        .expression_attribute_values(":username", AttributeValue::S(username));

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
        Err(e) => {
            println!("{e}");
            Err(String::from("Could not find user"))
        }
    }
}
