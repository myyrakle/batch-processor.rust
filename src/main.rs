mod command;
mod processors;

use command::Command;

#[tokio::main]
async fn main() {
    use clap::Parser;
    let args = Command::parse();

    let processor = args.value.processor;

    println!("# Processor Name: {}", processor);

    let start = std::time::Instant::now();

    {
        let Some(mut processor) = processors::get_processor(&processor) else {
            println!("# Processor not found");
            return;
        };

        processor.process().await;
    }

    let elapsed = start.elapsed();

    println!("# Elapsed: {:?}", elapsed);
}
