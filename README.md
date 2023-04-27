requirement:
- docker

This is a rust backend which show `hello world` on the root page
Run the app once to test the hello world by calling ./run.sh

To run the project, use `docker compose up` and the app will be run on port 80 on the host machine
it will also running locust for doing the load test, open http://localhost:8089 to open locust

I open two port for the hello world app (80 and 8080) because the locust container can't connect to port 80.