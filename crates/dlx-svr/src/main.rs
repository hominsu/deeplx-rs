mod run;
mod server;

use clap::{Args, Parser, Subcommand};

#[cfg(all(
    feature = "mimalloc",
    not(any(
        feature = "rpmalloc",
        feature = "snmalloc",
        feature = "tikv-jemallocator"
    ))
))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    feature = "rpmalloc",
    not(any(
        feature = "mimalloc",
        feature = "snmalloc",
        feature = "tikv-jemallocator"
    ))
))]
#[global_allocator]
static GLOBAL: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

#[cfg(all(
    feature = "snmalloc",
    not(any(
        feature = "mimalloc",
        feature = "rpmalloc",
        feature = "tikv-jemallocator"
    ))
))]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[cfg(all(
    feature = "tikv-jemallocator",
    not(any(feature = "mimalloc", feature = "rpmalloc", feature = "snmalloc"))
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

type Result<T, E = dlx::Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Opt {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run dlx-svr
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
        Commands::Run(args) => run::run(args)?,
    };

    Ok(())
}
