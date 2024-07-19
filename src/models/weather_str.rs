#[derive(Debug)]
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