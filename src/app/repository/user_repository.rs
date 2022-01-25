pub mod repository {
    use async_trait::async_trait;

    use crate::{Result, app::domain::user::User};
    
    #[async_trait]
    pub trait IUserRepository {
        async fn store(self: &Self)  -> Result<Vec<User>>;
        async fn list(self: &Self) -> Result<Vec<User>>;
    }
}
