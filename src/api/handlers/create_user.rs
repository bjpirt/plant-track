use std::env;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    config::Builder, operation::put_item::builders::PutItemFluentBuilder, types::AttributeValue,
    Client, Error,
};

use crate::types::user::User;

async fn get_dynamo_client() -> Client {
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

    Client::from_conf(builder.build())
}

async fn add_item(user: &User) -> Result<(), Error> {
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

    request.send().await?;

    Ok(())
}

pub async fn create_user(user: &User) -> Result<(), String> {
    let res: Result<(), Error> = add_item(&user).await;
    match res {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e}");
            Err(String::from("Error creating user"))
        }
    }
}
