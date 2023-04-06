# Bartender

## To get it running
```
docker-compose -f postgres.yaml up -d
diesel setup
diesel migration run
```

## To clean up
```
docker-compose -f postgres.yaml down
docker volume rm rust-actix-web-rest-api-diesel_postgres-data
docker-compose -f telemetry.yaml down
```
