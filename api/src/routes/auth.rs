use crate::crypto::{hash_password, verify_password};
use crate::models::*;
use crate::pool;
use crate::schema::users;
use diesel::prelude::*;
use diesel::*;
use poem::{
    error::{Error, Result},
    handler,
    http::StatusCode,
    web::{Data, Json},
};
use serde::Deserialize;
use serde_json::json;

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
            email,
            ..
        }) => {
            let logged_in =
                verify_password(&params.password.as_bytes(), &password_hash.to_string());
            println!("logged_in is {:?}", logged_in);
            Ok(Json(json!({ "email": email })))
        }
        Err(_) => Err(Error::from_status(StatusCode::BAD_REQUEST)),
    }
}
