use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let postal_code = &args[1];

    let api_key = "8c6f3dd4789e4646a5d44946231402";
    let api = weather::WeatherApi::new(api_key);
    let current = api.get_current(postal_code).await?;

    println!(
        "Current weather in {} ({}, {}): 
        Temperature: {}°F, 
        Condition: {}",
        current.location.name,
        current.location.region,
        current.location.country,
        current.current.temp_f,
        current.current.condition.text,
    );

    Ok(())
}
