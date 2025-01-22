mod error;
pub mod server;

use clap::{Args, Parser, Subcommand};

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Opt {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run deeplx
    Run(Bootstrap),
}

#[derive(Args, Clone)]
pub struct Bootstrap {
    /// Config path, eg: --conf ./configs
    #[clap(short, long, default_value = "configs")]
    conf: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    match opt.commands {
        Commands::Run(args) => server::run(args)?,
    };

    Ok(())
}
