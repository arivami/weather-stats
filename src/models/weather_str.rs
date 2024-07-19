#[derive(Debug)]
pub struct WeatherData {
    city: String,
    zip: String,
    temperature: f64,
    weather: String,
    humidity: String,
    wind_speed: f64,
}

impl WeatherData {
    pub fn new(city: String, zip: String, temperature: f64, weather: String, humidity: String, wind_speed: f64) -> Self {
        WeatherData { city, zip, temperature, weather, humidity, wind_speed }
    }

    // Add getters or other methods as needed
}