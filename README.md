[![Tests](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml)
[![Build binary release](https://github.com/noahgift/rdedupe/actions/workflows/release.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/release.yml)

# Individual Project #2: Weather Forecast

## Description

This is a course project which is a simple microservice for forecasting the weather in the future. This project image will be pushed to DockerHub, or Cloud based Container Registery (ECR), and it will be deployed automatically to Kubernetes cluster.

## Steps to run

- `make format` to format code
- `make lint` to lint
- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
- `make deploy` which is this `cargo lambda deploy`

```bash
@yifu-deng ➜ /workspaces/yifu-rust-project2/WeatherForecast (main) $ cargo run 27705   
Compiling weather v0.1.0 (/workspaces/yifu-rust-project2/WeatherForecast)
    Finished dev [unoptimized + debuginfo] target(s) in 2.62s
     Running `target/debug/weather 27705`
Weather forecast for Durham (North Carolina, USA)
        Last updated time: 2023-02-21 13:00
        Time zone: America/New_York
        The query time is 2023-02-21 13:08;
        Today is 2023-02-21,
        
Date: 2023-02-21,
            Maximum temperature: 69.3°F,
            Minimum temperature: 55.8°F,
            Average temperature: 61.2°F,
            Weather condition: Cloudy
            The probability of rain is 0%,
            The probability of snow is 0%,
            The average humidity is 60%,
            Sunrise: 06:56 AM; Sunset: 06:03 PM.

Date: 2023-02-22,
            Maximum temperature: 73.9°F,
            Minimum temperature: 50.4°F,
            Average temperature: 60.7°F,
            Weather condition: Overcast
            The probability of rain is 0%,
            The probability of snow is 0%,
            The average humidity is 69%,
            Sunrise: 06:55 AM; Sunset: 06:04 PM.

Date: 2023-02-23,
            Maximum temperature: 76.1°F,
            Minimum temperature: 60.4°F,
            Average temperature: 67.4°F,
            Weather condition: Partly cloudy
            The probability of rain is 0%,
            The probability of snow is 0%,
            The average humidity is 79%,
            Sunrise: 06:54 AM; Sunset: 06:05 PM.

Date: 2023-02-24,
            Maximum temperature: 61.9°F,
            Minimum temperature: 46°F,
            Average temperature: 56.7°F,
            Weather condition: Sunny
            The probability of rain is 0%,
            The probability of snow is 0%,
            The average humidity is 51%,
            Sunrise: 06:52 AM; Sunset: 06:06 PM.

Date: 2023-02-25,
            Maximum temperature: 39.6°F,
            Minimum temperature: 37.6°F,
            Average temperature: 40°F,
            Weather condition: Moderate rain
            The probability of rain is 88%,
            The probability of snow is 0%,
            The average humidity is 71%,
            Sunrise: 06:51 AM; Sunset: 06:07 PM.

Date: 2023-02-26,
            Maximum temperature: 67.8°F,
            Minimum temperature: 41.4°F,
            Average temperature: 53.6°F,
            Weather condition: Patchy rain possible
            The probability of rain is 73%,
            The probability of snow is 0%,
            The average humidity is 87%,
            Sunrise: 06:50 AM; Sunset: 06:07 PM.

Date: 2023-02-27,
            Maximum temperature: 51.3°F,
            Minimum temperature: 38.3°F,
            Average temperature: 46.4°F,
            Weather condition: Partly cloudy
            The probability of rain is 0%,
            The probability of snow is 0%,
            The average humidity is 59%,
            Sunrise: 06:48 AM; Sunset: 06:08 PM.
```

## Containerization

- I also **containerized** this project with `Docker`.

- `make docker-build` to build docker image
- `make docker-run` to run docker image


## Deploy with Kubernetes FastAPI app

1.  Push container to DockerHub (Optional): i.e. 
`docker build -t yifud/weather/weather .` and `docker push yifud/weather:weather`
Example of a pushed FastAPI container here:  https://hub.docker.com/repository/docker/yifud/weather
2. `minikube start`
3. `minikube dashboard --url`
4. Hover over link and "follow link"
5. Create a deployment: `kubectl create deployment weather-api --image=registry.hub.docker.com/yifud/weather`
6. View deployment: `kubectl get deployments`
7. Create service and expose it: `kubectl expose deployment weather-api --type=LoadBalancer --port=8080`
8. View services:  `kubectl get service weather-api`
9.  `minikube service weather-api --url`
10. Curl web service: i.e. `curl http://192.168.49.2:31224`
11. Cleanup
12. Cleanup
```bash
kubectl delete service weather-api
kubectl delete deployment weather-api
minikube stop
````

<img width="710" alt="image" src="https://user-images.githubusercontent.com/77519205/221040317-77019f96-e344-4455-9be0-7b6f3825e42e.png">

## Tasks

1. Get the current weather and condition by entering a postal code
2. Forecast the weather and condition for the next 7 days
3. Create a customized Docker container from the current version of Rust that deploys a simple python script.
4. Push image to DockerHub, or Cloud based Container Registery (ECR)
5. Project should deploy automatically to Kubernetes cluster
6. Deployment should be to some form of Kubernetes service (can be hosted like Google Cloud Run or Amazon EKS, etc)

## References

- [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
- [weather API](https://www.weatherapi.com/my/)
- [coursera-applied-de-kubernetes-lab](https://github.com/nogibjj/coursera-applied-de-kubernetes-lab)
- [Hello MiniKube](https://kubernetes.io/docs/tutorials/hello-minikube/)
- [Get started with Kubernetes(Python)](https://kubernetes.io/blog/2019/07/23/get-started-with-kubernetes-using-python/)

