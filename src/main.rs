


/* use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    
    run(service_fn(function_handler)).await
}
 */

 mod utils;
 mod openweathermap;
 mod config;
 use std::env;

 use config::config::{load_config, randomize_target_list, WeatherPullConf};
 
 use openweathermap::open_weather_data::ResponseItem;
use reqwest::Error;
//use futures::future::join_all;
use crate::openweathermap::open_weather_data::{Weather,APIRequestParams,WorkList};

use sqlx::{MySqlPool, mysql::MySqlQueryResult};

#[derive(Debug)]
struct WeatherData {
    city: String,
    zip: String,
    temperature: f64,
    weather: String,
    humidity: String,
    wind_speed: f64,
}

#[derive(Debug)]
enum AppError {
    DatabaseError(sqlx::Error),
    HttpRequestError(reqwest::Error),
    // Other errors...
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpRequestError(err)
    }
}


 #[tokio::main]
async fn main() -> Result<(), AppError> {

    let current_dir = env::current_dir().unwrap();
    //println!("Current directory: {}", current_dir.display());

    let pull_conf :WeatherPullConf = load_config("src/test-data/weather-pull-conf.json".to_string());

    let targets = randomize_target_list(pull_conf);

    let database_url = "mysql://root:password@mysql-container/my_database";

    // collection of WeatherData structs
    let mut data_list: Vec<WeatherData> = Vec::new();


    // Create a connection pool
    let pool = MySqlPool::connect(database_url).await?;

    let requests: Vec<WorkList> = targets.iter().map(|x|
        {
            let request_params = APIRequestParams{
                zip: x.to_string(),
                api_key:"5ffce5e80bb83c6f974df8aeb9542960".to_string(),
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
                let response = reqwest::get(x.url).await?;
                let resp_item: ResponseItem= ResponseItem{
                    zip:x.zip.to_lowercase(), weather: response.json().await?,
                };
                Ok(resp_item)
            }
        })
    ).await;

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

                let data = WeatherData {
                    zip: ri.zip,
                    city: ri.weather.name,
                    temperature: ri.weather.main.temp,
                    weather: ri.weather.weather[0].description.clone(),
                    humidity: format!("{}%", ri.weather.main.humidity),
                    wind_speed: ri.weather.wind.speed,
                };
                data_list.push(data);
            },
            Err(e) => println!("Error: {}", e),
        }
    }


    for data in data_list {
        
        
        create_table_if_not_exists(&pool).await?;
        insert_weather_data(&pool, &data).await?;
    }

    
    


    Ok(())
}



async fn create_table_if_not_exists(pool: &MySqlPool) -> Result<MySqlQueryResult, AppError> {
    // Check if table exists
    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = ?)",
    )
    .bind("weather_data")
    .fetch_one(pool)
    .await?;

    if !table_exists {
        // Create the table
        sqlx::query(
            r#"
            CREATE TABLE weather_data (
                id BIGINT AUTO_INCREMENT PRIMARY KEY,
                zip VARCHAR(255) NOT NULL,
                city VARCHAR(255) NOT NULL,
                temperature FLOAT NOT NULL,
                weather VARCHAR(255) NOT NULL,
                humidity VARCHAR(255) NOT NULL,
                wind_speed FLOAT NOT NULL,
                measurement_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;
    }

    Ok(MySqlQueryResult::default())
}

async fn insert_weather_data(pool: &MySqlPool, data: &WeatherData) -> Result<MySqlQueryResult, AppError> {
    // Insert data into the corresponding table
    sqlx::query(
        &format!(
            "INSERT INTO weather_data (zip, city, temperature, weather, humidity, wind_speed) VALUES (?, ?, ?, ?, ?, ?)"
        )
    )
    .bind(&data.zip)
    .bind(&data.city)
    .bind(data.temperature)
    .bind(&data.weather)
    .bind(&data.humidity)
    .bind(data.wind_speed)
    .execute(pool)
    .await?;

    Ok(MySqlQueryResult::default())
}