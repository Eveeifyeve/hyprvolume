use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    SetVolume {
        #[clap(flatten)]
        allargs: AllArgs
    },
    MuteVolume {
        #[clap(flatten)]
        allargs: AllArgs
    }, 
    GenerateExampleConfig {}, 
}


#[derive(Args, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct AllArgs {
    #[clap(long, short)]
    pub notify: Option<String> 
}
