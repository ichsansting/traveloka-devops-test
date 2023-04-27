docker compose up -d
echo -e "\n"
sleep 2

curl -I localhost
curl localhost

sleep 2
echo -e "\n"
docker compose down