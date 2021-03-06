use actix_web::*;
use actix_web_grants::proc_macro::{has_roles};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::schema_db_enum::UserRole;
use crate::User;

type DBPool = web::Data<Pool<ConnectionManager<PgConnection>>>;
type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

const POOL_ERR: &str = "Could not get DB connection from pool";

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub login: String,
    pub role: UserRole,
    pub password: String,
}

pub async fn handle_get_users(pool: DBPool) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("handle_get_users"))
}

pub async fn handle_get_user_by_id(pool: DBPool, user_id: web::Path<u32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("handle_get_user_by_id"))
}

#[has_roles("ADMIN")]
pub async fn handle_add_user(pool: DBPool, item: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    let dbc = pool.get().expect(POOL_ERR);

    let InputUser {login, role, password} = item.0;

    let user = web::block( || add_user(dbc, login, role, password))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

pub fn add_user(dbc: DBConnection, login: String, role: UserRole, password: String) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl;
    use rand::{thread_rng, distributions::Alphanumeric};
    use argon2::{hash_encoded, Config};

    let salt: String = thread_rng().sample_iter(Alphanumeric).take(16).map(char::from).collect();
    let hash = hash_encoded(password.as_ref(), salt.as_ref(), &Config::default()).unwrap();

    diesel::insert_into(dsl::users).values((
        dsl::login.eq(login),
        dsl::role.eq(role),
        dsl::salt.eq(salt),
        dsl::hash.eq(hash)
    )).get_result(&dbc)
}

pub async fn handle_delete_user(pool: DBPool, user_id: web::Path<u32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("handle_delete_user"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();


        //let resp = handle_add_user(AuthDetails {}, (), Json()).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        //let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}