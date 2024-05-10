use std::env;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{config::Builder, Client};

pub async fn get_dynamo_client() -> Client {
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
