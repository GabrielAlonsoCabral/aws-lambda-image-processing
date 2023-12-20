aws lambda create-function --function-name rust-s3-photo-optimization \
     --runtime provided.al2023 \
     --role arn:aws:iam::900909438974:role/cargo-lambda-role-8a8a5253-93f8-4052-a9c8-68aa63fc82f6 \
     --handler rust.handler \
     --zip-file fileb://target/lambda/rust-s3-photo-optimization/bootstrap.zip