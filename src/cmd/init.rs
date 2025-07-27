use clap::ArgMatches;
use std::str::FromStr;

use crate::cmd::terraform::TerraformClient;
use crate::cmd::cloud_provider;
use crate::config::Config;

pub struct InitCommand { }

impl InitCommand {

    pub fn new() -> InitCommand {
        return InitCommand { }
    }

    pub fn execute(&self, args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let env_path = args.get_one::<String>("env-path").unwrap();

        let config = Config::from_env(env_path).unwrap();

        let terraform_directory = args.get_one::<String>("terraform-directory").unwrap();   

        TerraformClient::check()?;
        TerraformClient::init(terraform_directory)?;

        let provider = args.get_one::<String>("provider").unwrap().to_lowercase();
        let region = args.get_one::<String>("region").unwrap().to_lowercase();

        let parsed_provider = cloud_provider::CloudProvider::from_str(&provider).unwrap();
        let parsed_region = cloud_provider::CloudProviderRegion::from_str(&region).unwrap();

        cloud_provider::auth(cloud_provider::CloudProviderAuthParams::new(parsed_provider, parsed_region), &config)?;

        Ok(())
    }
} 
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        // Create a test config with dummy values
        let config = Config {
            aws_access_key: "test_access_key".to_string(),
            aws_secret_key: "test_secret_key".to_string(),
        };
        let init_command = InitCommand::new(&config);
        
        assert_eq!(init_command.config.aws_access_key, "test_access_key");
        assert_eq!(init_command.config.aws_secret_key, "test_secret_key");
        
        // Note: To test the execute method, you would need to create mock ArgMatches
        // with the required "provider" and "region" arguments
    }   
}