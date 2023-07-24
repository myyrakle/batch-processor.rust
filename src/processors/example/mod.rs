use super::IProcessor;

pub struct ExampleProcessor {}

impl ExampleProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IProcessor for ExampleProcessor {
    async fn process(&mut self) {
        println!("Hello, world!");
    }
}
