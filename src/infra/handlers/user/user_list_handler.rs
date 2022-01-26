
// pub mod user_handler {
//     use warp::{Reply, reply::json, reject};

//     use crate::{infra::{repository::{mongo::{user_repository::repository::UserRepository, model::user::User}, repository::repository::Repository}, db::DB, routes::routes::WebResult}, app::repository::user_repository::repository::IUserRepository};

//     pub async fn users_list_handler(db: DB) -> WebResult<impl Reply> {
//         let user_repository = UserRepository::new(db.database.collection::<User>("user"));
//         let result = user_repository.list().await.map_err(|e| reject::custom(e))?;
//         Ok(json(&result))
//     }
// }