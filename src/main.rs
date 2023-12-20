use aws_config::{BehaviorVersion, SdkConfig};
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<S3Event>, s3_client: &Client) -> Result<(), Error> {
    if event.payload.records.len() == 0 {
        tracing::info!("Empty S3 event received");
    }

    let bucket = event.payload.records[0]
        .s3
        .bucket
        .name
        .as_ref()
        .expect("Bucket name to exist");

    let key = event.payload.records[0]
        .s3
        .object
        .key
        .as_ref()
        .expect("Object key to exist");

    println!("Bucket: {:?} key: {:?}", bucket, key);

    let s3_head = s3_client.head_object().bucket(bucket).key(key).send().await;

    match s3_head {
        Ok(_) => tracing::info!("S3 Obtained head successfully!"),
        Err(_) => tracing::info!("Failure with S3 head_object"),
    };

    println!("Metadata {:?}", s3_head.as_ref().unwrap().metadata());
    let metadata = s3_head.as_ref().unwrap().metadata().unwrap();

    let (longitude, latitude, obj_create_date, project_id) = (
        metadata.get("gpslongitude").expect("Missing gpslongitude"),
        metadata.get("gpslatitude").expect("Missing gpslatitude"),
        metadata.get("createdate").expect("Missing createdate"),
        metadata.get("projectid").expect("Missing projectid"),
    );

    println!(
        "Longitude {:?}; Latitude {:?}; CreateDate:{:?}; ProjectId:{:}",
        longitude, latitude, obj_create_date, project_id
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: SdkConfig = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client: Client = Client::new(&config);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let res = run(service_fn(|request: LambdaEvent<S3Event>| {
        function_handler(request, &s3_client)
    }))
    .await;

    res
}
