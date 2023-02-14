use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub temp_f: f32,
    pub condition: Condition,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub location: Location,
    pub current: Current,
}

pub struct WeatherApi {
    api_key: String,
}

impl WeatherApi {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub async fn get_current(&self, postal_code: &str) -> Result<Forecast, reqwest::Error> {
        let url = format!(
            "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7",
            self.api_key, postal_code
        );
        let client = Client::new();
        let res = client.get(&url).send().await?;
        let forecast = res.json::<Forecast>().await?;
        Ok(forecast)
    }
}
