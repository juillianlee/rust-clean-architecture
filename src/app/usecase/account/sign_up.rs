pub mod account {
    use crate::app::repository::user_repository::repository::IUserRepository;

    pub struct SignUp {
        repostiory: dyn IUserRepository,
    }

    impl SignUp {
        pub fn execute(self: &Self) {}
    }
}


#[cfg(test)]
mod test {
    use super::account::SignUp;


    #[test]
    fn test_sign_up_usercase() {
    }
}