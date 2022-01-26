pub mod account {
    use crate::app::{
        domain::user::User, repository::user_repository::repository::IUserRepository,
    };
    use crate::Result;
    pub struct SignUp {
        pub repostiory: Box<dyn IUserRepository>,
    }

    impl SignUp {
        pub async fn execute(&mut self, mut user: User) -> Result<User> {
            let result = self.repostiory.store(&mut user).await;
            if result.is_ok() {
                println!("User register successfully");
            }
            Ok(user.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::account::SignUp;
    use crate::app::{
        domain::user::User, repository::memory::user_repository::repository::UserRepository,
    };

    #[tokio::test]
    async fn test_sign_up_usercase() {
        let r = UserRepository::default();
        let mut usecase = SignUp {
            repostiory: Box::new(r),
        };
        let user_created = usecase
            .execute(User {
                id: String::from(""),
                name: String::from("Teste"),
                email: String::from("Teste"),
            })
            .await;

        assert_eq!(user_created.is_ok(), true);
        let u = user_created.unwrap();
        assert_ne!(u.id, "");
        assert_eq!(u.name, "Teste");
        assert_eq!(u.email, "Teste");
    }
}
