use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("smed")
        .version("0.1.0")
        .about("Manages infrastructure")
        .subcommand(
            Command::new("init")
                .about("Initializes the config")
                .arg(
                    Arg::new("terraform-directory").short('t').long("terraform-directory").required(false).default_value("./terraform").help("The directory to use for the terraform files")
                )
                .arg(
                    Arg::new("env-path").short('e').long("env-path").required(false).default_value("./.env").help("The path to the environment file")
                )
                .arg(
                    Arg::new("provider").short('p').long("provider")
                    .required(false)
                    .default_value("aws")
                    .help("Which cloud provider to use - AWS, GCP or AZURE")
                )
                .arg(Arg::new("region").short('r').long("region").required(false).default_value("us-east-1").help("The region to use for the cloud provider"))
        )
        .subcommand(
            Command::new("deploy")
                .about("Deploys to the cloud")
                .arg(
                    Arg::new("terraform-directory").short('t').long("terraform-directory").required(false).default_value("./terraform").help("The directory to use for the terraform files")
                )
        )
}
