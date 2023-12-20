aws lambda update-function-code \
    --function-name  rust-s3-photo-optimization \
    --zip-file fileb://target/lambda/rust-s3-photo-optimization/bootstrap.zip