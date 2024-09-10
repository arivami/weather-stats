//! This module contains functions that interact with the database.
//! 
//! The module includes functions to connect to the database, insert data into the database, and construct the database URL.
pub mod db_actions {

    use sea_orm::DatabaseConnection;
    use sea_orm::ActiveValue::Set;
    use sea_orm::ActiveModelTrait;

    
    use crate::utils::helper_funcs::WeatherData;
    use crate::models::weather_data;

    use crate::error_handling::AppError;
    use crate::utils::helper_funcs::EnvVars;

    use log::info;

 
    /// Construct the database URL
    /// 
    /// This function takes the environment variables and constructs the database URL.
    pub fn get_db_url(env_vars:EnvVars)->String {
        format!("mysql://{}:{}@{}/{}", env_vars.user, env_vars.password, env_vars.host, env_vars.database)
    }

    /// Connect to the database
    /// 
    /// This function connects to the database using the database URL and a SeaOrm DatabaseConnection.
    pub async fn connect_to_db(url:String)->Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
        info!("Connecting to the database");
        sea_orm::Database::connect(&url).await
    }


    /// Insert weather data into the database
    /// 
    /// This function takes a SeaOrm DatabaseConnection and a WeatherData struct and inserts the data into the database.
    pub async fn insert_weather_data(db: &DatabaseConnection, data: &WeatherData) -> Result<(), sea_orm::DbErr>{
        let current_weather: weather_data::ActiveModel = weather_data::ActiveModel {
            city: Set(data.city.clone()),
            zip: Set(data.zip.clone()),
            temperature: Set(data.temperature),
            weather: Set(data.weather.clone()),
            humidity: Set(data.humidity.clone()),
            wind_speed: Set(data.wind_speed),
            ..Default::default()
        };
        
        info!("Inserting data into the database...");
        current_weather.insert(db).await?;
        info!("Data inserted successfully");
    
    
        Ok(())
    }


}



