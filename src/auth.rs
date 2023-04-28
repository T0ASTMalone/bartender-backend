use crate::{errors::ServiceError, repository::database::Database, models::users::{User, NewUser}};

use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims { 
    sub: String, 
    company: String,
    exp: usize,
}

fn ensure_created(db: &Database, new_user: NewUser) -> bool {
    println!("new user {:?}", new_user);
    true
    /*
    let existing = User::get_user_by_email(db, new_user.email);

    match existing {
        Ok(_) => return true,
        _ => { }
    }

    let new_user = User::create_user(db, new_user);

    match new_user {
        Ok(_) => return true,
        _ => { }
    }

    return false;
    */
}

pub async fn validate_token(token: &str, db: &Database) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");

    let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"))
        .await
        .expect("faild to fetch jwks");

    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];

    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    
    let jwk = jwks.find(&kid).expect("Spcified key not found in set");
    let res = validate(token, jwk, validations);
    match res {
        Ok(jwt) => {
            println!("claims {:?}", jwt.claims);
            /*
            let user_exists_or_was_created = ensure_created(db, NewUser { 
                username: jwt.claims.get("username").unwrap().to_string().as_str(), 
                email: jwt.claims.get("email").unwrap().to_string().as_str(),
            });
            */

            return Ok(true)
        }
        Err(_) => {
            return Ok(false);
        }
    };
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    Ok(val)
}
