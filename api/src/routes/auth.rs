use crate::crypto::{hash_password, verify_password};
use crate::models::user::*;
use crate::pool;
use crate::schema::users;
use diesel::prelude::*;
use diesel::*;
use poem::{
    error::{Error, Result},
    handler,
    http::StatusCode,
    web::{Data, Json},
    session::Session,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{Duration, Utc};


#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub sub: String,
    pub exp: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct AuthParams {
    email: String,
    password: String,
}

#[handler]
pub async fn register(
    Json(params): Json<AuthParams>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let new_user = NewUser {
        email: params.email.to_string(),
        password_hash: hash_password(params.password.as_bytes()),
    };

    let conn = pool
        .get()
        .map_err(|_e| Error::from_status(StatusCode::BAD_REQUEST))?;

    let insert_result = insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&conn);

    println!("{:?}", insert_result);

    match insert_result {
        Ok(user) => Ok(Json(json!(user))),
        Err(_) => Err(Error::from_status(StatusCode::BAD_REQUEST)),
    }
}

#[handler]
pub async fn login(
    Json(params): Json<AuthParams>,
    session: &Session,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let email = params.email.to_string();

    let conn = pool
        .get()
        .map_err(|_e| Error::from_status(StatusCode::BAD_REQUEST))?;

    let result = users::table
        .filter(users::email.eq(&email))
        .first::<User>(&conn);

    println!("result is {:?}", result);

    match result {
        Ok(User {
            password_hash,
            ..
        }) => {
            let logged_in =
                verify_password(&params.password.as_bytes(), &password_hash.to_string());

            println!("logged_in is {:?}", logged_in);

            if logged_in {
                let token = UserSession {
                    sub: email,
                    exp: Utc::now() + Duration::days(1),
                };
                
                session.set("session", token);

                Ok(Json(json!({ "status": "ok" })))
            } else {
                Ok(Json(json!({ "status": "auth failed" })))
            }
        }
        Err(_) => Err(Error::from_status(StatusCode::BAD_REQUEST)),
    }
}
