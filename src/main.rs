mod dol;
mod gui;
mod hex_literal;
mod iso_tools;
mod patch_loader;
mod patcher;
mod paths;
mod updater;

use anyhow::Context;
use clap::Parser;
use dialoguer::Confirm;
use indicatif::ProgressBar;
use iso_tools::*;
use rfd::FileDialog;
use std::fs;
use updater::{CURRENT_VERSION, check_for_update, perform_update};

#[derive(Parser, Debug)]
#[clap(about = "Practice ROM Hack Patcher for Skyward Sword")]
#[clap(version = CURRENT_VERSION)]
struct Args {
    #[arg(long)]
    noui: bool,
    #[arg(requires = "noui")]
    game_version: Option<GameVersion>,
}

fn fix_macos_working_directory() -> anyhow::Result<()> {
    // If in a .app file, we need to fix working directory to the bundle's location
    // (unless running from source or with dx serve)
    #[cfg(target_os = "macos")]
    {
        use std::env;
        if let Ok(exe_path) = env::current_exe() {
            let mut current = exe_path.as_path();
            while let Some(parent) = current.parent() {
                if parent.extension().map_or(false, |ext| ext == "app") {
                    let app_parent = parent.parent().unwrap_or(parent);
                    let app_parent_str = app_parent.to_string_lossy();

                    // Keep current directory if testing with dx serve
                    if app_parent_str.contains("/target/dx") {
                        println!(
                            "Development .app detected (in target directory), keeping current working directory"
                        );
                        return Ok(());
                    }

                    env::set_current_dir(app_parent)?;
                    println!(
                        "Distributed .app bundle detected, working directory set to: {:?}",
                        app_parent
                    );
                    return Ok(());
                }
                current = parent;
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    fix_macos_working_directory()?;
    let args = Args::parse();
    if args.noui {
        if let Some(ver) = args.game_version {
            do_noui(ver)
        } else {
            panic!("When using --noui, you must specify a version.")
        }
    } else {
        gui::do_gui();
        Ok(())
    }
}

pub fn is_ready_to_patch(version: GameVersion) -> bool {
    paths::extract_dol_exists(version) && paths::dol_copy_exists(version)
}

fn do_noui(version: GameVersion) -> anyhow::Result<()> {
    match check_for_update() {
        Ok(Some(asset_name)) => {
            if Confirm::new()
                .with_prompt("Update detected. Do you want to update now?")
                .default(true)
                .interact()
                .context("Failed to read user input")?
            {
                perform_update(&asset_name)?;
                println!("Update complete! Please re-launch the app.");
                return Ok(());
            } else {
                println!("Update canceled.");
            }
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("Update check failed: {e}");
        }
    }

    assert!(version.is_supported()); // arg parser should only accept supported versions

    println!("Starting SSGZ Patcher {CURRENT_VERSION} for the {version} version");

    let extract_done = paths::extract_dol_exists(version);
    let dol_copied = paths::dol_copy_exists(version);

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

        do_extract_noui(version)?;
    }

    patcher::do_gz_patches(version)?;

    let repack_iso = Confirm::new()
        .with_prompt("Patching done, do you want to write an output iso?")
        .interact()
        .unwrap();

    if repack_iso {
        let bar = ProgressBar::new(100);
        bar.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{wide_bar:.cyan/blue}] {percent}% ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        do_repack(version, &mut |done_percent| {
            bar.set_position(done_percent as u64);
        })?;
        bar.finish_with_message("Rebuilding done.");
    }

    println!(
        "All done, happy speedrunning! Press Z and C simultaneously to access practice menus!"
    );
    Ok(())
}

fn do_extract_noui(version: GameVersion) -> anyhow::Result<()> {
    let ver_str = version.to_string();
    let file = FileDialog::new()
        .set_title(format!("Select a clean {ver_str} ISO."))
        .add_filter("Game ISO", &["iso"])
        .set_directory("./")
        .pick_file()
        .with_context(|| "Must have chosen an iso file.")?;

    // Attempt to extract iso and validate the correct version was given
    let mut extractor = WiiIsoExtractor::new_with_version(file, version)?;
    let ext_path = paths::extract_path(version);
    fs::create_dir_all(&ext_path)?;
    let total_bytes = extractor.size_of_extract()? as u64;
    // Use indicatif's ProgressBar to display progress in the terminal
    println!("Extracting files...");
    let bar = ProgressBar::new(total_bytes as u64);
    bar.set_style(
        indicatif::ProgressStyle::with_template(
            "{spinner:.green} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    extractor.extract_to(ext_path.clone(), &mut |done_bytes| {
        bar.set_position(done_bytes);
    })?;
    bar.finish_with_message("Extraction done.");
    paths::copy_dol_after_extract(version)?;

    Ok(())
}

fn do_extract_ui<T: FnMut(u8)>(version: GameVersion, callback: &mut T) -> anyhow::Result<()> {
    let ver_str = version.to_string();
    let file = FileDialog::new()
        .set_title(format!("Select a clean {ver_str} ISO."))
        .add_filter("Game ISO", &["iso"])
        .set_directory("./")
        .pick_file()
        .with_context(|| "Must have chosen an iso file.")?;

    // Attempt to extract iso and validate the correct version was given
    let mut extractor = WiiIsoExtractor::new_with_version(file, version)?;
    let ext_path = paths::extract_path(version);
    fs::create_dir_all(&ext_path)?;
    let total_bytes = extractor.size_of_extract()? as u64;
    // Here, callback operates on the extraction percentage rather than raw byte count
    extractor.extract_to(ext_path.clone(), &mut |done_bytes| {
        callback(((done_bytes * 100) / total_bytes) as u8);
    })?;
    paths::copy_dol_after_extract(version)?;

    Ok(())
}

fn do_repack<T: FnMut(u8)>(version: GameVersion, callback: &mut T) -> anyhow::Result<()> {
    let mut out_dir = FileDialog::new()
        .set_title("Choose a directory for the patched ISO.")
        .set_directory("./")
        .pick_folder()
        .with_context(|| "Must have chosen an output directory.")?;

    out_dir.push(version.iso_name());

    rebuild_from_directory(paths::extract_path(version), out_dir, &mut |done_percent| {
        callback(done_percent);
    })?;

    Ok(())
}
