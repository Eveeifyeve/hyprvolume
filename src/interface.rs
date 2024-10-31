use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    SetVolume {
        #[clap(long, short)]
        volume: String,

        #[clap(flatten)]
        allargs: AllArgs
    },
    MuteVolume {
        #[clap(flatten)]
        allargs: AllArgs
    }, 
    GenerateExampleConfig {}, 
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Selector {
    Output,
    Input,
}

#[derive(Args, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct AllArgs {
    #[clap(long, short)]
    pub notify: Option<String>,
    #[clap(long, short, value_enum)]
    pub select: Selector
}
