use std::collections::HashMap;
use std::path::Path;
use std::{fs, process::Command};
use std::io;

use tera::{Tera, Context};
use serde::Deserialize;
use serde_json;
use std::fmt;

pub struct TerraformClient;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TerraformValue {
    String {
        value: String,
    },
    List {
        value: Vec<String>,
    },
}

impl fmt::Display for TerraformValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerraformValue::String { value } => write!(f, "{}", value),
            TerraformValue::List { value } => {
                let joined = value.join(", ");
                write!(f, "[{}]", joined)
            }
        }
    }
}

pub type TerraformOutput = HashMap<String, TerraformValue>;

impl TerraformClient {
    pub fn check() -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("terraform")
        .arg("-version")
        .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    println!("\x1b[32m✔ Terraform is installed:\x1b[0m\n{}", version);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("\x1b[31m✖ Terraform exists but returned an error:\x1b[0m\n{}", stderr);
                    Err("Terraform command returned an error".into())
                }
            }

            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                eprintln!("\x1b[31m✖ Terraform is not installed or not in PATH\x1b[0m");
                Err("Terraform binary not found".into())
            }

            Err(e) => {
                eprintln!("\x1b[31m✖ Failed to check Terraform version:\x1b[0m {}", e);
                Err(e.into())
            }
        }
    }
   
    pub fn init(terraform_directory: &str) -> Result<(), Box<dyn std::error::Error>> {   
        let init_output = Command::new("cd")
        .arg(terraform_directory)
        .arg("&&")
        .arg("terraform")
        .arg("init")
        .output()?;
    
        if init_output.status.success() {
            println!("\x1b[32m✔ Terraform initialized successfully\x1b[0m");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&init_output.stderr);
            eprintln!("\x1b[31m✖ Terraform init failed:\x1b[0m\n{}", stderr);
            Err("Terraform init failed".into())
        }
    }
    
    fn generate_main_tf(
        template_path: &str,
        output_path: &Path,
        variables: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tera = Tera::new(template_path)?; // e.g. "templates/*.tf.tera"
        let mut context = Context::new();

        for (k, v) in variables {
            context.insert(String::from(k), &v);
        }

        let rendered = tera.render("main.tf.tera", &context)?;
        fs::create_dir_all(output_path)?;
        fs::write(output_path.join("main.tf"), rendered)?;

        println!("\x1b[32m✔ Terraform file generated at {}\x1b[0m", output_path.display());
        Ok(())
    }

    pub fn apply(terraform_directory: &str) -> Result<TerraformOutput, Box<dyn std::error::Error>> {
        let vars = Self::build_apply_vars();

        Self::generate_main_tf("src/templates/*.tf.tera", Path::new(terraform_directory), &vars)?;

        Self::run_apply_command(terraform_directory)?;

        let output = Self::get_output_ips(terraform_directory)?;

        Ok(output)
    }

    fn build_apply_vars() -> HashMap<String, String> {
        let mut vars = HashMap::new();

        vars.insert(String::from("worker_count"), String::from("2"));    

        vars
    }

    fn run_apply_command(terraform_directory: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\x1b[34m🌍 Applying Terraform configuration...\x1b[0m");
        
        let apply_output = Command::new("terraform")
        .arg("apply")
        .arg("-auto-approve")
        .current_dir(terraform_directory)
        .output()?;

        if apply_output.status.success() {
            println!("\x1b[32m✔ Terraform applied successfully\x1b[0m");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&apply_output.stderr);
            eprintln!("\x1b[31m✖ Terraform apply failed:\x1b[0m\n{}", stderr);
            Err("Terraform apply failed".into())
        }

    }

    fn get_output_ips(terraform_directory: &str) -> Result<TerraformOutput, Box<dyn std::error::Error>> {
        println!("\x1b[34m🌍 Getting output IPs...\x1b[0m");

        let output = Command::new("terraform")
        .arg("output")
        .arg("-json")
        .current_dir(terraform_directory)
        .output()?;

        let output = String::from_utf8_lossy(&output.stdout);

        let parsed: TerraformOutput = serde_json::from_str(&output)?;

        println!("Output IPs: {:?}", parsed);

        Ok(parsed)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_terraform_version() {
        let result = check().unwrap();
        assert_eq!(result, ());
    }

}