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


## TODO:

- [ ] Add pagination to generate cocktails
- [ ] Remove mappings when fetching cocktails. Only add mappings to 
      get_cockatail_by_id
- [ ] Make cocktails unique 
```
CREATE UNIQUE INDEX person_name_upper ON person(
    UPPER(first_name), UPPER(last_name));
```

