
// Internal
use weather_stats::config::config::*;

use weather_stats::models::weather_str::WeatherData;

use weather_stats::utils::helper_funcs::*;

use weather_stats::error_handling::AppError;

use weather_stats::db_actions::db_actions::*;




// External

use std::env;
use dotenvy::dotenv;






 #[tokio::main]
async fn main() -> Result<(), AppError> {

    

    let pull_conf :WeatherPullConf = load_config("/workspace/src/test-data/weather-pull-conf.json".to_string());

    let targets = randomize_target_list(pull_conf);


    // this line only needs to be here if using separate .env file
    dotenv().ok();

    // get all environment variables
    let env_vars = get_env_vars();
    let api_key = env_vars.api_key.clone();
    

    // construct the database url
    let db_url = get_db_url(env_vars);


    // connect to the database
    let db = connect_to_db(db_url).await;


  
    let target_data = get_target_data(targets, api_key).await;



    

    for response in target_data {
        match response {
            Ok(ri) => {
                println!("City:{}, Zip:{}, Temperature: {}", ri.weather.name, ri.zip, ri.weather.main.temp);
                println!("Weather: {}, Humidity: {}%, Wind Speed: {} m/s", ri.weather.weather[0].description, 
                    ri.weather.main.humidity, ri.weather.wind.speed);
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
                
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
