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
- [ ] look into error 
```
thread 'actix-rt|system:0|arbiter:0' panicked at 'Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.', /home/t0astm/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.27.0/src/runtime/blocking/shutdown.rs:51:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
- [ ] Add pagination to generate cocktails
- [ ] Remove mappings when fetching cocktails. Only add mappings to 
      get_cockatail_by_id
- [ ] Make cocktails unique 
```
CREATE UNIQUE INDEX person_name_upper ON person(
    UPPER(first_name), UPPER(last_name));
```
- [ ] Add proper error handlening / Response Errors
```
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
```
