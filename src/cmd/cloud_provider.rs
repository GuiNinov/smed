use std::process::Command;
use std::fmt;
use std::str::FromStr;

use crate::config::Config;

#[derive(Debug)]
pub enum CloudProvider {
    AWS,
    GCP,
    AZURE,
}

impl fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            CloudProvider::AWS => "AWS",
            CloudProvider::GCP => "GCP",
            CloudProvider::AZURE => "Azure",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for CloudProvider {    
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aws" => Ok(CloudProvider::AWS),
            "gcp" => Ok(CloudProvider::GCP),
            "azure" => Ok(CloudProvider::AZURE),
            _ => Err(format!("Unknown cloud provider: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum CloudProviderRegion {
    UsEast1,
    UsEast2,
}

impl fmt::Display for CloudProviderRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            CloudProviderRegion::UsEast1 => "us-east-1",
            CloudProviderRegion::UsEast2 => "us-east-2",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for CloudProviderRegion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "us-east-1" => Ok(CloudProviderRegion::UsEast1),
            "us-east-2" => Ok(CloudProviderRegion::UsEast2),
            _ => Err(format!("Unknown region: {}", s)),
        }
    }
}
pub struct CloudProviderAuthParams {
    provider: CloudProvider,
    region: CloudProviderRegion,
}

impl CloudProviderAuthParams {
    pub fn new(provider: CloudProvider, region: CloudProviderRegion) -> Self {
        Self { provider, region }
    }
}

trait CloudProviderAuth {
    fn auth(&self, region: CloudProviderRegion, config: &Config) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn auth(params: CloudProviderAuthParams, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let result = match params.provider {
        CloudProvider::AWS => AwsCloudProviderAuth.auth(params.region, config),
        CloudProvider::GCP => GcpCloudProviderAuth.auth(params.region, config),
        CloudProvider::AZURE => AzureCloudProviderAuth.auth(params.region, config),
    };
    result
}

struct AwsCloudProviderAuth;
struct GcpCloudProviderAuth;
struct AzureCloudProviderAuth;

impl CloudProviderAuth for AwsCloudProviderAuth {
    fn auth(&self, region: CloudProviderRegion, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let set_region = Command::new("aws")
            .arg("configure")
            .arg("set")
            .arg("region")
            .arg(region.to_string())
            .arg("--")
            .output()?;

        let set_access_key = Command::new("aws")
            .arg("configure")
            .arg("set")
            .arg("aws_access_key_id")
            .arg(&config.aws_access_key)
            .arg("--")
            .output()?;

        let set_secret_key = Command::new("aws")
            .arg("configure")
            .arg("set")
            .arg("aws_secret_access_key")
            .arg(&config.aws_secret_key)
            .arg("--")
            .output()?;

        if set_region.status.success() && set_access_key.status.success() && set_secret_key.status.success() {
            println!("Authenticated to AWS in region: {:?}", region.to_string());
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&set_region.stderr);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, stderr)))
        }
    }
}

impl CloudProviderAuth for GcpCloudProviderAuth {
    fn auth(&self, region: CloudProviderRegion, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Authenticating to GCP in region: {:?} with config: {:?}", region, config);
        // Insert GCP-specific logic here
        Ok(())
    }
}

impl CloudProviderAuth for AzureCloudProviderAuth {
    fn auth(&self, region: CloudProviderRegion, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Authenticating to Azure in region: {:?} with config: {:?}", region, config);
        // Insert Azure-specific logic here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth() {
        let result = auth(CloudProviderAuthParams {
            provider: CloudProvider::AWS,
            region: CloudProviderRegion::UsEast1,
        }, &Config {
            aws_access_key: "test".to_string(),
            aws_secret_key: "test".to_string(),
        }).unwrap();
        assert_eq!(result, ());
    }
}