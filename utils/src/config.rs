use std::env;
use std::path::Path;
use clap::Parser;
use serde::Deserialize;

/// Struct to hold environment variables loaded from .env
#[derive(Clone, Parser, Deserialize)]
pub struct Config {
    // Session cookie required to download a day's data
    #[clap(long, env)]
    pub session_cookie: String,
}

impl Config {
    pub fn from_env() -> Config {
        // Get the directory of the Cargo manifest
        let binding = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");
        let workspace_member_dir = Path::new(&binding);

        // Get the parent directory (workspace root)
        let workspace_root = workspace_member_dir.parent().expect("Unable to determine workspace root");

        // Load environment variables from the .env file in the workspace root
        dotenvy::from_path(workspace_root.join(".env")).expect("Failed to load .env file");

        // Parse environment variables into the Config struct
        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("Missing env vars: {error:#?}"),
        }
    }
}
