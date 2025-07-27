mod init;
mod deploy;
mod terraform;
mod cloud_provider;
mod kube_manager;

use clap::ArgMatches;

pub fn run(cli: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match cli.subcommand() {
        Some(("init", _)) => {
            let command = init::InitCommand::new();

            let args = cli.subcommand_matches("init").unwrap();

            command.execute(args)
        },
        Some(("deploy", args)) => deploy::handle(args),
        _ => Ok(()),
    }
}