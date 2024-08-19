
pub mod helper_funcs{
    use futures::future::join_all;

    use crate::openweathermap::open_weather_data::ResponseItem;

    #[derive(Debug)]
    pub struct EnvVars {
        pub host:String,
        pub user:String,
        pub password:String,
        pub database:String,
        pub api_key:String,
    }

    pub fn get_env_vars() -> EnvVars {
        let host = std::env::var("DB_HOST").expect("DB_HOST not set");
        let user = std::env::var("DB_USER").expect("DB_USER not set");
        let password = std::env::var("DB_PASS").expect("DB_PASS not set");
        let database = std::env::var("DB_NAME").expect("DB_NAME not set");
        let api_key = std::env::var("API_KEY").expect("API_KEY not set");
        EnvVars{host,user,password,database,api_key}
    }

    pub fn get_db_url(env_vars:EnvVars)->String {
        format!("mysql://{}:{}@{}/{}", env_vars.user, env_vars.password, env_vars.host, env_vars.database)
    }

    pub async fn connect_to_db(url:String)->sea_orm::DatabaseConnection {
        println!("Connecting to the database");
        sea_orm::Database::connect(&url).await.unwrap()
    }

    pub async fn get_zip_data(zip:String, api_key:String)->Result<ResponseItem, reqwest::Error> {
        println!("Requesting data from API");
        let url = format!("http://api.openweathermap.org/data/2.5/weather?zip={}&appid={}&units={}", zip, api_key, "imperial");
        let response = reqwest::get(url).await?;
        let resp_item: ResponseItem= ResponseItem{
            zip:zip.to_lowercase(), weather: response.json().await?,
        };
        println!("Response: {:?}", resp_item);
        Ok(resp_item)
    }

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

