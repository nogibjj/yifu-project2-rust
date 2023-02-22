use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weatherforecast {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub tz_id: String,
    pub localtime: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub last_updated: String,
    pub temp_f: f32,
    pub condition: Condition,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>,
}

#[derive(Debug, Deserialize)]
pub struct Forecastday {
    pub date: String,
    pub day: Day,
    pub astro: Astro,
    pub hour: Vec<Hour>,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    pub maxtemp_f: f32,
    pub mintemp_f: f32,
    pub avgtemp_f: f32,
    pub avghumidity: f32,
    pub condition: Condition,
    pub daily_chance_of_rain: f32,
    pub daily_chance_of_snow: f32,
}

#[derive(Debug, Deserialize)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
}

#[derive(Debug, Deserialize)]
pub struct Hour {
    pub time: String,
    pub temp_f: f32,
    pub condition: Condition,
}

pub struct Weather {
    api_key: String,
}

impl Weather {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub async fn get_weather(&self, input: &str) -> Result<Weatherforecast, reqwest::Error> {
        let url = format!(
            "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7&aqi=no&alerts=no",
            self.api_key, input
        );
        let client = Client::new();
        let res = client.get(&url).send().await?;
        let forecast = res.json::<Weatherforecast>().await?;
        Ok(forecast)
    }
}

// First version of the code, just get the current weather
// impl WeatherApi {
//     pub fn new(api_key: &str) -> Self {
//         Self {
//             api_key: api_key.to_string(),
//         }
//     }

//     pub async fn get_weather(&self, postal_code: &str) -> Result<Forecast, reqwest::Error> {
//         let url = format!(
//             "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7&aqi=no&alerts=no",
//             self.api_key, postal_code
//         );
//         let client = Client::new();
//         let res = client.get(&url).send().await?;
//         let forecast = res.json::<Forecast>().await?;
//         Ok(forecast)
//     }
// }
