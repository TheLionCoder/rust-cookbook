use std::collections::HashMap;

use serde::Deserialize;
use toml::de::Error;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

impl Package {
    fn get_authors(&self) -> &Vec<String> {
        &self.authors
    }

}
pub fn deserialize_toml_config_file() -> Result<(), Error> {
    let toml_content: &str = r#"
        [package]
        name= "your_package"
        version= "0.1.0"
        authors= ["thelioncoder", "evoreyes@epssanitas.com"]

        [dependencies]
        serde = "1.0"
            "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.dependencies["serde"], "1.0");

    let authors:&Vec<String> = package_info.package.get_authors();
    assert_eq!(authors, &vec!["thelioncoder".to_string(),"evoreyes@epssanitas.com".to_string()]);

    Ok(())
}
