use std::env;

use crate::db::dynamo::get_dynamo_client::get_dynamo_client;
use aws_sdk_dynamodb::{
    operation::query::builders::QueryFluentBuilder, types::AttributeValue, Client,
};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

#[derive(Serialize)]
pub struct Claims {
    pub user_id: String,
    exp: usize,
}

fn generate_auth_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        user_id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub async fn login(email: String, password: String) -> Result<String, String> {
    let client: Client = get_dynamo_client().await;

    let request: QueryFluentBuilder = client
        .query()
        .table_name(String::from("users"))
        .index_name("emailIndex")
        .key_condition_expression("email = :email")
        .expression_attribute_values(":email", AttributeValue::S(email));

    let res = request.send().await;

    match res {
        Ok(results) => {
            if let Some(items) = results.items {
                println!("Found user");
                if let Some(user) = items.get(0) {
                    println!("Extracted user");
                    let saved_password = user.get("password");
                    let validated = user.get("validated");
                    let userid: Option<&AttributeValue> = user.get("id");

                    if let (Some(saved_password), Some(validated), Some(userid)) =
                        (saved_password, validated, userid)
                    {
                        let saved_password_value = saved_password.as_s();
                        let validated_value = validated.as_bool();
                        let validated_userid = userid.as_s();

                        if let (
                            Ok(saved_password_value),
                            Ok(validated_value),
                            Ok(validated_userid),
                        ) = (saved_password_value, validated_value, validated_userid)
                        {
                            println!("Validated: {validated_value} Pass: {saved_password_value}");
                            if *validated_value && *saved_password_value == password {
                                if let Ok(auth_token) =
                                    generate_auth_token(validated_userid.clone())
                                {
                                    return Ok(auth_token);
                                }
                            }
                        }
                    }
                }
            }

            Err(String::from("Error logging in"))
        }
        Err(e) => {
            println!("{e}");
            Err(String::from("Error logging in"))
        }
    }
}
