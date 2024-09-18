//! This module contains helper functions for the main program.
//! 
//! The module includes functions to get environment variables and get data from the OpenWeatherMap API.
//! The module also includes a struct to hold the environment variables and a struct to hold the weather data.
pub mod helper_funcs{
    use futures::future::join_all;
    //use serde::de::Error;
    use std::error::Error;

    use crate::openweathermap::open_weather_data::ResponseItem;


    use dotenvy::dotenv;

    use log::info;

    #[derive(Debug)]
    pub struct EnvVars {
        pub host:String,
        pub user:String,
        pub password:String,
        pub database:String,
        pub api_key:String,
    }


    pub struct WeatherData {
        pub city: String,
        pub zip: String,
        pub temperature: f64,
        pub weather: String,
        pub humidity: String,
        pub wind_speed: f64,
    }
    
    impl WeatherData {
        pub fn new(city: String, zip: String, temperature: f64, weather: String, humidity: String, wind_speed: f64) -> Self {
            WeatherData { city, zip, temperature, weather, humidity, wind_speed }
        }
    }

    /// Get environment variables
    /// 
    /// This function retrieves the environment variables and places them in a struct for future use.
    pub fn get_env_vars() -> Result<EnvVars, Box<dyn Error> > {
        info!("Getting environment variables");
        dotenv().ok();
        let host = std::env::var("DB_HOST").expect("DB_HOST not set");
        let user = std::env::var("DB_USER").expect("DB_USER not set");
        let password = std::env::var("DB_PASS").expect("DB_PASS not set");
        let database = std::env::var("DB_NAME").expect("DB_NAME not set");
        let api_key = std::env::var("API_KEY").expect("API_KEY not set");
        Ok(EnvVars{host,user,password,database,api_key})
    }

    

    /// Get data from the OpenWeatherMap API
    /// 
    /// This function takes a zip code and an API key and returns the data from the OpenWeatherMap API.
    pub async fn get_zip_data(zip:String, api_key:String)->Result<ResponseItem, reqwest::Error> {
        info!("Requesting data from API");
        let url = format!("http://api.openweathermap.org/data/2.5/weather?zip={}&appid={}&units={}", zip, api_key, "imperial");
        let response = reqwest::get(url).await?;
        let resp_item: ResponseItem= ResponseItem{
            zip:zip.to_lowercase(), weather: response.json().await?,
        };
        Ok(resp_item)
    }

    /// Get data for multiple zip codes
    /// 
    /// This function takes a vector of zip codes and an API key and returns the data from the OpenWeatherMap API for each zip code.
    pub async fn get_target_data(target_list:Vec<String>, api_key:String)->Vec<Result<ResponseItem, reqwest::Error>> {
        let futures = target_list.into_iter().map(|x| {
            let api_key = api_key.clone(); // Clone api_key for each future
            async move {
                get_zip_data(x, api_key).await
            }
        });

        join_all(futures).await 
    }
}

