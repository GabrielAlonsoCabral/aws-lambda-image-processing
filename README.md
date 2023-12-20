# Rust AWS Lambda (Image processing)
    This repo has as role track for images in specif bucket
    and insert in MongoDB his metadata.

    Also this project is being developed to support large files processing,
    which is his main needs behind this project. 

```
Developed by @GabrielAlonsoCabral
```

## Build
```
    sh build.sh
```

## Update Lambda
```
    sh update-lambda.sh
```

## Create lambda
```
    sh create-lambda.sh
```

## Usage
```
    sh test.sh
```


## refs
 - Useful to integrate lambda code with s3
    <br/>
    https://docs.aws.amazon.com/lambda/latest/dg/with-s3-example.html

 - Useful to startup "cargo lambda"  
    <br/>
    https://www.cargo-lambda.info/guide/getting-started.html