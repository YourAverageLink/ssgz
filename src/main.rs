mod dol;
mod hex_literal;
mod iso_tools;
mod patch_loader;
mod patcher;
mod paths;

use anyhow::Error;
use clap::Parser;
use dialoguer::Confirm;
use iso_tools::*;
use rfd::FileDialog;
use std::fs;

#[derive(Parser, Debug)]
#[clap(about = "Practice ROM Hack Patcher for Skyward Sword")]
struct Args {
    version: GameVersion,
}
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let version = args.version;
    assert!(version.is_supported()); // arg parser should only accept supported versions

    println!(
        "Starting SSGZ Patcher {0} for the {version} version",
        env!("CARGO_PKG_VERSION")
    );

    let extract_done = paths::extract_dol_exists(version);
    let dol_copied = paths::dol_copy_exists(version);
    let ext_path = paths::extract_path(version);

    if !(extract_done && dol_copied) {
        let ver_str = version.to_string();
        if !extract_done {
            println!(
                "Please provide a clean copy of the {ver_str} version to create a practice ROM."
            );
        } else {
            println!(
                "Couldn't find copy of original main.dol file. It is recommended to redo extraction for the {ver_str} version."
            );
        }
        let file = FileDialog::new()
            .set_title(format!("Select a clean {ver_str} ISO."))
            .add_filter("Game ISO", &["iso"])
            .set_directory("./")
            .pick_file()
            .expect("Must have chosen an iso file.");

        let mut extractor = WiiIsoExtractor::new(file, version)?;
        extractor.validate()?;
        fs::create_dir_all(&ext_path)?;
        extractor.extract_to(ext_path.clone())?;
        paths::copy_dol_after_extract(version)?;
    }

    patcher::do_gz_patches(version)?;

    let repack_iso = Confirm::new()
        .with_prompt("Patching done, do you want to write an output iso?")
        .interact()
        .unwrap();

    if repack_iso {
        let mut out_dir = FileDialog::new()
            .set_title("Choose a directory for the patched ISO.")
            .set_directory("./")
            .pick_folder()
            .expect("Must have chosen an output directory.");

        out_dir.push(version.iso_name());

        rebuild_from_directory(ext_path, out_dir)?;
    }

    println!(
        "All done, happy speedrunning! Press Z and C simultaneously to access practice menus!"
    );
    Ok(())
}
