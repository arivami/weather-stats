//! Tests for the helper_funcs module
use weather_stats::utils::helper_funcs::*;
use weather_stats::openweathermap::open_weather_data::*;

use dotenvy::dotenv;



/// Test get_env_vars function
/// 
/// This test checks that the get_env_vars function returns a struct with correct information about environment variables.
#[test]
fn test_get_env_vars() {
    dotenv().ok();
    let host = std::env::var("DB_HOST").expect("DB_HOST not set");
    let user = std::env::var("DB_USER").expect("DB_USER not set");
    let password = std::env::var("DB_PASS").expect("DB_PASS not set");
    let database = std::env::var("DB_NAME").expect("DB_NAME not set");
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");

    let result = get_env_vars().unwrap();

    assert_eq!(result.host, host);
    assert_eq!(result.user, user);
    assert_eq!(result.password, password);
    assert_eq!(result.database, database);
    assert_eq!(result.api_key, api_key);
}


/// Test get_zip_data function
/// 
/// This test checks that the get_zip_data function makes an API call and returns proper data.
#[tokio::test]
async fn test_get_zip_data() {
    let zip = "95124".to_string();
    let api = std::env::var("API_KEY").expect("API_KEY not set");
    let result = get_zip_data(zip, api).await.unwrap();

    let expected = ResponseItem {
        zip: "95124".to_string(),
        weather: Weather{ 
            main: Main{ temp: 0.0, humidity: 0 },
            weather: vec![WeatherDescription{ description: "clear sky".to_string() }],
            wind: Wind{ speed: 0.0 },
            name: "San Jose".to_string(),
        },
    };

    assert_eq!(result.zip, expected.zip);
    assert_eq!(result.weather.name, expected.weather.name);
   
}


/// Test get_target_data function
/// 
/// This test checks that the get_target_data function returns a vector of weather data for a list of zip codes.
#[tokio::test]
async fn test_get_target_data() {
    let target_list = vec!["95124".to_string(), "95014".to_string(), "95123".to_string()];
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");
    let result = get_target_data(target_list, api_key).await;

    assert_eq!(result.len(), 3);

    // map the result to a vector of zip codes
    let zips: Vec<String> = result.iter().map(|x| x.as_ref().unwrap().zip.clone()).collect();
    assert_eq!(zips, vec!["95124".to_string(), "95014".to_string(), "95123".to_string()]);


}


