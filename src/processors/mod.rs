pub(crate) mod example;

#[async_trait::async_trait]
pub trait IProcessor {
    async fn process(&mut self);
}

pub fn get_processor(name: &str) -> Option<Box<dyn IProcessor>> {
    match name {
        "EXAMPLE" => Some(Box::new(example::ExampleProcessor::new())),
        _ => None,
    }
}
