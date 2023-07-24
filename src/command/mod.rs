use serde::Deserialize;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOption,
}

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOption {
    #[clap(name = "processor", short, long)]
    pub processor: String,
}
