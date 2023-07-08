use crate::config::deserialize_url;
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct CaddyConfig {
    #[serde(deserialize_with = "deserialize_url")]
    url: Url,
}

#[derive(Debug, Deserialize)]
struct Writer {
    pub filename: String,
    pub output: String,
    pub roll_keep: i64,
    pub roll_local_time: bool,
}

#[derive(Debug, Deserialize)]
struct Encoder {
    pub format: String,
}

#[derive(Debug, Deserialize)]
struct File {
    pub encoder: Encoder,
    pub writer: Writer,
}

#[derive(Debug, Deserialize)]
struct Logs {
    pub file: File,
}

#[derive(Debug, Deserialize)]
struct Logging {
    pub logs: Logs,
}

#[derive(Debug, Deserialize)]
struct Issuer {
    pub ca: String,
    pub email: String,
    pub module: String,
    pub trusted_roots_pem_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TLSConfig {
    pub issuers: Vec<Issuer>,
    pub subjects: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Automation {
    pub policies: Vec<TLSConfig>,
}

#[derive(Debug, Deserialize)]
struct Tls {
    pub automation: Automation,
}

#[derive(Debug, Deserialize)]
struct Matcher {
    pub host: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Route {
    pub handle: serde_yaml::Value,
    #[serde(rename = "match")]
    pub matcher: Vec<Matcher>,
    pub terminal: bool,
}

#[derive(Debug, Deserialize)]
struct Server {
    pub listen: Vec<String>,
    pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Servers {
    pub srvs: Vec<Server>,
}

#[derive(Debug, Deserialize)]
struct Http {
    pub servers: Servers,
}

#[derive(Debug, Deserialize)]
struct Apps {
    pub http: Http,
    pub tls: Tls,
}

#[derive(Debug, Deserialize)]
struct Admin {
    pub listen: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    pub admin: Admin,
    pub apps: Apps,
    pub logging: Logging,
}
