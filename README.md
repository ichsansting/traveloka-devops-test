requirement:
- docker

This is a rust backend which show `hello world` on the root page

why rust?
in my understanding golang will be more suitable for backend service and devops overall, but i did read about how discord handle scaling on their message and their choice to change the source code to rust, that makes me interested in rust, so i make this test as an opportunity to try and learn it.

Run the app once to test the hello world by calling ./run.sh
To run the project, use `docker compose up` and the app will be run on port 80 on the host machine<br/>
it will also running locust for doing the load test, open http://localhost:8089 to open locust

I publish two port for the hello world app (80 and 8080) because the locust container can't connect to port 80 (i think it's because i'm using codespace).

In the solution 2, there is a script for testing the crud operation, run by calling ./test_crud.sh

Potential Improvements:
- better error handling, currently error handling is not that much needed for this simple app
- better folder structure, like make the url path is also can be seen on the filename/foldering, so people can easily understand about the flow on the project
- use better tools/library for automated test the CRUD, i don't really write testing before, so i'll just use simple cURL and read the response code
- reduce redundant code, like db connection
- improve security (cors)
- using orm, so the database can be changed as needed
    on my implementation i want to just doing the crud first so i'm using sqlite, because i don't need to setup anything, i use library only for the sqlite, so it will takes more time to change the database. i thought i could just add sqlite container and do the database connection as usual, but i forgot that it just a normal file, even tho i can start sqlite on another container. the things i need for it is just the storage. so i'll not gonna use the database in different container, it should not makes any different because the connection to database will not that much different from connection to locust (docker-compose), it just using different port and host.