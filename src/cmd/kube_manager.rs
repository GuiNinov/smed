use std::process::Command;

use crate::cmd::terraform::TerraformOutput;

pub struct KubeManager { }

struct SshCommand {
    command: String,
    description: String
}

impl KubeManager {
    pub fn setup_rancher_cluster(ips: &TerraformOutput, common_token: &str) -> Result<(), Box<dyn std::error::Error>> {

        println!("\n");
        println!("\x1b[36m🔧 Setting up Rancher cluster...\x1b[0m");


        let rancher_ip = ips.get("rancher_ip").unwrap().to_string();
        
        let commands = Self::get_rancher_commands(&rancher_ip, common_token);

        for c in commands {
            Self::run_ssh_command("ubuntu", &rancher_ip, "~/.ssh/id_rsa", &c.command, &c.description)?;
        }

        Ok(())
    }

    pub fn setup_etcd_cluster(ips: &TerraformOutput, common_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n");
        println!("\x1b[36m🔧 Setting up Etcd...\x1b[0m");
        
        let etcd_public_ip = ips.get("etcd_public_ip").unwrap().to_string();

        let commands = Self::get_etcd_commands(&etcd_public_ip, common_token);

        for c in commands {
            Self::run_ssh_command("ubuntu", &etcd_public_ip, "~/.ssh/id_rsa", &c.command, &c.description)?;
        }

        Ok(())
    }

    pub fn setup_control_plane_cluster(ips: &TerraformOutput, common_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n");
        println!("\x1b[36m🔧 Setting up Control Plane...\x1b[0m");
     
        let control_plane_ip = ips.get("control_plane_ip").unwrap().to_string();
        let etcd_private_ip = ips.get("etcd_private_ip").unwrap().to_string();

        let commands = Self::get_control_plane_commands(&control_plane_ip, &etcd_private_ip, common_token);

        for c in commands {
            Self::run_ssh_command("ubuntu", &control_plane_ip, "~/.ssh/id_rsa", &c.command, &c.description)?;
        }

        Ok(())
    }


    fn expand_tilde(path: &str) -> String {
        if path.starts_with("~") {
            let home = std::env::var("HOME").unwrap_or_default();
            path.replacen("~", &home, 1)
        } else {
            path.to_string()
        }
    }
    
    fn run_ssh_command(
        ssh_user: &str,
        target_ip: &str,
        key_path: &str,
        command: &str,
        description: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("\x1b[36m👉 {}\x1b[0m", command);
        println!("{}", description);

        let ssh_target = format!("{}@{}", ssh_user, target_ip);
        let key_path = Self::expand_tilde(key_path);

        let status = Command::new("ssh")
            .arg("-i")
            .arg(&key_path)
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg(&ssh_target)
            .arg(command)
            .status()?;

        if status.success() {
            println!("\x1b[32m✔ Success\x1b[0m\n");
            Ok(())
        } else {
            println!("\x1b[31m✖ Failed to run: {}\x1b[0m", command);
            Err(format!("Failed to run: {}", description).into())
        }
    }

    fn get_rancher_commands(rancher_ip: &str, common_token: &str) -> Vec<SshCommand> {
        return vec![
            SshCommand {
                command: "sudo sh -c 'curl -sfL https://get.rke2.io | INSTALL_RKE2_TYPE=\"server\" sh'".to_string(),
                description: "Install RKE2".to_string(),
            },
            SshCommand {
                command: "sudo mkdir -p /etc/rancher/rke2".to_string(),
                description: "Create RKE2 directory".to_string(),
            },
            SshCommand {
                command: format!("sudo tee /etc/rancher/rke2/config.yaml > /dev/null <<EOF
token: {}
tls-san:
    - {}
    - {}.sslip.io
node-taint: []
EOF", common_token, rancher_ip, rancher_ip).to_string(),
                description: "Create RKE2 config file".to_string(),
            },
            SshCommand {
                command: "sudo systemctl enable rke2-server".to_string(),
                description: "Enable RKE2 service".to_string(),
            },
            SshCommand {
                command: "sudo systemctl start rke2-server".to_string(),
                description: "Start RKE2 service".to_string(),
            },
            SshCommand {
                command: "sudo ln -s /var/lib/rancher/rke2/bin/kubectl /usr/local/bin/kubectl && sudo chmod +x /usr/local/bin/kubectl".to_string(),
                description: "Create symlink for kubectl".to_string(),
            },
            SshCommand {
                command: "export KUBECONFIG=/etc/rancher/rke2/rke2.yaml".to_string(),
                description: "Set KUBECONFIG".to_string(),
            },
        ];
    }

    fn get_etcd_commands(etcd_public_ip: &str, common_token: &str) -> Vec<SshCommand> {
        return vec![
            SshCommand {
                command: "sudo sh -c 'curl -sfL https://get.rke2.io | INSTALL_RKE2_TYPE=\"server\" sh'".to_string(),                
                description: "Install RKE2 server".to_string(),
            },
            SshCommand {
                command: "sudo mkdir -p /etc/rancher/rke2".to_string(),
                description: "Create RKE2 directory".to_string(),
            },
            SshCommand {
                command: format!("sudo tee /etc/rancher/rke2/config.yaml > /dev/null <<EOF
token: {}
tls-san:
    - {}
node-taint:
    - \"etcd=true:NoExecute\"
EOF", 
             common_token, etcd_public_ip).to_string(),
                description: "Create RKE2 config file".to_string(),
            },
            SshCommand {
                command: "sudo systemctl enable rke2-server".to_string(),
                description: "Enable RKE2 service".to_string(),
            },
            SshCommand {
                command: "sudo systemctl start rke2-server".to_string(),
                description: "Start RKE2 service".to_string(),
            },
            SshCommand {
                command: "sudo ln -s /var/lib/rancher/rke2/bin/kubectl /usr/local/bin/kubectl && sudo chmod +x /usr/local/bin/kubectl".to_string(),
                description: "Create symlink for kubectl".to_string(),
            },
            SshCommand {
                command: "export KUBECONFIG=/etc/rancher/rke2/rke2.yaml".to_string(),
                description: "Set KUBECONFIG".to_string(),
            },
        ];
    }

    fn get_control_plane_commands(control_plane_ip: &str, etcd_private_ip: &str, common_token: &str) -> Vec<SshCommand> {
        return vec![
            SshCommand {
                command: "sudo sh -c 'curl -sfL https://get.rke2.io | INSTALL_RKE2_TYPE=\"server\" sh'".to_string(),                
                description: "Install RKE2 server".to_string(),
            },
            SshCommand {
                command: "sudo mkdir -p /etc/rancher/rke2".to_string(),
                description: "Create RKE2 directory".to_string(),
            },
            SshCommand {
                command: format!("sudo tee /etc/rancher/rke2/config.yaml > /dev/null <<EOF
server: https://{}:9345
token: {}
tls-san:
    - {}
node-taint:
    - \"controlplane=true:NoExecute\"
EOF", etcd_private_ip, common_token, control_plane_ip).to_string(),
                description: "Create RKE2 config file".to_string(),
            },
            SshCommand {
                command: "sudo systemctl enable rke2-server".to_string(),
                description: "Enable RKE2 service".to_string(),
            },
            SshCommand {
                command: "sudo systemctl start rke2-server".to_string(),
                description: "Start RKE2 service".to_string(),
            },
            SshCommand {
                command: "sudo ln -s /var/lib/rancher/rke2/bin/kubectl /usr/local/bin/kubectl && sudo chmod +x /usr/local/bin/kubectl".to_string(),
                description: "Create symlink for kubectl".to_string(),
            },
            SshCommand {
                command: "export KUBECONFIG=/etc/rancher/rke2/rke2.yaml".to_string(),
                description: "Set KUBECONFIG".to_string(),
            },
        ];
    }

}