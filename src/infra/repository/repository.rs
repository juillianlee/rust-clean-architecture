pub mod repository {
    use async_trait::async_trait;
    use mongodb::Collection;

    #[async_trait]
    pub trait Repository<T> {
        fn new(collection: Collection<T>) -> Self;
    }
}
