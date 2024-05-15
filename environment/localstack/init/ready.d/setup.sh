#!/bin/bash
AWS_REGION=eu-west-2

awslocal dynamodb create-table --region $AWS_REGION --table-name users \
  --attribute-definitions '[{ "AttributeName": "id", "AttributeType": "S" },
    { "AttributeName": "email", "AttributeType": "S" },
    { "AttributeName": "username", "AttributeType": "S" }]' \
  --key-schema '[{ "AttributeName": "id", "KeyType": "HASH" }]' \
  --billing-mode PAY_PER_REQUEST \
  --global-secondary-indexes '[
    {
      "IndexName": "emailIndex",
      "Projection": { "ProjectionType": "ALL" },
      "KeySchema": [
        {"AttributeName": "email", "KeyType": "HASH"}
      ]
    },
    {
      "IndexName": "usernameIndex",
      "Projection": { "ProjectionType": "ALL" },
      "KeySchema": [
        {"AttributeName": "username", "KeyType": "HASH"}
      ]
    }
  ]'

awslocal dynamodb create-table --region $AWS_REGION --table-name plants \
  --attribute-definitions '[{ "AttributeName": "id", "AttributeType": "S" },
    { "AttributeName": "user_id", "AttributeType": "S" }]' \
  --key-schema '[{ "AttributeName": "id", "KeyType": "HASH" }]' \
  --billing-mode PAY_PER_REQUEST \
  --global-secondary-indexes '[
    {
      "IndexName": "useridIndex",
      "Projection": { "ProjectionType": "ALL" },
      "KeySchema": [
        {"AttributeName": "user_id", "KeyType": "HASH"}
      ]
    }
  ]'
