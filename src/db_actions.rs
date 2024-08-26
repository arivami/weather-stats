

pub mod db_actions {

    use sea_orm::DatabaseConnection;
    use sea_orm::ActiveValue::Set;
    use sea_orm::ActiveModelTrait;

    
    use crate::models::weather_str::WeatherData;
    use crate::models::weather_data;

    use crate::error_handling::AppError;

 



    pub async fn insert_weather_data(db: &DatabaseConnection, data: &WeatherData) -> Result<(), AppError>{
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


}



