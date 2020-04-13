use failure::Error;
use semver::Version;

use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub config: UpgrenadeConfig,
    pub version: Vec<VersionEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionEntry {
    pub name: String,
    pub link: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpgrenadeConfig {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: Option<u16>,
}

fn default_host() -> String {
    "0.0.0.0".to_owned()
}

impl Config {
    pub fn version_map(&self) -> Result<BTreeMap<Version, String>, Error> {
        self.version
            .iter()
            .cloned()
            .map(|entry| Ok((entry.name.parse()?, entry.link)))
            .collect()
    }
}
