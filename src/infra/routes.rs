pub mod user_routes;

pub mod routes {
    use std::convert::Infallible;
    use warp::{Rejection, Filter};
    use crate::{infra::{db::DB, routes::user_routes::user_routes}, error};

    pub type Result<T> = std::result::Result<T, error::Error>;
    pub type WebResult<T> = std::result::Result<T, Rejection>;

    pub async fn init() -> Result<()> {
        let db = DB::init().await?;

        let user_routes = user_routes::make(db.clone());

        let routes = user_routes.recover(error::handle_rejection);

        print!("Started on port 8080");
        warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
        Ok(())
    }

    pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}
