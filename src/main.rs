mod cli;
mod cmd;
mod config;

fn main() {    
    let cli = cli::build_cli().get_matches();


    if let Err(e) = cmd::run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}