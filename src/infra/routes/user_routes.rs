pub mod user_routes {
    use warp::{Filter, filters::BoxedFilter, Reply};

    use crate::{infra::{db::DB, handlers::user::user_list_handler::user_handler::users_list_handler, routes::routes::with_db}};
    
    pub fn make(db: DB) -> BoxedFilter<(impl Reply,)> {
        return warp::path::end()
        .and(warp::get())
        .and(with_db(db))
        .and_then(users_list_handler).boxed()
    }
}