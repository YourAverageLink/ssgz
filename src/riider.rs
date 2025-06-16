// TODO - rework into functions rather than CLI
// Taken from the main.rs file in the disc-riider repo

use clap::Parser;
use disc_riider::{builder, structs::WiiPartType, WiiIsoReader};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};
use thiserror::Error;

#[derive(Debug, Parser)]
#[clap(about = "Utility to extract wii isos")]
enum Commands {
    #[clap(about = "show sections of the iso")]
    Sections { filename: PathBuf },
    #[clap(about = "extract the system files of an iso partition to a destination folder")]
    ExtractSys {
        filename: PathBuf,
        destination: PathBuf,
        #[clap(long, default_value = "DATA")]
        section: String,
    },
    #[clap(about = "print all file names present in the given section")]
    PrintFiles {
        filename: PathBuf,
        #[clap(long, default_value = "DATA")]
        section: String,
    },
    #[clap(about = "repack an ISO")]
    Rebuild {
        src_dir: PathBuf,
        dest_file: PathBuf,
    },
}

#[derive(Error, Debug)]
enum MyError {
    #[error("IO Error: {io_error}")]
    IOError {
        #[from]
        io_error: std::io::Error,
    },
    #[error("Read error: {error}")]
    BinrwError {
        #[from]
        error: binrw::error::Error,
    },
    #[error("{0} is not a valid section, options are: DATA, CHANNEL, UPDATE")]
    InvalidSection(String),
    #[error("section {0:?} not present!")]
    SectionNotFound(WiiPartType),
    #[error("{0}")]
    StringError(String),
}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        MyError::StringError(s)
    }
}

fn extract_to_path() {}

fn main() -> Result<(), MyError> {
    let args = Commands::parse();
    match args {
        Commands::Sections { filename } => {
            let f = File::open(filename)?;
            let reader = WiiIsoReader::open(f)?;
            for partition in reader.partitions() {
                println!("{:?}: {:X}", partition.get_type(), partition.get_offset());
            }
        }
        Commands::PrintFiles { section, filename } => {
            let f = File::open(filename)?;
            let mut reader = WiiIsoReader::open(f)?;
            let part_type = match section.to_ascii_uppercase().as_str() {
                "DATA" => WiiPartType::Data,
                "CHANNEL" => WiiPartType::Channel,
                "UPDATE" => WiiPartType::Update,
                _ => {
                    return Err(MyError::InvalidSection(section));
                }
            };
            let partition = reader
                .partitions()
                .iter()
                .find(|p| p.get_type() == part_type)
                .cloned()
                .ok_or_else(|| MyError::SectionNotFound(part_type))?;
            let part_reader = reader.open_partition(partition)?;
            part_reader.get_fst().print_tree();
        }
        Commands::ExtractSys {
            section,
            destination,
            filename,
        } => {
            let f = File::open(filename)?;
            let mut reader = WiiIsoReader::open(f)?;
            let part_type = match section.to_ascii_uppercase().as_str() {
                "DATA" => WiiPartType::Data,
                "CHANNEL" => WiiPartType::Channel,
                "UPDATE" => WiiPartType::Update,
                _ => return Err(MyError::InvalidSection(section)),
            };
            let partition = reader
                .partitions()
                .iter()
                .find(|p| p.get_type() == part_type)
                .cloned()
                .ok_or_else(|| MyError::SectionNotFound(part_type))?;

            let mut part_reader = reader.open_partition(partition)?;
            part_reader.extract_system_files(&destination, &mut reader)?;
        }
        Commands::Rebuild { src_dir, dest_file } => {
            let mut f = OpenOptions::new()
                .truncate(true)
                .read(true)
                .write(true)
                .create(true)
                .open(&dest_file)?;
            builder::build_from_directory(&src_dir, &mut f, &mut |percent| -> () {
                println!("rebuilding... {}%", percent);
            })
            .map_err(|e| format!("{e:?}"))?;
        }
    }
    Ok(())
}