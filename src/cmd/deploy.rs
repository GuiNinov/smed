use clap::ArgMatches;

use crate::cmd::terraform::TerraformClient;
use crate::cmd::kube_manager::KubeManager;

pub fn handle(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // let env = args.get_one::<String>("env").unwrap();

    let terraform_directory = args.get_one::<String>("terraform-directory").unwrap();

    let output = TerraformClient::apply(terraform_directory)?;

    let common_token = "my-manual-token";

    KubeManager::setup_rancher_cluster(&output, &common_token)?;

    KubeManager::setup_etcd_cluster(&output, &common_token)?;

    KubeManager::setup_control_plane_cluster(&output, &common_token)?;

    Ok(())
}