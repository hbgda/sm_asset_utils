pub mod header;
pub mod section;

use std::{path::PathBuf, fs::File, error::Error, io::{BufReader, copy, Read, BufRead}, hash::{Hash, self}, collections::HashMap};
use flate2::bufread::ZlibDecoder;

use self::section::{SectionInfo, ArchiveFileEntry, ArchiveSizeEntry};

const TOC_SIGNATURE: [u8; 4] = [0xAF, 0x12, 0xAF, 0x77];

pub struct Toc {
    file: PathBuf,
    header: header::TocHeader,
    file_entries: Vec<ArchiveFileEntry>,
    asset_hashes: Vec<u64>,
    size_entries: Vec<ArchiveSizeEntry>
}

impl Toc {
    pub fn read(path: PathBuf) -> Result<Toc, Box<dyn Error>> {
        let file_buf = BufReader::new(File::open(path)?);        
        let toc_buf = Toc::decompress(file_buf).unwrap();
        Toc::parse(toc_buf).unwrap();
        todo!()
    }

    pub fn parse(buf: Vec<u8>) -> Result<Toc, Box<dyn Error>> {
        let mut off = 0;
        let header = header::parse(&buf[0..16])?;
        off += 16;

        let section = Toc::_parse_section_info(&buf[off..off + 12])?;
        let offset = section.offset as usize;
        let size = section.size as usize;
        let file_entries = Toc::_parse_archive_entries(&buf[offset..offset + size]);
        println!("File Entries: {}", file_entries.len());
        // for entry in file_entries {
        //     println!("Name: {}", entry.get_filename()?);
        // }
        off += 12;

        let section = Toc::_parse_section_info(&buf[off..off + 12])?;
        let offset = section.offset as usize;
        let size = section.size as usize;
        let asset_hashes = Toc::_parse_asset_hashes(&buf[offset..offset + size]);
        println!("Asset Hashes: {}", asset_hashes.len());
        off += 12;

        let section = Toc::_parse_section_info(&buf[off..off + 12])?;
        let offset = section.offset as usize;
        let size = section.size as usize;
        let size_entries = Toc::_parse_size_entries(&buf[offset..offset + size]);
        println!("Size Entries: {}", size_entries.len());
        off += 12;

        if size_entries.len() != asset_hashes.len() {
            return Err("Failed to properly parse toc file.".into());
        }

        let section = Toc::_parse_section_info(&buf[off..off + 12])?;
        let offset = section.offset as usize;
        let size = section.size as usize;
        let key_hashes = Toc::_parse_key_hashes(&buf[offset..offset + size]);
        println!("Key Hashes  : {}", key_hashes.len());
        off += 12;

        
        todo!()
    }

    pub fn decompress<T: BufRead>(mut buf: T) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut toc_sig = [0u8; 4];
        buf.read_exact(&mut toc_sig)?;
        if toc_sig != TOC_SIGNATURE {
            return Err(format!("Expected toc signature {TOC_SIGNATURE:#x?}, got {toc_sig:#x?}.").into())
        }

        let mut _size_buf = [0u8; 4];
        buf.read_exact(&mut _size_buf)?;
        
        let mut decoder = ZlibDecoder::new(buf);
        let mut out = vec![];
        
        copy(&mut decoder, &mut out)
            .expect("Failed to read from toc file.");
        
        Ok(out)
    }

    pub fn compress<T: BufRead>(buf: T) -> Vec<u8> {
        todo!()
    }
}

impl Toc {
    fn _parse_section_info(buf: &[u8]) -> Result<SectionInfo, Box<dyn Error>> {
        if buf.len() < 12 {
            return Err("Invalid section buffer.".into())
        }
        Ok(unsafe {
            *(&buf[0]
                as *const u8
                as *const SectionInfo
            )
        })
    }

    fn _parse_archive_entries(buf: &[u8]) -> Vec<ArchiveFileEntry> {
        let mut entries = Vec::new();
        for i in 0..buf.len() / 72 {
            let i = i * 72;
            entries.push(unsafe {
                *(&buf[i]
                    as *const u8
                    as *const ArchiveFileEntry
                )
            });
        }
        entries
    }

    fn _parse_asset_hashes(buf: &[u8]) -> Vec<u64> {
        let mut hashes = Vec::new();
        for i in 0..buf.len() / 8 {
            let i = i * 8;
            hashes.push( unsafe {
                *(&buf[i]
                    as *const u8
                    as *const u64
                )
            });
        }
        hashes
    }
    
    fn _parse_size_entries(buf: &[u8]) -> Vec<ArchiveSizeEntry> {
        let mut entries = Vec::new();
        for i in 0..buf.len() / 12 {
            let i = i * 12;
            entries.push( unsafe {
                *(&buf[i]
                    as *const u8
                    as *const ArchiveSizeEntry
                )
            });
        }
        entries
    }

    fn _parse_key_hashes(buf: &[u8]) -> Vec<u64> {
        let mut hashes = Vec::new();
        for i in 0..buf.len() / 8 {
            let i = i * 8;
            hashes.push(unsafe {
                *(&buf[i]
                    as *const u8
                    as *const u64
                )
            });
        }
        hashes
    }
}