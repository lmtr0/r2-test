use s3::{Bucket, Region, creds::Credentials};
use serde_json::json;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load evironment");

    let secret_key = std::env::var("CF_SHA256_API_KEY").expect("Consider putting CF_SHA256_API_KEY in your .env file");
    let access_key = std::env::var("CF_API_KEY_ID").expect("Consider putting CF_API_KEY_ID in your .env file");
    let account_id = std::env::var("CF_ACCOUNT_ID").expect("Consider putting CF_ACCOUNT_ID in your .env file");

    println!("Creds:\nAKey: {}\nSKey: {}\nAID: {}", access_key, secret_key, account_id);

    let region = Region::Custom {
        region: format!(""),
        endpoint: format!("https://{}.r2.cloudflarestorage.com", account_id),
    }; 

    let secret_key = Some(secret_key.as_str());
    let access_key = Some(access_key.as_str());

    let credentials = Credentials::new(access_key, secret_key, None, None, Some("Hellow")).unwrap();

    let mut bucket = Bucket::new("higenku-suite", region, credentials).unwrap();
    bucket.set_listobjects_v2();

    let content = json!({
        "data": "hello there",
        "owner": "hello there"
    });

    println!("Started");
    
    let path = format!("/mycoolfile.json");

    let content = content.to_string();
    let content = content.as_bytes();

    let e = bucket.put_object_with_content_type(&path, content, "application/json").await.unwrap();
    
    println!("Put:");
    println!("{:?}",String::from_utf8(e.0));
    
    // let (head, _) =  bucket.head_object(&path).await.unwrap();
    // println!("Head");
    // println!("{:?}", head.e_tag);
    
    let (data, _) = bucket.get_object(&path).await.unwrap();
    
    println!("Get");
    println!("{:?}", String::from_utf8(data));


    let (data, code) = bucket.get_object("/higenku.jpg").await.unwrap();
    println!("Get2");
    println!("{}", code);
    println!("{}", String::from_utf8(data).unwrap());

}
