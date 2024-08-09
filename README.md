# tx-web-server
add .env
```env
GOOGLE_APPLICATION_CREDENTIALS=path/to/backet/creds.json
```

run docker
```bash
docker run -d --name tx-web-server -p 80:80 --env-file=./.env --restart on-failure -v /etc/data/credentials.json:/etc/data/credentials.json tx-web-server:latest 
```
