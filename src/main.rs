
 mod utils;
 mod openweathermap;
 mod config;
 use std::env;

 use config::config::{load_config, randomize_target_list, WeatherPullConf};
 
 use openweathermap::open_weather_data::ResponseItem;
use reqwest::Error;
use sea_orm::DatabaseConnection;
//use futures::future::join_all;
use crate::openweathermap::open_weather_data::{Weather,APIRequestParams,WorkList};

use sqlx::{MySqlPool, mysql::MySqlQueryResult};
use sea_orm::Database;
use sea_orm::ActiveValue::{Set, NotSet, Unchanged};
use sea_orm::ActiveModelTrait;

mod models;

use models::weather_str::WeatherData;
use models::weather_data;

#[derive(Debug)]
enum AppError {
    //DatabaseError(sqlx::Error),
    HttpRequestError(reqwest::Error),
    DatabaseError(sea_orm::DbErr),
    // Other errors...
}

// impl From<sqlx::Error> for AppError {
//     fn from(err: sqlx::Error) -> Self {
//         AppError::DatabaseError(err)
//     }
// }

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpRequestError(err)
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err)
    }
    
}


 #[tokio::main]
async fn main() -> Result<(), AppError> {

    let current_dir = env::current_dir().unwrap();
    //println!("Current directory: {}", current_dir.display());

    let pull_conf :WeatherPullConf = load_config("weather-pull-conf.json".to_string());

    let targets = randomize_target_list(pull_conf);


    let mysql_host = env::var("DB_HOST").expect("DB_HOST not set");
    let mysql_user = env::var("DB_USER").expect("MYSQL_USER not set");
    let mysql_password = env::var("DB_PASS").expect("DB_PASS not set");
    let mysql_database = env::var("DB_NAME").expect("DB_NAME not set");
    let api_key: String = env::var("API_KEY").expect("API_KEY not set");

    // Construct the MySQL connection URL
    let database_url = format!("mysql://{}:{}@{}/{}", mysql_user, mysql_password, mysql_host, mysql_database);
    // mysql://admin:password@weather-results.cz6gyqqeadbq.us-east-2.rds.amazonaws.com/weather


    // collection of WeatherData structs
    // let mut data_list: Vec<WeatherData> = Vec::new();


    // Create a connection pool
    //let pool = MySqlPool::connect(&database_url).await?;
    let db: DatabaseConnection = Database::connect(&database_url).await?;

    println!("Starting API call");
    let requests: Vec<WorkList> = targets.iter().map(|x|
        {
            let request_params = APIRequestParams{
                zip: x.to_string(),
                api_key:api_key.clone(),
                unit:"imperial".to_string()

                };
            let url = format!(
                    "http://api.openweathermap.org/data/2.5/weather?zip={}&appid={}&units={}",
                    request_params.zip, request_params.api_key, request_params.unit
                );
            WorkList{zip: request_params.zip, url: url}         
    } 
    ).collect();


    let responses: Vec<Result<ResponseItem, Error>> = futures::future::join_all(
        requests.into_iter().map(|x| {
            async move {
                println!("Requesting: {}", x.url);
                let response = reqwest::get(x.url).await?;
                let resp_item: ResponseItem= ResponseItem{
                    zip:x.zip.to_lowercase(), weather: response.json().await?,
                };
                println!("Response: {:?}", resp_item);
                Ok(resp_item)
            }
        })
    ).await;
    print!("Finished API call");

    for response in responses {
        match response {
            Ok(ri) => {
                println!("City:{}", ri.weather.name);
                println!("Zip:{}", ri.zip);
                println!("Temperature: {}", ri.weather.main.temp);
                println!("Weather: {}", ri.weather.weather[0].description);
                println!("Humidity: {}%", ri.weather.main.humidity);
                println!("Wind Speed: {} m/s", ri.weather.wind.speed);
                println!();


                let data = WeatherData::new(
                    ri.weather.name,
                    ri.zip,
                    ri.weather.main.temp,
                    ri.weather.weather[0].description.clone(),
                    format!("{}%", ri.weather.main.humidity),
                    ri.weather.wind.speed
                );

                insert_weather_data(&db, &data).await?;
                
                // data_list.push(data);
            },
            Err(e) => println!("Error: {}", e),
        }
    }


    // for data in data_list {
    //     insert_weather_data(&db, &data).await?;
    // }

    
    


    Ok(())
}




async fn insert_weather_data(db: &DatabaseConnection, data: &WeatherData) -> Result<(), AppError>{
    let current_weather: weather_data::ActiveModel = weather_data::ActiveModel {
        city: Set(data.city.clone()),
        zip: Set(data.zip.clone()),
        temperature: Set(data.temperature),
        weather: Set(data.weather.clone()),
        humidity: Set(data.humidity.clone()),
        wind_speed: Set(data.wind_speed),
        ..Default::default()
    };
    
    println!("Inserting data into the database...");
    current_weather.insert(db).await?;
    println!("Data inserted successfully");


    Ok(())
}