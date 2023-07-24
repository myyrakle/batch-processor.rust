pub(crate) mod example;

#[async_trait::async_trait]
pub trait IProcessor {
    async fn process(&mut self);
}
