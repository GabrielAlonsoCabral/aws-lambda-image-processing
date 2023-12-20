

aws lambda create-function --function-name rust-s3-photo-optimization \
     --runtime provided.al2023 \
     --role arn:aws:iam::900909438974:role/lambda-s3-trigger-role \
     --handler rust.handler \
     --zip-file fileb://target/lambda/rust-s3-photo-optimization/bootstrap.zip