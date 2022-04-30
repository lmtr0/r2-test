// use s3::{Bucket, Region, creds::Credentials};

use aws_sdk_s3::{Client, Endpoint, Credentials};
// use serde_json::json;

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

    println!("Creds:\nAKey: {}\nSKey: {}\nAID: {}", access_key, secret_key, account_id);

    // let region = Region::Custom {
    //     region: format!("earth"),
    //     endpoint: format!("https://r2.cloudflarestorage.com/higenku-suite"),
    // }; 

    let secret_access_key = secret_key.as_str();
    let access_key_id = access_key.as_str();


    let shared_config = aws_config::from_env()
        .endpoint_resolver(Endpoint::immutable("https://982723f0682411a61ffe979a716fd7e1.r2.cloudflarestorage.com".parse().expect("invalid URI")))
        .region("earth")
        .credentials_provider(Credentials::new(access_key_id, secret_access_key, None, None, "default"))
        .load().await;
    let res = Client::new(&shared_config)
        .list_objects_v2()
        .set_bucket(Some("higenku-suite".to_string()))
        .send().await;

    match res {
        Ok(e) => println!("\n\n\n{:#?}", e),
        Err(e) => println!("\n\n\n{:#?}", e),
    };
    // let credentials = Credentials::new(access_key, secret_key, None, None, Some("Hellow")).unwrap();

    // let mut bucket = Bucket::new(&account_id, region, credentials).unwrap();
    // let bucket = Bucket::new(&account_id, region, credentials).unwrap();
    // bucket.set_listobjects_v2();

    // let content = json!({
    //     "data": "hello there",
    //     "owner": "hello there"
    // });

    // println!("\n\n");
    // println!("Started");
    
    // let path = format!("/higenku.jpg");

    // let content = content.to_string();
    // let content = content.as_bytes();

    // let e = bucket.get_object(&path).await.unwrap();
    
    // println!("Put");
    // println!("{:?}", e.1);
    
    // let (head, _) =  bucket.head_object(&path).await.unwrap();
    // println!("Head");
    // println!("{:?}", head.e_tag);
    
    // let (data, _) = bucket.get_object(&path).await.unwrap();
    
    // println!("Get");
    // println!("{:?}", String::from_utf8(data));


    // let (data, code) = bucket.get_object("/higenku.jpg").await.unwrap();
    // println!("Get2");
    // println!("{}", code);
    // println!("{}", String::from_utf8(data).unwrap());

}
