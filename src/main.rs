use s3::{creds::Credentials, region::Region, Bucket};

// use serde_json::json;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

const MESSAGE: &str = "{ \"name\": \"value\" }";

#[tokio::main]
async fn main() {
    // fern::Dispatch::new()
    //     .format(|out, message, record| {
    //         out.finish(format_args!(
    //             "[{}][{}] {}",
    //             record.target(),
    //             record.level(),
    //             message
    //         ))
    //     })
    //     .level(log::LevelFilter::Debug)
    //     .chain(std::io::stdout())
    //     .apply().unwrap();
    dotenv::dotenv().expect("Failed to load evironment");

    let secret_key = std::env::var("SECRET_KEY").expect("Consider putting CF_SHA256_API_KEY in your .env file");
    let access_key = std::env::var("ACCESS_KEY").expect("Consider putting CF_API_KEY_ID in your .env file");
    let account_id = std::env::var("CF_ACCOUNT_ID").expect("Consider putting CF_ACCOUNT_ID in your .env file");

    let endpoint = format!("https://{}.r2.cloudflarestorage.com", account_id);

    let credentials = Credentials::new(
        Some(&access_key),
        Some(&secret_key),
        None,
        None,
        None
    ).expect("Failed to create credentials");

    println!("Creds: {:#?}", credentials);
    println!("Endpoint: {:#?}", endpoint);



    let backend = Storage {
        region: Region::Custom {
            region: "auto".into(),
            endpoint,
        },
        credentials,
        bucket: "suite".to_string(),
    };



    let mut bucket = Bucket::new(&backend.bucket, backend.region, backend.credentials).expect("Failed to create bucket")
        .with_path_style();
    bucket.set_listobjects_v2();
    
    
    // PUT
    println!("====================PUT");
    bucket.add_header("x-amz-meta-key", "value");
    
    for i in 0..10 {
        let path = format!("/dir/file_{}", i);
        bucket.add_header("x-amz-meta-path", &path);

        let (message, _) = bucket.put_object_with_content_type(path, MESSAGE.as_bytes(), "application/json").await.expect("Failed to get object");
        println!("MESSAGE: {}", String::from_utf8(message).unwrap());
        
    }
    
    // LIST
    println!("====================LIST");
    let e = bucket.list("".to_string(), None).await.expect("Couldn't listv2");
    for i in e {
        println!("-------------====================-------------");
        println!("{:#?}", i.contents);
    }

    for i in 0..10 {
        let path = format!("/dir/file_{}", i);

        // HEAD
        println!("====================HEAD");
        let (data, _) = bucket.head_object(&path).await.expect("Failed to head object");
        println!("META {:?}", data.metadata);
        
        // // GET
        // println!("====================GET");
        // let (data, _) = bucket.get_object(&path).await.expect("Failed to get object");
        // let string = String::from_utf8(data).unwrap();
        // println!("DATA: {}", string);
    
        // Delete
        // println!("====================DELETE");
        let (_, _) = bucket.delete_object(&path).await.expect("Failed to get object");
    }
}
