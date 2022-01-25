pub mod repository {
    use async_trait::async_trait;

    use crate::app::{
        domain::user::User, repository::user_repository::repository::IUserRepository,
    };
    use crate::Result;

    #[derive(Clone, Debug)]
    struct UserRepository {
        users: Vec<User>,
    }

    impl Default for UserRepository {
        fn default() -> UserRepository {
            UserRepository { users: Vec::new() }
        }
    }

    #[async_trait]
    impl IUserRepository for UserRepository {
        async fn store(&mut self) -> Result<Vec<User>> {
            self.users.push(User {
                id: String::from("iasjida"),
                name: String::from("iasjida"),
                email: String::from("iasjida"),
            });
            Ok(self.users.clone())
        }

        async fn list(self: &Self) -> Result<Vec<User>> {
            Ok(self.users.clone())
        }
    }
}
