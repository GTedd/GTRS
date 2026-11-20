use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "resources/jassets/"]
pub struct Resources;

/// Environment variable naming a locally built `patchbukkit.jar` that replaces the
/// embedded one at startup. For development only: it lets Java-side changes be tested
/// without rebuilding the native library.
pub const DEV_JAR_ENV: &str = "GTRS_BRIDGE_JAR";

pub fn setup_resources(resources_folder: &Path) -> Result<()> {
    sync_embedded_resources(resources_folder)?;
    cleanup_stale_files(resources_folder);
    apply_dev_jar_override(resources_folder)?;

    Ok(())
}

fn apply_dev_jar_override(resources_folder: &Path) -> Result<()> {
    let Some(dev_jar) = std::env::var_os(DEV_JAR_ENV).filter(|v| !v.is_empty()) else {
        return Ok(());
    };
    let dev_jar = PathBuf::from(dev_jar);
    if !dev_jar.is_file() {
        anyhow::bail!(
            "{DEV_JAR_ENV} is set to '{}', which is not a file",
            dev_jar.display()
        );
    }
    let target = resources_folder.join("patchbukkit.jar");
    fs::copy(&dev_jar, &target)?;
    tracing::warn!(
        "Using development bridge JAR from {} instead of the embedded one",
        dev_jar.display()
    );
    Ok(())
}

fn cleanup_stale_files(resources_folder: &Path) {
    let embedded_paths: HashSet<PathBuf> = Resources::iter()
        .map(|p| PathBuf::from(p.to_string()))
        .collect();

    for entry in walkdir::WalkDir::new(resources_folder)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Ok(rel_path) = path.strip_prefix(resources_folder)
            && !embedded_paths.contains(rel_path)
        {
            tracing::warn!("Removing stale embedded file: {}", rel_path.display());
            let _ = fs::remove_file(path);
        }
    }

    // Clean up any empty subdirectories in resources_folder
    for entry in walkdir::WalkDir::new(resources_folder)
        .contents_first(true)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_dir() && e.path() != resources_folder)
    {
        let _ = fs::remove_dir(entry.path());
    }
}

pub fn sync_embedded_resources(resources_folder: &Path) -> Result<()> {
    for resource_path_str in Resources::iter() {
        let resource_path = resources_folder.join(resource_path_str.to_string());
        let resource = Resources::get(&resource_path_str).unwrap();

        if resource_path.exists() {
            update_resource_if_changed(&resource_path, &resource.data)?;
        } else {
            tracing::info!("Extracting new resource: {}", resource_path.display());
            write_resource(&resource_path, &resource.data)?;
        }
    }
    Ok(())
}

fn write_resource(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, data)?;
    Ok(())
}

fn update_resource_if_changed(path: &Path, new_data: &[u8]) -> Result<()> {
    // Quick check: If file sizes differ, it's definitely changed, TODO: use Hash ?
    let metadata = fs::metadata(path).ok();
    let size_matches = metadata.is_some_and(|m| m.len() == new_data.len() as u64);

    if !size_matches || fs::read(path)? != new_data {
        tracing::debug!("Updating changed resource: {}", path.display());
        fs::write(path, new_data)?;
    }

    Ok(())
}
