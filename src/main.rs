mod command;

use command::Command;

#[tokio::main]
async fn main() {
    use clap::Parser;
    let args = Command::parse();

    let processor = args.value.processor;

    println!("# Processor Name: {}", processor);

    let start = std::time::Instant::now();

    {}

    let elapsed = start.elapsed();

    println!("# Elapsed: {:?}", elapsed);
}
