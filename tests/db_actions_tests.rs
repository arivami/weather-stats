//! Tests for the db_actions module

#[cfg(test)]
use sea_orm::{
    entity::prelude::*, entity::*,
    DatabaseBackend, MockDatabase, MockExecResult,
};

use dotenvy::dotenv;
use weather_stats::models::weather_data;
use weather_stats::utils::helper_funcs::EnvVars;
use weather_stats::db_actions::db_actions::*;
use chrono::Utc;


/// Set up environment variables
/// 
/// A helper function to set up fake environment variables for testing.
fn setup() ->  EnvVars {
    let result = EnvVars {
        host: "host".to_string(),
        user: "user".to_string(),
        password: "password".to_string(),
        database: "database".to_string(),
        api_key: "api_key".to_string(),
    };
    result
}


#[test]
fn test_get_db_url() {
    let env_vars = setup();
    let db_url = get_db_url(env_vars);
    assert_eq!(db_url, "mysql://user:password@host/database");
}

#[tokio::test]
async fn test_connect_to_db() {
    
    dotenv().ok();
    let host = std::env::var("DB_HOST").expect("DB_HOST not set");
    let user = std::env::var("DB_USER").expect("DB_USER not set");
    let password = std::env::var("DB_PASS").expect("DB_PASS not set");
    let database = std::env::var("DB_NAME").expect("DB_NAME not set");
    let env_vars = EnvVars {host,user,password,database, api_key:"".to_string()};


    let db_url = format!("mysql://{}:{}@{}/{}", env_vars.user, env_vars.password, env_vars.host, env_vars.database);
    let db = connect_to_db(db_url).await.unwrap();

     
    assert_eq!(db.ping().await.is_ok(), true);

}



/// Test inserting data into database table
/// 
/// This test creates a mock database connection and inserts a row into the weather_data table.
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