use self_update::{self, backends::github::Update, cargo_crate_version};
use semver::Version;
use std::env;
use anyhow::Context;

pub const CURRENT_VERSION: &str = cargo_crate_version!();

const REPO_OWNER: &str = "YourAverageLink";
const REPO_NAME: &str = "ssgz";
const BIN_NAME: &str = "ssgz";

fn get_release_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        "macos_intel"
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        "macos_apple_silicon"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

// Returns asset name if an update is available, otherwise None
pub fn check_for_update() -> anyhow::Result<Option<String>> {
    if cfg!(debug_assertions) {
        println!("Running in debug mode, skipping update check.");
        return Ok(None);
    }

    let platform = get_release_platform();

    let update = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .show_download_progress(true)
        .current_version(CURRENT_VERSION)
        .build()?;

    let release = update.get_latest_release()?;

    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name.contains(platform))
        .with_context(|| format!("Failed to find release for {}", platform))?;

    let latest_version =
        Version::parse(&release.version).context("Failed to parse latest version from GitHub")?;

    let current_version =
        Version::parse(CURRENT_VERSION).context("Failed to parse current version")?;

    if latest_version <= current_version {
        // already up to date
        return Ok(None);
    }

    return Ok(Some(asset.name));
}

pub fn perform_update(asset_name: &str) -> anyhow::Result<()> {
    let _status = Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .show_output(true)
        .no_confirm(true)
        .target(asset_name)
        .current_version(&CURRENT_VERSION.to_string())
        .build()
        .context("Failed to configure self-update for actual download")?
        .update()
        .context("Update failed")?;
    Ok(())
}