# scp to vps on same directory as the docker compose file
# only if volumne needs to be recreated
# docker compose down -v 
docker compose pull && docker compose up -d
