pub mod repository {
    use std::process::id;

    use async_trait::async_trait;
    use uuid::Uuid;

    use crate::app::{
        domain::user::User, repository::user_repository::repository::IUserRepository,
    };
    use crate::Result;

    #[derive(Clone, Debug)]
    pub struct UserRepository {
        users: Vec<User>,
    }

    impl Default for UserRepository {
        fn default() -> UserRepository {
            UserRepository { users: Vec::new() }
        }
    }

    #[async_trait]
    impl IUserRepository for UserRepository {
        async fn store(&mut self, user: &mut User) -> Result<()> {
            user.id = Uuid::new_v4().to_string();
            self.users.push(user.clone());
            Ok(())
        }

        async fn list(self: &Self) -> Result<Vec<User>> {
            Ok(self.users.clone())
        }
    }
}
