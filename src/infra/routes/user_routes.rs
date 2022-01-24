pub mod user_routes {
    use warp::{Filter, filters::BoxedFilter, Reply};

    use crate::{with_db, users_list_handler, infra::db::DB};

    
    pub fn make(db: DB) -> BoxedFilter<(impl Reply,)> {
        return warp::path::end()
        .and(warp::get())
        .and(with_db(db))
        .and_then(users_list_handler).boxed()
    }
}