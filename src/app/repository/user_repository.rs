pub mod repository {
    use async_trait::async_trait;

    use crate::{Result, app::domain::user::User};
    
    #[async_trait]
    pub trait IUserRepository {
        async fn store(&mut self, user: &mut User)  -> Result<()>;
        async fn list(&self) -> Result<Vec<User>>;
    }
}
