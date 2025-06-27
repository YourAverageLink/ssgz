use const_format::formatcp;
use std::fs;
use std::path::{MAIN_SEPARATOR as SEP, PathBuf};

use crate::iso_tools::GameVersion;
const ROOT_PATH: &str = formatcp!(".{SEP}");
const BASE_EXTRACT_PATH: &str = formatcp!("{ROOT_PATH}extract");
const BASE_DOL_PATH: &str = formatcp!("{ROOT_PATH}original-dol");

pub fn original_dol_path(version: GameVersion) -> PathBuf {
    let version_str = version
        .path_name()
        .expect("Supported version must have path name");
    let path_str = format!("{BASE_DOL_PATH}{SEP}{version_str}{SEP}main.dol");
    PathBuf::from(path_str)
}

pub fn extract_path(version: GameVersion) -> PathBuf {
    let version_str = version
        .path_name()
        .expect("Supported version must have path name");
    let path_str = format!("{BASE_EXTRACT_PATH}{SEP}{version_str}");
    PathBuf::from(path_str)
}

pub fn modified_dol_path(version: GameVersion) -> PathBuf {
    let version_str = version
        .path_name()
        .expect("Supported version must have path name");
    let path_str = format!("{BASE_EXTRACT_PATH}{SEP}{version_str}{SEP}DATA{SEP}sys{SEP}main.dol");
    PathBuf::from(path_str)
}

pub fn custom_rel_path(version: GameVersion) -> PathBuf {
    let version_str = version
        .path_name()
        .expect("Supported version must have path name");
    let path_str = format!(
        "{BASE_EXTRACT_PATH}{SEP}{version_str}{SEP}DATA{SEP}files{SEP}rels{SEP}customNP.rel"
    );
    PathBuf::from(path_str)
}

pub fn extract_practice_saves_path(version: GameVersion) -> PathBuf {
    let version_str = version
        .path_name()
        .expect("Supported version must have path name");
    let path_str = format!("{BASE_EXTRACT_PATH}{SEP}{version_str}{SEP}DATA{SEP}files{SEP}saves");
    PathBuf::from(path_str)
}

pub fn extract_dol_exists(version: GameVersion) -> bool {
    modified_dol_path(version).exists()
}

pub fn dol_copy_exists(version: GameVersion) -> bool {
    original_dol_path(version).exists()
}

pub fn copy_dol_after_extract(version: GameVersion) -> anyhow::Result<()> {
    let src_path = modified_dol_path(version); // after the extract, this dol is still clean
    let dest_path = original_dol_path(version);
    fs::create_dir_all(dest_path.parent().unwrap())?;
    fs::copy(src_path, dest_path)?;
    Ok(())
}
