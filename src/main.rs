mod error;
mod infra;
mod app;

use app::repository::user_repository::repository::IUserRepository;
use infra::repository::{user_repository::repository::{UserRepository}, repository::repository::Repository};
use mongodb::{bson::{doc, self}};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use warp::{reject, reply::json, Filter, Rejection, Reply};


use crate::infra::{routes::user_routes::user_routes, db::DB};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserReponse {
    #[serde(default)]
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let user_routes = user_routes::make(db.clone());

    let routes = user_routes.recover(error::handle_rejection);

    print!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

pub async fn users_list_handler(db: DB) -> WebResult<impl Reply> {
    let user_repository = UserRepository::new(db.database.collection::<User>("user"));
    let result = user_repository.list().await.map_err(|e| reject::custom(e))?;
    Ok(json(&result))
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

