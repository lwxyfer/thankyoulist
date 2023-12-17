use reqwest::{header, Client};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
struct DependencyInfo {
    name: String,
    version: String,
    description: Option<String>,
    url: Option<String>,
    license: Option<String>,
}

// 定义一个模块级的 Client 实例
lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .build()
        .expect("Failed to create reqwest client");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 { &args[1] } else { "." };

    let package_json_path = format!("{}/package.json", directory);
    let cargo_toml_path = format!("{}/Cargo.toml", directory);

    if fs::metadata(&package_json_path).is_ok() {
        let package_json = fs::read_to_string(package_json_path)?;
        let package_info: serde_json::Value = serde_json::from_str(&package_json)?;

        if let Some(dependencies) = package_info.get("dependencies") {
            if let serde_json::Value::Object(dependencies) = dependencies {
                let mut dependency_list = Vec::new();
                let mut count: i32 = 0;

                for (name, value) in dependencies {
                    let version = value.as_str().unwrap_or("").to_string();

                    if let Some((description, url, license)) = get_npm_package_info(&name).await {
                        println!("Count: {}/{}", count, dependencies.len());
                        count += 1;

                        let dependency_info = DependencyInfo {
                            name: name.to_owned(),
                            version,
                            description: Some(description),
                            url: Some(url),
                            license: Some(license),
                        };
                        dependency_list.push(dependency_info);
                    } else {
                        // 获取信息失败
                        println!("Failed to get package information.");
                    }
                }

                let json = serde_json::to_string_pretty(&dependency_list)?;
                println!("{}", json);

                let mut file = File::create("thankyoulist.json")?;
                file.write_all(json.as_bytes())?;
            }
        }
    } else if fs::metadata(&cargo_toml_path).is_ok() {
        let cargo_toml = fs::read_to_string(cargo_toml_path)?;
        let cargo_info: Value = cargo_toml.parse()?;
        if let Some(dependencies) = cargo_info.get("dependencies") {
            if let Value::Table(dependencies) = dependencies {
                let mut dependency_list = Vec::new();
                for (name, value) in dependencies {
                    if let Value::String(version) = value {
                        if let Some((description, url, license)) = get_crate_info(&name).await {
                            let dependency_info = DependencyInfo {
                                name: name.to_owned(),
                                version: version.to_owned(),
                                description: Some(description),
                                url: Some(url),
                                license: Some(license),
                            };
                            dependency_list.push(dependency_info);
                        } else {
                            // 获取信息失败
                            println!("Failed to get package information.");
                        }
                    }
                }
                let json = serde_json::to_string_pretty(&dependency_list)?;
                println!("{}", json);

                let mut file = File::create("thankyoulist.json")?;
                file.write_all(json.as_bytes())?;
            }
        }
    } else {
        eprintln!("No package.json or Cargo.toml found in the specified directory.");
    }

    Ok(())
}

async fn get_npm_package_info(package_name: &str) -> Option<(String, String, String)> {
    let url = format!("https://registry.npmjs.org/{}", package_name);
    let response = reqwest::get(&url).await.ok()?;
    let body = response.text().await.ok()?;
    let json: serde_json::Value = serde_json::from_str(&body).ok()?;

    let description = json["description"].as_str()?.to_owned();
    let url = json["homepage"].as_str()?.to_owned();
    let license = json["license"].as_str()?.to_owned();

    Some((description, url, license))
}

async fn get_crate_info(package_name: &str) -> Option<(String, String, String)> {
    let url = format!("https://crates.io/api/v1/crates/{}", package_name);

    let response = CLIENT.get(&url).send().await.ok()?;
    let body = response.text().await.ok()?;
    let json: serde_json::Value = serde_json::from_str(&body).ok()?;

    let crate_info = json["crate"].as_object().to_owned();

    let description = json["crate"]
        .get("description")
        .and_then(|d| d.as_str())
        .map(|d| d.to_owned())
        .unwrap_or(String::new());
    let url = json["crate"]
        .get("homepage")
        .and_then(|d| d.as_str())
        .map(|d| d.to_owned())
        .unwrap_or(String::new());
    let license = crate_info
        .and_then(|info| info.get("license"))
        .map(|value| value.to_string())
        .unwrap_or(String::new());

    Some((description, url, license))
}
