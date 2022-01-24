pub mod repository {
    use crate::{app::repository::user_repository::repository::IUserRepository as AppUserRepository, Result, User, infra::repository::repository::repository::Repository};
    use async_trait::async_trait;
    use futures::stream::TryStreamExt;
    use mongodb::Collection;

    #[derive(Clone, Debug)]
    pub struct UserRepository {
        collection: Collection<User>,
    }

   

    #[async_trait]
    impl Repository<User> for UserRepository {
        fn new(collection: Collection<User>) -> Self {
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
                result.push(user);
            }

            Ok(result)
        }

        async fn list(self: &Self) -> Result<Vec<User>> {
            let mut result: Vec<User> = Vec::new();

            let mut cursor = self.collection.find(None, None).await?;
            while let Some(user) = cursor.try_next().await? {
                result.push(user);
            }

            Ok(result)
        }
    }

}
