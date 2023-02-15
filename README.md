# Individual Project #2: Weather Forecast

=======

## Description

This is a course project which is a simple microservice for forecasting the weather in the future. This project image will be pushed to DockerHub, or Cloud based Container Registery (ECR), and it will be deployed automatically to Kubernetes cluster.

## Steps to run

- `make format` to format code
- `make lint` to lint
- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
- `make deploy` which is this `cargo lambda deploy`

```bash
@yifu-deng ➜ /workspaces/yifu-rust-project2/CrimeData (main) $ make format
cargo fmt --quiet
@yifu-deng ➜ /workspaces/yifu-rust-project2/CrimeData (main) $ make lint
cargo clippy --quiet
@yifu-deng ➜ /workspaces/yifu-rust-project2/CrimeData (main) $ cargo run 27705
   Compiling weather v0.1.0 (/workspaces/yifu-rust-project2/WeatherForecast)
    Finished dev [unoptimized + debuginfo] target(s) in 3.93s
     Running `target/debug/weather 27705`
Current weather in Durham (North Carolina, USA): 
        Temperature: 53.1°F, 
        Condition: Partly cloudy
```

## Tasks

1. Get the current weather and condition by entering a postal code
2. Forecast the weather and condition for the next 7 days
3. Deploy to kubernetes

## References

- [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
- [weather API](https://www.weatherapi.com/my/)
