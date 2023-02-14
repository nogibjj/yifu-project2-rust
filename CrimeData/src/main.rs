// create a function that make a request to the API of the Crime Data Explorer
fn get_crime_data() -> Result<CrimeData, Box<dyn Error>> {
    // create a client
    let client = reqwest::blocking::Client::new();
    // make a request to the API
    let res = client.get("https://crime-data-explorer.fr.cloud.gov/api").send()?;
    // convert the response to a CrimeData struct
    let crime_data: CrimeData = res.json()?;
    // return the CrimeData struct
    Ok(crime_data)
}
