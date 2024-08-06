

pub mod open_weather_data {
   
    use serde::Deserialize;

    pub struct WorkList{
        pub zip:String,
        pub url:String
    } 

    #[derive(Deserialize, Debug)]
    pub struct Weather {
        pub main: Main,
        pub weather: Vec<WeatherDescription>,
        pub wind: Wind,
        pub name: String,
    }
    
    #[derive(Deserialize, Debug)]
    pub struct Main {
        pub temp: f64,
        pub humidity: u8,
    }
    
    #[derive(Deserialize, Debug)]
    pub struct WeatherDescription {
        pub description: String,
    }
    
    #[derive(Deserialize, Debug)]
    pub struct Wind {
        pub speed: f64,
    }


    pub struct APIRequestParams{
        pub zip: String,
        pub api_key: String,
        pub unit: String,
    }

    #[derive(Debug)]
    pub struct ResponseItem{
        pub zip:String,
        pub weather:Weather,
    }

}