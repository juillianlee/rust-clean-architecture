pub mod repository {
    use crate::{
        app::{repository::user_repository::repository::IUserRepository as AppUserRepository, domain::user::User},
        infra::repository::{mongo::model::user::User as UserModel, repository::repository::Repository},
        Result,
    };
    use async_trait::async_trait;
    use futures::stream::TryStreamExt;
    use mongodb::Collection;

    #[derive(Clone, Debug)]
    pub struct UserRepository {
        collection: Collection<UserModel>,
    }

    #[async_trait]
    impl Repository<UserModel> for UserRepository {
        fn new(collection: Collection<UserModel>) -> Self {
            let collection = collection;
            Self {
                collection: collection,
            }
        }
    }

    #[async_trait]
    impl AppUserRepository for UserRepository {
        async fn store(self: &Self) -> Result<Vec<User>> {
            let mut result: Vec<User> = Vec::new();
            let mut cursor = self.collection.find(None, None).await?;
            while let Some(user) = cursor.try_next().await? {
                let u = User {
                    id: user.id.to_hex(),
                    name: user.name,
                    email: user.email
                };
                result.push(u);
            }

            Ok(result)
        }

        async fn list(self: &Self) -> Result<Vec<User>> {
            let mut result: Vec<User> = Vec::new();
            let mut cursor = self.collection.find(None, None).await?;
            while let Some(user) = cursor.try_next().await? {
                let u = User {
                    id: user.id.to_hex(),
                    name: user.name,
                    email: user.email
                };
                result.push(u);
            }

            Ok(result)
        }
    }
}
