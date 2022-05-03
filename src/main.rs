use s3::{creds::Credentials, region::Region, Bucket};

// use serde_json::json;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

const MESSAGE: &str = "Hi there";

#[tokio::main]
async fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply().unwrap();
    dotenv::dotenv().expect("Failed to load evironment");

    let secret_key = std::env::var("CF_SHA256_API_KEY").expect("Consider putting CF_SHA256_API_KEY in your .env file");
    let access_key = std::env::var("CF_API_KEY_ID").expect("Consider putting CF_API_KEY_ID in your .env file");
    let account_id = std::env::var("CF_ACCOUNT_ID").expect("Consider putting CF_ACCOUNT_ID in your .env file");

    let endpoint = format!("https://r2.cloudflarestorage.com/suite");

    let credentials = Credentials::new(
        Some(&access_key),
        Some(&secret_key),
        None,
        None,
        None
    ).expect("Failed to create credentials");


    let backend = Storage {
        name: "r2".into(),
        region: Region::Custom {
            region: "".into(),
            endpoint,
        },
        credentials,
        bucket: account_id,
        location_supported: false,
    };

    let bucket = Bucket::new(&backend.bucket, backend.region, backend.credentials).expect("Failed to create bucket");


    if backend.location_supported {
        // Get bucket location
        println!("{:?}", bucket.location().await);
    }

    let (message, code) = bucket.put_object("test_file", MESSAGE.as_bytes()).await.expect("Failed to get object");
    // println!("{}", bucket.presign_get("test_file", 604801, None)?);
    println!("{}", String::from_utf8(message).unwrap());
    assert_eq!(200, code);


    // Get the "test_file" contents and make sure that the returned message
    // matches what we sent.
    let (data, code) = bucket.get_object("test_file").await.expect("Failed to get object");
    let string = String::from_utf8(data).unwrap();
    // println!("{}", string);
    assert_eq!(200, code);
    assert_eq!(MESSAGE, string);

}
