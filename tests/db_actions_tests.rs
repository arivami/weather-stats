
#[cfg(test)]
use sea_orm::{
    entity::prelude::*, entity::*,
    DatabaseBackend, MockDatabase, MockExecResult,
};

use weather_stats::models::weather_data;
use chrono::Utc;




#[tokio::test]
async fn test_insert_weather_data() {

    let today = Utc::now().date_naive();
    let noon = today.and_hms_opt(12, 0, 0).unwrap();
    
    let db = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results([
                [weather_data::Model {
                    id:15, 
                    zip:"95124".to_string(), 
                    city:"San Jose".to_string(), 
                    temperature:70.0, 
                    weather:"good".to_string(), 
                    humidity:"high".to_string(), 
                    wind_speed:1.0,
                    measurement_time: noon,
                    
                }],
                [weather_data::Model {
                    id:16, 
                    zip:"95014".to_string(), 
                    city:"Cupertino".to_string(), 
                    temperature:70.0, 
                    weather:"good".to_string(), 
                    humidity:"high".to_string(), 
                    wind_speed:1.0,
                    measurement_time: noon,
                }],
            ])
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 15,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 16,
                    rows_affected: 1,
                },
            ])
            .into_connection();

        let my_weather = weather_data::ActiveModel {
            city: Set("San Jose".to_string()),
            zip: Set("95124".to_string()),
            temperature: Set(70.0),
            weather: Set("good".to_string()),
            humidity: Set("high".to_string()),
            wind_speed: Set(1.0),
            ..Default::default()
        };

        assert_eq!(
            my_weather.clone().insert(&db).await,
            Ok(weather_data::Model {
                id: 15,
                city: "San Jose".to_string(),
                zip: "95124".to_string(),
                temperature: 70.0,
                weather: "good".to_string(),
                humidity: "high".to_string(),
                wind_speed: 1.0,
                measurement_time: noon,
               
            })
        );

}