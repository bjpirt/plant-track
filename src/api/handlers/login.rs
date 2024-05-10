use crate::db::dynamo::get_dynamo_client::get_dynamo_client;
use aws_sdk_dynamodb::{
    operation::query::builders::QueryFluentBuilder, types::AttributeValue, Client,
};

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

                    if let (Some(saved_password), Some(validated)) = (saved_password, validated) {
                        let saved_password_value = saved_password.as_s();
                        let validated_value = validated.as_bool();

                        if let (Ok(saved_password_value), Ok(validated_value)) =
                            (saved_password_value, validated_value)
                        {
                            println!("Validated: {validated_value} Pass: {saved_password_value}");
                            if *validated_value && *saved_password_value == password {
                                return Ok("LOGIN_TOKEN".to_string());
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
