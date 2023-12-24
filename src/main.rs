use std::str::FromStr;

use aws_config::{BehaviorVersion, SdkConfig};
use aws_lambda_events::{event::s3::S3Event, s3::S3Entity};
use aws_sdk_s3::Client as SdkS3Client;
use dotenv::dotenv;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    results::InsertOneResult,
    Client, Collection, Database,
};

use lib::{get_env, Coordinates, Photo};

async fn function_handler(
    event: LambdaEvent<S3Event>,
    s3_client: &SdkS3Client,
    db: &Database,
) -> Result<(), Error> {
    if event.payload.records.len() == 0 {
        tracing::info!("Empty S3 event received");
    }

    let record: S3Entity = event.payload.records[0].s3.clone();
    let bucket: String = record.bucket.name.expect("Expect Bucket name to exist");
    let key: String = record.object.key.expect("Expect key to exist").to_string();

    let bucket_clone: String = bucket.clone();
    let key_clone: String = key.clone();

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

    let (longitude_float, latitude_float) = (
        longitude.parse::<f32>().unwrap(),
        latitude.parse::<f32>().unwrap(),
    );

    let collection: Collection<Photo> = db.collection("photos");

    let document_uri: String = format!("https://{}.s3.amazonaws.com/{}", bucket_clone, key_clone);

    let photo: Photo = Photo {
        uri: document_uri,
        shot_on: obj_create_date.parse::<i64>().unwrap(),
        project: ObjectId::from_str(&project_id).unwrap(),
        _id: ObjectId::new(),
        location: Coordinates {
            _id: ObjectId::new(),
            kind: "Point".to_string(),
            coordinates: vec![longitude_float, latitude_float],
        },
    };

    let document_created: InsertOneResult = collection.insert_one(photo, None).await?;

    println!("Document created {:?}", document_created);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: SdkConfig = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client: SdkS3Client = SdkS3Client::new(&config);

    dotenv().ok();

    let mut client_options: ClientOptions = ClientOptions::parse(get_env("DB_URI")).await?;

    client_options.app_name = Some("Rust MongoDB Collection Backup".to_string());

    let client: Client = Client::with_options(client_options)?;

    println!("Trying to connect");

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Connected successfully.");

    let db: Database = client.database(&get_env("DB_NAME"));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let res = run(service_fn(|request: LambdaEvent<S3Event>| {
        function_handler(request, &s3_client, &db)
    }))
    .await;

    res
}
