docker build . -t registry.digitalocean.com/rich-registry/rich-darts
docker container stop test && docker container rm test
docker run -d --name test -v sqlite:/home/ -e DATABASE_URL='/home/richDarts.db' -e LOG_URL='/home/server.log' registry.digitalocean.com/rich-registry/rich-darts
docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' test



