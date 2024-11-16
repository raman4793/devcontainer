use bollard::Docker;
use std::path::PathBuf;

use clap::Parser;
mod attach;
mod build;
mod config;
mod run;
mod utils;

use crate::config::DEVCONTAINER_ROOT;

#[tokio::main]
async fn main() {
    let args = config::Cli::parse();
    let dev_container_spec_file = PathBuf::from(&DEVCONTAINER_ROOT).join("devcontainer.json");
    let client: Docker = Docker::connect_with_socket_defaults().unwrap();
    let dev_container_spec = utils::get_dev_container_spec(dev_container_spec_file);
    match args.command {
        config::Commands::Build => build::build(&dev_container_spec, &client).await,
        config::Commands::Run => run::run(&dev_container_spec, &client).await,
        config::Commands::Down => utils::down(&dev_container_spec, &client).await,
        config::Commands::Attach => attach::attach(&dev_container_spec, &client).await,
    }
}
