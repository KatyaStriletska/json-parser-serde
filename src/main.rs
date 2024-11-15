use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use uuid::Uuid;
use url::Url;
use serde_yaml::to_string as to_yaml;
use toml::to_string as to_toml;


#[derive(Debug, Deserialize, Serialize)]
struct PublicTariff
{
    id: u32,
    price: u32, 
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String, 
}

#[derive(Debug, Deserialize, Serialize)]
struct PrivateTariff
{
    client_price: u32, 
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String, 
}

#[derive(Debug, Deserialize, Serialize)]
struct Stream 
{
    user_id: Uuid,  // Uuid
    is_private: bool,
    settings: u32,
    shard_url: Url,  // url
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff, 
}

#[derive(Debug, Deserialize, Serialize)]
struct Gift
{
    id: u32, 
    price: u32,
    description: String, 
}

#[derive(Debug, Deserialize, Serialize)]
struct Debug
{
    #[serde(with = "humantime_serde")]
    duration: Duration, 
    at: DateTime<Utc>, // Date
}

#[derive(Debug, Deserialize, Serialize)]
struct Request
{
    #[serde(rename = "type")]
    request_type: RequestType, 
    stream: Stream, 
    gifts: Vec<Gift>,
    debug: Debug, 
}
#[derive(Debug, Deserialize, Serialize)]
enum RequestType{
    #[serde(rename = "success")]
    Success, 
    #[serde(rename = "failure")]
    Failure

}

fn main() {
    let mut file = File:: open("request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json:: from_str(&json_str).unwrap();
    let yaml_str = to_yaml(&request).unwrap();
    println!("YAML : \n{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    println!("TOML : \n{}", toml_str);

    // println!("{json_str}");




    // can dalete/ we just start with it
    // let user = User {
    //     name: String::from("Katyha"), 
    //     email: "k.@blabka.com".to_string(), 
    //     birthdate: "27/02/05".to_string(), 
    // };

    // let json:String = serde_json::to_string(&user).expect( "Serialize error");
    // println!("{}", json);

    // let deserialized_user: User = serde_json::from_str(&json).expect("");
    // println!("{:?}", deserialized_user);
}

