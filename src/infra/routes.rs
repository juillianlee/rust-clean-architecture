pub mod user_routes;

pub mod routes {
    use crate::{
        error,
        infra::{db::DB},
    };
    use std::{convert::Infallible, env};
    use warp::{Filter, Rejection};

    pub type Result<T> = std::result::Result<T, error::Error>;
    pub type WebResult<T> = std::result::Result<T, Rejection>;

    pub async fn init() -> Result<()> {
        let db = DB::init().await?;

        // let user_routes = user_routes::make(db.clone());

        // let routes = user_routes.recover(error::handle_rejection);

        let port: u16 = env::var("PORT")
            .unwrap_or(String::from("8080"))
            .parse()
            .unwrap();

        println!("Started on port {}", port);
        // warp::serve(routes).run(([0, 0, 0, 0], port)).await;
        Ok(())
    }

    pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}
