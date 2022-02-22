use diesel::{QueryDsl, RunQueryDsl};
use poem::{
    Endpoint, IntoResponse, Request, Response,
    Result,
    error::Error,
    http::StatusCode,
    session::Session,
};
use crate::models::user::User;
use crate::pool::Pool;
use crate::schema::users;
use crate::routes::auth::UserSession;

// Get this user from the database.
fn get_user(user_id: String, conn: &diesel::PgConnection) -> Result<User> {
    let id = user_id.parse::<i32>().map_err(|_e| Error::from_status(StatusCode::BAD_REQUEST))?;

    users::table.find(id).first::<User>(conn).map_err(|_e| Error::from_status(StatusCode::BAD_REQUEST))
}

// Get the user_id and database pool out of the request.
fn deserialize_params(req: &Request) -> Result<(String, &Pool)> {
    let user_id = req
            .path_params::<String>()
            .map_err(|_| Error::from_string("user_id not found", StatusCode::BAD_REQUEST))?;

    let pool = req
            .extensions()
            .get::<Pool>()
            .ok_or(Error::from_string("db pool not found", StatusCode::BAD_REQUEST))?;

    Ok((user_id, pool))
}

pub async fn auth_middleware<E: Endpoint>(next: E, mut req: Request) -> Result<Response> {
    println!("auth middleware req: {:?}", req);

    let (user_id, pool) = deserialize_params(&req)?;

    // Get a connection from the pool.
    let conn = pool.get()
        .map_err(|_| Error::from_string("could not establish db connection", StatusCode::BAD_REQUEST))?;

    let user = get_user(user_id, &conn)?;

    let session = req
        .extensions()
        .get::<Session>()
        .ok_or(Error::from_string("session not found", StatusCode::BAD_REQUEST))?;

    let user_session = session.get::<UserSession>("session")
            .ok_or(Error::from_string("no session token", StatusCode::BAD_REQUEST))?;

    println!("found session: {:?}", user_session);

    // Check validity of this session.
    if user_session.sub == user.email && user_session.exp > chrono::Utc::now() {
        // Insert it into this request.
        req.extensions_mut().insert(user);

        let res = next.call(req).await;

        match res {
            Ok(resp) => {
                let resp = resp.into_response();
                println!("response: {}", resp.status());
                Ok(resp)
            }
            Err(err) => {
                println!("error: {}", err);
                Err(err)
            }
        }
    } else {
        Err(Error::from_string("bad session match", StatusCode::BAD_REQUEST))
    }
}
