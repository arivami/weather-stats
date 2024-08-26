use weather_stats::utils::helper_funcs::*;
use weather_stats::openweathermap::open_weather_data::*;

use dotenvy::dotenv;


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
fn test_get_env_vars() {
    dotenv().ok();
    let host = std::env::var("DB_HOST").expect("DB_HOST not set");
    let user = std::env::var("DB_USER").expect("DB_USER not set");
    let password = std::env::var("DB_PASS").expect("DB_PASS not set");
    let database = std::env::var("DB_NAME").expect("DB_NAME not set");
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");

    let result = get_env_vars();

    assert_eq!(result.host, host);
    assert_eq!(result.user, user);
    assert_eq!(result.password, password);
    assert_eq!(result.database, database);
    assert_eq!(result.api_key, api_key);
}

#[test]
fn test_get_db_url() {
    let env_vars = setup();
    let db_url = get_db_url(env_vars);
    assert_eq!(db_url, "mysql://user:password@host/database");
}

#[tokio::test]
#[ignore]
async fn test_connect_to_db() {
    // let env_vars = get_env_vars();
    // let db_url = get_db_url(env_vars);
    // let db = connect_to_db(db_url);


    //assert!(db.ping().await.is_ok());
    // db.clone().close().await;
    // assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire)));
}

#[tokio::test]
async fn test_get_zip_data() {
    let zip = "95124".to_string();
    let env_vars = get_env_vars();  
    let api = env_vars.api_key;
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

#[tokio::test]
async fn test_get_target_data() {
    let target_list = vec!["95124".to_string(), "95014".to_string(), "95123".to_string()];
    let env_vars = get_env_vars();
    let api_key = env_vars.api_key;
    let result = get_target_data(target_list, api_key).await;

    assert_eq!(result.len(), 3);

    // map the result to a vector of zip codes
    let zips: Vec<String> = result.iter().map(|x| x.as_ref().unwrap().zip.clone()).collect();
    assert_eq!(zips, vec!["95124".to_string(), "95014".to_string(), "95123".to_string()]);


}


