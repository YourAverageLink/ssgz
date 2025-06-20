// Port of disc_riider_py API (at least features necessary for gz)
use std::{
    fmt::{self},
    fs::{self, OpenOptions, create_dir_all},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use crate::hex;
use anyhow::Error;

use binrw::{BinWrite, BinWriterExt};
use disc_riider::{
    Fst, FstNode, WiiIsoReader, WiiPartitionReadInfo, builder::build_from_directory,
    structs::WiiPartType,
};
use indicatif::ProgressBar;
use sha1::{Digest, Sha1};
use clap::ValueEnum;

struct Section {
    part: String,
    fst: Fst,
    partition_reader: WiiPartitionReadInfo,
}

pub struct WiiIsoExtractor {
    iso: WiiIsoReader<fs::File>,
    partition: Section,
    version: GameVersion,
}

pub fn binrw_write_file(
    p: &Path,
    value: &impl for<'a> BinWrite<Args<'a> = ()>,
) -> Result<(), Error> {
    let mut f = fs::File::create(p)?;
    f.write_be(value)?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, ValueEnum)]
pub enum GameVersion {
    #[value(name = "us")]
    NTSC1_0,
    #[value(name = "jp")]
    JP,
    #[value(skip)] 
    NTSC1_2,
    #[value(skip)] 
    PAL1_0,
    #[value(skip)] 
    Unknown,
}

impl GameVersion {
    pub fn is_supported(&self) -> bool {
        match *self {
            GameVersion::NTSC1_0 => true,
            GameVersion::JP => true,
            _ => false,
        }
    }

    fn from_hash(hash: [u8; 20]) -> Self {
        const NTSC1_0_HASH: [u8; 20] = hex!("450a6806f46d59dcf8278db08e06f94865a4b18a");
        const JP_HASH: [u8; 20] = hex!("2848bb574bfcbf97f075adc4e0f4692ddd7fd0e8");
        const NTSC1_2_HASH: [u8; 20] = hex!("30cad7e8a88442b1388867f01bc6461097f4a152");
        const PAL1_0_HASH: [u8; 20] = hex!("8f6bf468447d9f10172cc4a472a56e1f526a5cb4");
        match hash {
            NTSC1_0_HASH => GameVersion::NTSC1_0,
            JP_HASH => GameVersion::JP,
            NTSC1_2_HASH => GameVersion::NTSC1_2,
            PAL1_0_HASH => GameVersion::PAL1_0,
            _ => GameVersion::Unknown,
        }
    }

    pub fn path_name(&self) -> Option<&str> {
        match *self {
            GameVersion::NTSC1_0 => Some("US"),
            GameVersion::JP => Some("JP"),
            _ => None,
        }
    }

    pub fn iso_name(&self) -> &str {
        match *self {
            GameVersion::NTSC1_0 | GameVersion::NTSC1_2 => "SOUE01.iso",
            GameVersion::JP => "SOUJ01.iso",
            GameVersion::PAL1_0 => "SOUP01.iso",
            GameVersion::Unknown => "",
        }
    }
}

impl fmt::Display for GameVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameVersion::NTSC1_0 => write!(f, "North American NTSC 1.00"),
            GameVersion::JP => write!(f, "Japanese"),
            GameVersion::NTSC1_2 => write!(f, "North American NTSC 1.02"),
            GameVersion::PAL1_0 => write!(f, "European PAL 1.00"),
            GameVersion::Unknown => write!(f, "Unknown version"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HashMismatchError {
    #[error("Error when reading hash: {0}")]
    BadHashRead(Error),
    #[error("{0} version is currently not supported.")]
    UnsupportedVersion(GameVersion),
    #[error("Given version is unknown and unsupported.")]
    UnknownVersion,
    #[error("{0} is a supported version, but {1} was expected.")]
    WrongSupportedVersion(GameVersion, GameVersion),
}

impl WiiIsoExtractor {
    pub fn new(path: PathBuf, version: GameVersion) -> Result<Self, Error> {
        let iso_file = fs::File::open(&path)?;
        let mut iso = WiiIsoReader::open(iso_file)?;
        let section_str = "DATA".to_owned();
        let part_type = WiiPartType::Data;

        let partition = iso
            .partitions()
            .iter()
            .find(|p| p.get_type() == part_type)
            .cloned()
            .unwrap();

        let partition_reader = iso.open_partition(partition)?;
        let section = Section {
            part: section_str,
            fst: partition_reader.get_fst().clone(),
            partition_reader,
        };
        Ok(WiiIsoExtractor {
            iso,
            partition: section,
            version,
        })
    }

    pub fn get_dol_hash(&mut self) -> Result<[u8; 20], Error> {
        let dol = self.partition.partition_reader.read_dol(&mut self.iso)?;
        let mut hasher = Sha1::new();
        hasher.update(&dol);
        Ok(hasher.finalize().try_into().unwrap())
    }

    // Verify that the given ISO has the right DOL hash
    pub fn validate(&mut self) -> Result<(), HashMismatchError> {
        let hash = self
            .get_dol_hash()
            .map_err(|e| HashMismatchError::BadHashRead(e))?;
        let found_version = GameVersion::from_hash(hash);
        if found_version == GameVersion::Unknown {
            return Err(HashMismatchError::UnknownVersion);
        }
        if !found_version.is_supported() {
            return Err(HashMismatchError::UnsupportedVersion(found_version));
        }
        if found_version != self.version {
            return Err(HashMismatchError::WrongSupportedVersion(
                found_version,
                self.version,
            ));
        }

        Ok(())
    }

    pub fn extract_to(&mut self, path: PathBuf) -> Result<(), Error> {
        let disc_header = self.iso.get_header().clone();
        let region = self.iso.get_region().clone();
        let section_path = path.join(format!("{}", self.partition.part));

        let section_path_disk = section_path.join("disc");
        create_dir_all(&section_path_disk)?;

        binrw_write_file(&section_path_disk.join("header.bin"), &disc_header)?;
        fs::write(section_path_disk.join("region.bin"), region)?;

        self.partition
            .partition_reader
            .extract_system_files(&section_path, &mut self.iso)?;
        let mut buffer = [0; 0x10_000];
        // count files
        let mut total_bytes = 0usize;
        self.partition
            .fst
            .callback_all_files::<std::io::Error, _>(&mut |_, node| {
                if let FstNode::File { length, name, .. } = node {
                    if !name.ends_with(".thp") {
                        total_bytes += *length as usize;
                    }
                }

                Ok(())
            })?;

        println!("Extracting files...");
        let bar = ProgressBar::new(total_bytes as u64);
        bar.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        let mut done_bytes = 0usize;
        let mut wii_encrypt_reader = self
            .partition
            .partition_reader
            .get_crypto_reader(&mut self.iso);
        self.partition
            .fst
            .callback_all_files::<std::io::Error, _>(&mut |names, node| {
                if let FstNode::File { offset, length, .. } = node {
                    let mut filepath = section_path.join("files");
                    for name in names {
                        filepath.push(name);
                    }
                    if !names.last().unwrap().ends_with(".thp") {
                        // println!("{filepath:?}");
                        // TODO: reduce create dir calls?
                        create_dir_all(filepath.parent().unwrap())?;

                        let mut outfile = fs::File::create(&filepath)?;
                        wii_encrypt_reader.seek(SeekFrom::Start(*offset))?;
                        let mut bytes_left = *length as usize;
                        loop {
                            let bytes_to_read = bytes_left.min(buffer.len());
                            let bytes_read =
                                wii_encrypt_reader.read(&mut buffer[..bytes_to_read])?;
                            if bytes_read == 0 {
                                break;
                            }

                            outfile.write_all(&buffer[..bytes_read])?;
                            done_bytes += bytes_read;
                            bytes_left -= bytes_read;
                            bar.inc(bytes_read as u64);
                            // println!("{}", bytes_left);
                        }
                    }
                }

                Ok(())
            })?;

        drop(wii_encrypt_reader);

        let certs = self
            .partition
            .partition_reader
            .read_certificates(&mut self.iso)?;
        binrw_write_file(&section_path.join("cert.bin"), &certs)?;
        let tmd = self.partition.partition_reader.read_tmd(&mut self.iso)?;
        binrw_write_file(&section_path.join("tmd.bin"), &tmd)?;
        binrw_write_file(
            &section_path.join("ticket.bin"),
            &self
                .partition
                .partition_reader
                .get_partition_header()
                .ticket,
        )?;

        bar.finish_with_message("Extraction done.");
        Ok(())
    }
}

pub fn rebuild_from_directory(src_dir: PathBuf, dest_path: PathBuf) -> Result<(), Error> {
    let mut dest_file = OpenOptions::new()
        .truncate(true)
        .read(true)
        .write(true)
        .create(true)
        .open(&dest_path)?;
    println!("Rebuilding ISO...");
    let bar = ProgressBar::new(100);
        bar.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{wide_bar:.cyan/blue}] {percent}% ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
    build_from_directory(&src_dir, &mut dest_file, &mut |done_percent| {
        bar.set_position(done_percent as u64);
    })?;
    bar.finish_with_message("Rebuilding done.");
    Ok(())
}
