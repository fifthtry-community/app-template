use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask", version, about = "fastn xtask CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewApp { name: String },
    BuildWasm,
    RunTemplate,
    OptimiseWasm,
    PublishApp,
    UpdateUi,
    RunUi,
    UpdateWww,
    RunWww,
    UpdateTemplate,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::NewApp { name } => fastn_xtask::new_app(&name).await,
        Commands::BuildWasm => fastn_xtask::build_wasm(),
        Commands::RunTemplate => fastn_xtask::run_template(),
        Commands::OptimiseWasm => fastn_xtask::optimise_wasm(),
        Commands::PublishApp => fastn_xtask::publish_app(),
        Commands::UpdateUi => fastn_xtask::update_ui(),
        Commands::RunUi => fastn_xtask::run_ui(),
        Commands::UpdateWww => fastn_xtask::update_www(),
        Commands::RunWww => fastn_xtask::run_www(),
        Commands::UpdateTemplate => fastn_xtask::update_template(),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
