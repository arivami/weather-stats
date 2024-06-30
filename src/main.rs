


/* use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    
    run(service_fn(function_handler)).await
}
 */

 mod utils;
 mod openweathermap;
 mod config;

 use config::config::{load_config, randomize_target_list, WeatherPullConf};
 
 use openweathermap::open_weather_data::ResponseItem;
use reqwest::Error;
 //use futures::future::join_all;
 use crate::openweathermap::open_weather_data::{Weather,APIRequestParams,WorkList};


 #[tokio::main]
async fn main() -> Result<(), Error> {

    let pull_conf :WeatherPullConf = load_config("/Users/mikez/Projects/weather-stats/src/test-data/weather-pull-conf.json".to_string());

    let targets = randomize_target_list(pull_conf);

    let requests: Vec<WorkList> = targets.iter().map(|x|
        {
            let request_params = APIRequestParams{
                zip: x.to_string(),
                api_key:"5ffce5e80bb83c6f974df8aeb9542960".to_string(),
                unit:"imperial".to_string()

                };
            let url = format!(
                    "http://api.openweathermap.org/data/2.5/weather?zip={}&appid={}&units={}",
                    request_params.zip, request_params.api_key, request_params.unit
                );
            WorkList{zip: request_params.zip, url: url}         
    } 
    ).collect();


      let responses: Vec<Result<ResponseItem, Error>> = futures::future::join_all(
        requests.into_iter().map(|x| {
            async move {
                let response = reqwest::get(x.url).await?;
                let resp_item: ResponseItem= ResponseItem{
                    zip:x.zip.to_lowercase(), weather: response.json().await?,
                };
                Ok(resp_item)
            }
        })
    ).await;

 for response in responses {
    match response {
        Ok(ri) => {
            println!("City:{}", ri.weather.name);
            println!("Zip:{}", ri.zip);
            println!("Temperature: {}", ri.weather.main.temp);
            println!("Weather: {}", ri.weather.weather[0].description);
            println!("Humidity: {}%", ri.weather.main.humidity);
            println!("Wind Speed: {} m/s", ri.weather.wind.speed);
            println!();
        },
        Err(e) => println!("Error: {}", e),
    }
}

    


    Ok(())
}