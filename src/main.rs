
use futures::TryFutureExt;
// Internal
use weather_stats::config::config::*;

use weather_stats::models::weather_str::WeatherData;

use weather_stats::utils::helper_funcs::*;

use weather_stats::error_handling::AppError;

use weather_stats::db_actions::db_actions::*;




// External
use dotenvy::dotenv;

use flexi_logger::{Duplicate, Logger, WriteMode, FileSpec};
use log::{info, warn, error, debug};






 #[tokio::main]
async fn main() {


    Logger::try_with_str("debug")
    .unwrap()
    .log_to_file(FileSpec::default()
        .suppress_timestamp()
        .directory("logs")
    ) // Set up default file logging
    .duplicate_to_stderr(Duplicate::All) // Duplicate all logs to stderr
    .write_mode(WriteMode::BufferAndFlush) // Buffer logs and flush them periodically
    .start()
    .unwrap();

     



    // this line only needs to be here if using separate .env file
    //dotenv().ok();

    // get all environment variables
    let env_vars = get_env_vars().unwrap_or_else(|err| {
        error!("Failed to get environment variables: {}", err);
        std::process::exit(1);
    });


    let api_key = env_vars.api_key.clone();
    

    // construct the database url
    let db_url = get_db_url(env_vars);


    // connect to the database
    let db = connect_to_db(db_url).await.unwrap_or_else(|err| {
            error!("Failed to connect to the database: {}", err);
            std::process::exit(1);
        });

    let targets = load_config_db(&db).await.unwrap_or_else(|err| {
        error!("Failed to load target zips from the database: {}", err);
        std::process::exit(1);
    });


  
    //let target_data = get_target_data(targets, api_key).await;
    let target_data = get_target_data(targets.zips, api_key).await;



    

    for response in target_data {
        match response {
            Ok(ri) => {
                debug!("City:{}, Zip:{}, Temperature: {}", ri.weather.name, ri.zip, ri.weather.main.temp);
                debug!("Weather: {}, Humidity: {}%, Wind Speed: {} m/s", ri.weather.weather[0].description, 
                    ri.weather.main.humidity, ri.weather.wind.speed);


                let data = WeatherData::new(
                    ri.weather.name,
                    ri.zip,
                    ri.weather.main.temp,
                    ri.weather.weather[0].description.clone(),
                    format!("{}%", ri.weather.main.humidity),
                    ri.weather.wind.speed
                );

                insert_weather_data(&db, &data).await.unwrap_or_else(|err| {
                    error!("Failed to insert data into the database: {}", err);
                    info!("Proceeding to next record");
                });
                
            },
            Err(e) => error!("Failed to get data from the API: {}", e),
        }
    }
    
}
