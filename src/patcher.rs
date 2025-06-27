use std::fs;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

use crate::dol::Dol;
use crate::iso_tools::GameVersion;
use crate::patch_loader::{PatchData, get_patch_data};
use crate::paths::{
    custom_rel_path, extract_practice_saves_path, modified_dol_path, original_dol_path,
};

pub fn do_gz_patches(version: GameVersion) -> anyhow::Result<()> {
    // Load necessary patch data for this version
    let patch_data = get_patch_data(version).unwrap();
    let og_dol: Vec<u8> = fs::read(&original_dol_path(version))?;
    let mut dol = Dol::new(og_dol);

    println!("Patching main.dol...");
    patch_dol(&mut dol, &patch_data)?;
    dol.save_changes();

    let mut dest_dol = OpenOptions::new()
        .truncate(true)
        .read(true)
        .write(true)
        .create(true)
        .open(&modified_dol_path(patch_data.version))?;

    dest_dol.write(dol.data.as_slice())?;

    println!("Copying practice saves...");
    copy_practice_saves(&patch_data)?;

    println!("Copying custom REL file...");
    copy_custom_rel(&patch_data)?;

    Ok(())
}

fn copy_practice_saves(patch_data: &PatchData) -> anyhow::Result<()> {
    let target_path = extract_practice_saves_path(patch_data.version);
    // Only want to copy wiiking2.sav, skip.dat, and banner.bin
    let file_types = ["wiiking2.sav", "skip.dat", "banner.bin"];
    let mut files_to_copy = Vec::new();

    for filename in file_types {
        let pattern = format!("**/{}", filename);
        files_to_copy.extend(
            patch_data
                .practice_saves_dir
                .find(&pattern)?
                .filter_map(|e| e.as_file()),
        );
    }

    for file in files_to_copy {
        let target_file = target_path.join(file.path());

        // Create save parent directory if needed
        if let Some(parent) = target_file.parent() {
            create_dir_all(parent)?;
        }

        fs::write(target_file, file.contents())?;
    }

    Ok(())
}

fn copy_custom_rel(patch_data: &PatchData) -> anyhow::Result<usize> {
    let mut dest_file = OpenOptions::new()
        .truncate(true)
        .read(true)
        .write(true)
        .create(true)
        .open(&custom_rel_path(patch_data.version))?;

    Ok(dest_file.write(patch_data.custom_rel)?)
}

fn get_free_space_address(version: GameVersion) -> Option<u32> {
    match version {
        GameVersion::NTSC1_0 => Some(0x806782C0),
        GameVersion::JP => Some(0x8067B540),
        _ => None,
    }
}

fn get_dol_size(version: GameVersion) -> Option<u32> {
    match version {
        GameVersion::NTSC1_0 => Some(0x57A680),
        GameVersion::JP => Some(0x57D8C0),
        _ => None,
    }
}

fn get_thread_stack_update_end_locations(version: GameVersion) -> Option<[u32; 2]> {
    match version {
        GameVersion::NTSC1_0 => Some([0x803AC480, 0x803AC48C]),
        GameVersion::JP => Some([0x803ACDD0, 0x803ACDDC]),
        _ => None,
    }
}

fn get_thread_stack_update_start_locations(version: GameVersion) -> Option<[[u32; 2]; 3]> {
    match version {
        GameVersion::NTSC1_0 => Some([
            [0x803AC47C, 0x803AC484],
            [0x803A2988, 0x803A2990],
            [0x803A2AF0, 0x803A2AF4],
        ]),
        GameVersion::JP => Some([
            [0x803ACDCC, 0x803ACDD4],
            [0x803A32D8, 0x803A32E0],
            [0x803A3440, 0x803A3444],
        ]),
        _ => None,
    }
}

fn split_pointer_hi_lo(pointer: u32) -> (u32, u32) {
    let mut high_halfword = (pointer & 0xFFFF0000) >> 16;
    let low_halfword = pointer & 0xFFFF;
    if low_halfword >= 0x8000 {
        // If the low halfword has the highest bit set, it will be considered a negative number.
        // Therefore we need to add 1 to the high halfword (equivalent to adding 0x10000) to compensate for the low halfword being negated.
        high_halfword += 1;
    }

    (high_halfword, low_halfword)
}

fn patch_dol(dol: &mut Dol, patch_data: &PatchData) -> anyhow::Result<()> {
    let free_space_start = get_free_space_address(patch_data.version)
        .expect("Supported version must have free space addr defined to patch.");

    for (org_address, patch_bytes) in patch_data.patch_diffs.iter() {
        if *org_address >= free_space_start {
            add_free_space_section(dol, patch_bytes, patch_data.version)?;
        } else {
            dol.write_data_bytes(*org_address, patch_bytes)?;
        }
    }

    Ok(())
}

fn add_free_space_section(dol: &mut Dol, bytes: &[u8], version: GameVersion) -> anyhow::Result<()> {
    // Need to make a copy and modify it to satisfy the borrow checker
    let mut dol_section = dol.sections[2];
    let patch_length = bytes.len() as u32;
    assert!(dol_section.size == 0);

    // First, add a new text section to the dol (Text2).
    dol_section.offset =
        get_dol_size(version).expect("Supported version must have dol size defined to patch.");
    dol_section.address = get_free_space_address(version)
        .expect("Supported version must have free space addr defined to patch.");
    dol_section.size = patch_length;

    dol.sections[2] = dol_section;
    // Write custom code to the end of the dol
    dol.write_data_bytes(dol_section.address, bytes)?;

    let padded_patch_length = (patch_length + 3) & 0xFFFFFFFC;
    let new_start_ptr = dol.sections[2].address + padded_patch_length;
    let (high_halfword, low_halfword) = split_pointer_hi_lo(new_start_ptr);
    let thread_stack_update_start_locations = get_thread_stack_update_start_locations(version)
        .expect(
            "Supported version must have thread stack update start locations defined to patch.",
        );
    let thread_stack_update_end_locations = get_thread_stack_update_end_locations(version)
        .expect("Supported version must have thread stack update end locations defined to patch.");

    dol.write_data_u32(
        thread_stack_update_end_locations[0],
        0x3CA00000 | high_halfword,
    )?;
    dol.write_data_u32(
        thread_stack_update_end_locations[1],
        0x38A50000 | low_halfword,
    )?;

    // More hardcoded pointers that come later
    let new_end_ptr = new_start_ptr + 0x10000;
    let (high_halfword, low_halfword) = split_pointer_hi_lo(new_end_ptr);
    for loc_row in &thread_stack_update_start_locations {
        dol.write_data_u32(loc_row[0], 0x3C600000 | high_halfword)?;
        dol.write_data_u32(loc_row[1], 0x38630000 | low_halfword)?;
    }

    let high_halfword = (new_end_ptr & 0xFFFF0000) >> 16;
    let low_halfword = new_end_ptr & 0xFFFF;
    dol.write_data_u32(0x80004284, 0x3C200000 | high_halfword)?;
    dol.write_data_u32(0x80004288, 0x60210000 | low_halfword)?;

    Ok(())
}
