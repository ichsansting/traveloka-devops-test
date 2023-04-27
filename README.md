requirement:

- docker

This is a rust backend which show `hello world` on the root page

<h2>why rust?</h2>
in my understanding golang will be more suitable for backend service and devops overall, but i did read about how discord handle scaling on their message and their choice to change the source code to rust, that makes me interested in rust, so i make this test as an opportunity to try and learn it.

<h1>Task 1</h1>

Change branch and Run the app once to test the hello world

```bash
git checkout solutions/task1
./run.sh
```

To run the project, use `docker compose up` and the app will be run on port 80 on the host machine
it will also running locust for doing the load test, open http://localhost:8089 to open locust

```bash
docker compose up -d
```

I publish two port for the hello world app (80 and 8080) because the locust container can't connect to port 80 (i think it's because i'm using codespace).

<h1>Task 2</h1>

Change branch and Run solution 2

```bash
docker compose down
git checkout solutions/task2
docker compose up -d --build
```

there is a script for testing the crud operation, run by calling ./test_crud.sh

```bash
./test_crud.sh
```

Scale up the app to 2

```bash
docker compose up -d --scale app=2
```

Scale down the app back to 1

```bash
docker compose up -d --scale app=1
```

Rollback
```bash
git checkout solutions/task1
docker compose up --build
```
The easiest way is just to change the branch and then re build the image, the better approach is to tagging the docker image and then use docker-compose variable to switch between versions, but i think in this scenario changing the version frequently is not necessary.

Potential Improvements:

- better error handling, currently error handling is not that much needed for this simple app
- better folder structure, like make the url path is also can be seen on the filename/foldering, so people can easily understand about the flow on the project
- use better tools/library for automated test the CRUD, i don't really write testing before, so i'll just use simple cURL and read the response code
- reduce redundant code, like db connection
- improve security (cors)
- using orm, so the database can be changed as needed
   on my implementation i want to just doing the crud first so i'm using sqlite, because i don't need to setup anything, i use library only for the sqlite, so it will takes more time to change the database. i thought i could just add sqlite container and do the database connection as usual, but i forgot that it just a normal file, even tho i can start sqlite on another container. the things i need for it is just the storage. so i'll not gonna use the database in different container, it should not makes any different because the connection to database will not that much different from connection to locust (docker-compose), it just using different port and host.
