use crate::db::dynamo::get_dynamo_client::get_dynamo_client;
use aws_sdk_dynamodb::{
    operation::put_item::builders::PutItemFluentBuilder, types::AttributeValue, Client,
};

use crate::types::user::User;

pub async fn create_user(user: &User) -> Result<(), String> {
    let client: Client = get_dynamo_client().await;

    let request: PutItemFluentBuilder = client
        .put_item()
        .table_name(String::from("users"))
        .item("id", AttributeValue::S(user.id.clone()))
        .item("username", AttributeValue::S(user.username.clone()))
        .item("name", AttributeValue::S(user.name.clone()))
        .item("email", AttributeValue::S(user.email.clone()))
        .item("password", AttributeValue::S(user.password.clone()))
        .item("created", AttributeValue::S(user.created.to_string()))
        .item("validated", AttributeValue::Bool(user.validated))
        .item("contact", AttributeValue::Bool(user.contact));

    let res = request.send().await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{e}");
            Err(String::from("Error creating user"))
        }
    }
}
