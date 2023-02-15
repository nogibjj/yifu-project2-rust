# Individual Project #2: Weather Forecast

=======
# Rust-micorservice
This is a course project which is a simple microservice for forecasting the weather in the future. This project image will be pushed to DockerHub, or Cloud based Container Registery (ECR), and it will be deployed automatically to Kubernetes cluster.

## Steps to run
- `make format` to format code
- `make lint` to lint
- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
- `make deploy` which is this `cargo lambda deploy`

## Tasks

1. Get the current weather and condition by entering a postal code
2. Forecast the weather and condition for the next 7 days
3. Deploy to kubernetes

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [weather API](https://www.weatherapi.com/my/)
