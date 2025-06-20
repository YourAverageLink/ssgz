use std::collections::HashMap;
use serde::Deserialize;
use include_dir::{Dir, include_dir}; // Why is this one directory higher than include_bytes / include_str???

use crate::iso_tools::GameVersion;

// For custom rel info
macro_rules! embed_rel {
    ($version:literal) => {
        include_bytes!(concat!("../custom-rel/", $version, "/customNP.rel"))
    };
}

// For patch info
macro_rules! embed_patch_diffs {
    ($version:literal) => {
        include_str!(concat!("../asm/patch_diffs/", $version, "/ss_necessary_diff.txt"))
    };
}

pub struct PatchData {
    pub version: GameVersion,
    pub custom_rel: &'static [u8],
    pub patch_diffs: PatchDiffMap,
    pub practice_saves_dir: Dir<'static>,
}

#[derive(Debug, Deserialize)]
struct PatchDiffEntry {
    #[serde(rename = "Data")]
    data: Box<[u8]>,
}

#[derive(Deserialize, Debug)]
struct FullPatchList(pub HashMap<String, PatchDiffMapRaw>);

pub type PatchDiffMap = HashMap<u32, Box<[u8]>>;
type PatchDiffMapRaw = HashMap<u32, PatchDiffEntry>;

// TODO - don't like how this has to be manually done
// Compiling necessary patch data into the executable
pub fn get_patch_data(version: GameVersion) -> Option<PatchData> {
    match version {
        GameVersion::NTSC1_0 => Some(PatchData {
            version: GameVersion::NTSC1_0,
            custom_rel: embed_rel!("US"),
            patch_diffs: parse_diffs(embed_patch_diffs!("us")),
            practice_saves_dir: include_dir!("practice-saves/US/saves/"),
        }),
        GameVersion::JP => Some(PatchData {
            version: GameVersion::JP,
            custom_rel: embed_rel!("JP"),
            patch_diffs: parse_diffs(embed_patch_diffs!("jp")),
            practice_saves_dir: include_dir!("practice-saves/JP/saves/"),
        }),
        _ => None,
    }
}

fn parse_diffs(raw_diff_str: &'static str) -> PatchDiffMap {
    let full_pd: FullPatchList = serde_yml::from_str(raw_diff_str).unwrap();
    full_pd.0.get("main.dol")
        .unwrap()
        .iter()
        .map(|(addr, patch)| (addr.clone(), patch.data.clone()))
        .collect()
}