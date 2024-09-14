# Parkinson's Pulse Service
The Backend service for parkinson's pulse

# Run locally
For front end to run this locally you can do the following steps
```bash
git clone https://github.com/BigLNotation/parkinsons_pulse_service.git
cd parkinsons_pulse_service
docker compose up -d
docker run -d -p 4444:4444 .
```
or something along these lines, you are smart you can do it :)

### Notes:
This will not have any data in it unless you spec out a volume for the container. 
To access the server the has now been started the url will be `localhost:4444/`. 

# Configurations
## ENV
- `API_PORT`, Specs the port that this app will serve on.

# Tracing
## Jaeger
This service allows for local use of jaeger for tracing. 
![jaeger_log_example.png](docs/jaeger_log_example.png)
To enable this feature please follow the following steps.

- Run Jaeger `docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest`
- Run with jaeger tracing feature enabled `cargo run --features jaeger_tracing`
- Open Jaeger `http://localhost:16686/`

If you are using JetBrains you can create a profile to auto run this feature:
![JetBrainsJaegerFeat.png](docs/JetBrainsJaegerFeat.png)
