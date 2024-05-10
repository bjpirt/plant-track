#!/bin/bash
AWS_REGION=eu-west-2

awslocal dynamodb create-table --region $AWS_REGION --table-name users \
  --attribute-definitions '[{ "AttributeName": "id", "AttributeType": "S" },
    { "AttributeName": "email", "AttributeType": "S" }]' \
  --key-schema '[{ "AttributeName": "id", "KeyType": "HASH" }]' \
  --billing-mode PAY_PER_REQUEST \
  --global-secondary-indexes '[
    {
      "IndexName": "emailIndex",
      "Projection": { "ProjectionType": "ALL" },
      "KeySchema": [
        {"AttributeName": "email", "KeyType": "HASH"}
      ]
    }
  ]'
