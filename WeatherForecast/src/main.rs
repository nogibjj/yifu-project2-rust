use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Weather Forecast")
}

#[get("/{postal_code_name}")]
async fn get_weather_forecast(postal_code_name: web::Path<String>) -> impl Responder {
    let api_key = "8c6f3dd4789e4646a5d44946231402";
    let api = weather::Weather::new(api_key);
    let forecast = api.get_weather(&postal_code_name).await.unwrap();

    let last_updated = forecast.current.last_updated;
    let timezone = forecast.location.tz_id;
    let localtime = forecast.location.localtime;

    let mut forecast_strings = Vec::new();
    for forecast_day in forecast.forecast.forecastday.iter() {
        let date = &forecast_day.date;
        let max_temp = forecast_day.day.maxtemp_f;
        let min_temp = forecast_day.day.mintemp_f;
        let avg_temp = forecast_day.day.avgtemp_f;
        let weather_condition = &forecast_day.day.condition.text;
        let chance_of_rain = forecast_day.day.daily_chance_of_rain;
        let chance_of_snow = forecast_day.day.daily_chance_of_snow;
        let avg_humidity = forecast_day.day.avghumidity;
        let sunrise = &forecast_day.astro.sunrise;
        let sunset = &forecast_day.astro.sunset;

        let forecast_string = format!(
            "\nDate: {date},
            Maximum temperature: {max_temp}°F,
            Minimum temperature: {min_temp}°F,
            Average temperature: {avg_temp}°F,
            Weather condition: {weather_condition}
            The probability of rain is {chance_of_rain}%,
            The probability of snow is {chance_of_snow}%,
            The average humidity is {avg_humidity}%,
            Sunrise: {sunrise}; Sunset: {sunset}.\n"
        );

        forecast_strings.push(forecast_string);
    }

    let forecast_string = forecast_strings.join("");

    let output_string = format!(
        "Weather forecast for {} ({}, {})
        Last updated time: {}
        Time zone: {}
        The query time is {};
        Today is {},
        {}
        ",
        forecast.location.name,
        forecast.location.region,
        forecast.location.country,
        last_updated,
        timezone,
        localtime,
        forecast.forecast.forecastday[0].date,
        forecast_string
    );

    println!("{output_string}");

    HttpResponse::Ok().body(output_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_weather_forecast))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}


// This version is to get the output in the command line
// use std::env;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();
//     let postal_code_name = &args[1];

//     let api_key = "8c6f3dd4789e4646a5d44946231402";
//     let api = weather::Weather::new(api_key);
//     let forecast = api.get_weather(postal_code_name).await?;

//     let last_updated = forecast.current.last_updated;
//     let timezone = forecast.location.tz_id;
//     let localtime = forecast.location.localtime;

//     let mut forecast_strings = Vec::new();
//     for forecast_day in forecast.forecast.forecastday.iter() {
//         let date = &forecast_day.date;
//         let max_temp = forecast_day.day.maxtemp_f;
//         let min_temp = forecast_day.day.mintemp_f;
//         let avg_temp = forecast_day.day.avgtemp_f;
//         let weather_condition = &forecast_day.day.condition.text;
//         let chance_of_rain = forecast_day.day.daily_chance_of_rain;
//         let chance_of_snow = forecast_day.day.daily_chance_of_snow;
//         let avg_humidity = forecast_day.day.avghumidity;
//         let sunrise = &forecast_day.astro.sunrise;
//         let sunset = &forecast_day.astro.sunset;

//         let forecast_string = format!(
//             "\nDate: {date},
//             Maximum temperature: {max_temp}°F,
//             Minimum temperature: {min_temp}°F,
//             Average temperature: {avg_temp}°F,
//             Weather condition: {weather_condition}
//             The probability of rain is {chance_of_rain}%,
//             The probability of snow is {chance_of_snow}%,
//             The average humidity is {avg_humidity}%,
//             Sunrise: {sunrise}; Sunset: {sunset}.\n"
//         );

//         forecast_strings.push(forecast_string);
//     }

//     let forecast_string = forecast_strings.join("");

//     let output_string = format!(
//         "Weather forecast for {} ({}, {})
//         Last updated time: {}
//         Time zone: {}
//         The query time is {};
//         Today is {},
//         {}
//         ",
//         forecast.location.name,
//         forecast.location.region,
//         forecast.location.country,
//         last_updated,
//         timezone,
//         localtime,
//         forecast.forecast.forecastday[0].date,
//         forecast_string
//     );

//     println!("{output_string}");

//     Ok(())
// }

// First version of the code, just for the current weather
// use std::env;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();
//     let postal_code = &args[1];

//     let api_key = "8c6f3dd4789e4646a5d44946231402";
//     let api = weather::WeatherApi::new(api_key);
//     let current = api.get_current(postal_code).await?;

//     println!(
//         "Current weather in {} ({}, {}):
//         Temperature: {}°F,
//         Condition: {}",
//         current.location.name,
//         current.location.region,
//         current.location.country,
//         current.current.temp_f,
//         current.current.condition.text,
//     );

//     Ok(())
// }
