pub mod header;
pub mod section;

use std::{path::PathBuf, fs::File, error::Error, io::{BufReader, copy, Read, BufRead}, hash::{Hash, self}, collections::HashMap};
use flate2::bufread::ZlibDecoder;

use crate::toc::section::ArchiveSpanEntry;

use self::section::{SectionInfo, ArchiveFileEntry, ArchiveSizeEntry, ArchiveChunkEntry};

const TOC_SIGNATURE: [u8; 4] = [0xAF, 0x12, 0xAF, 0x77];

pub struct Toc {
    header: header::TocHeader,
    file_entries: Vec<ArchiveFileEntry>,
    asset_hashes: Vec<u64>,
    size_entries: Vec<ArchiveSizeEntry>,
    key_hashes: Vec<u64>,
    chunk_entries: Vec<ArchiveChunkEntry>,
    span_entries: Vec<ArchiveSpanEntry>
}

impl Toc {
    pub fn read(path: PathBuf) -> Result<Toc, Box<dyn Error>> {
        let file_buf = BufReader::new(File::open(path)?);        
        let toc_buf = Toc::decompress(file_buf).unwrap();
        Toc::parse(toc_buf)
    }

    pub fn parse(buf: Vec<u8>) -> Result<Toc, Box<dyn Error>> {
        let mut off = 0;
        let header = header::parse(&buf[0..16])?;
        off += 16;

        let (_, file_entries) = Toc::_parse_section::<ArchiveFileEntry>(&buf, &mut off)?;
        println!("File Entries  : {}", file_entries.len());

        let (_, asset_hashes) = Toc::_parse_section::<u64>(&buf, &mut off)?;
        println!("Asset Hashes  : {}", asset_hashes.len());

        let (_, size_entries) = Toc::_parse_section::<ArchiveSizeEntry>(&buf, &mut off)?;
        println!("Size Entries  : {}", size_entries.len());

        if size_entries.len() != asset_hashes.len() {
            return Err("Failed to properly parse toc file.".into());
        }

        let (_, key_hashes) = Toc::_parse_section::<u64>(&buf, &mut off)?;
        println!("Key Hashes    : {}", key_hashes.len());

        let (_, chunk_entries) = Toc::_parse_section::<ArchiveChunkEntry>(&buf, &mut off)?;
        println!("Chunk Entries : {}", chunk_entries.len());
        
        let (_, span_entries) = Toc::_parse_section::<ArchiveSpanEntry>(&buf, &mut off)?;
        println!("Span Entries  : {}", span_entries.len());

        Ok(Toc {
            header,
            file_entries,
            asset_hashes,
            size_entries,
            key_hashes,
            chunk_entries,
            span_entries
        })
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
    fn _parse_section<T: Sized + Copy>(buf: &[u8], offset: &mut usize) -> Result<(SectionInfo, Vec<T>), Box<dyn Error>> {
        let section = Toc::_parse_section_info(&buf[*offset..*offset + 12])?;
        *offset += 12;
        let offset = section.offset as usize;
        let size = section.size as usize;
        let data = Toc::_parse_buf(&buf[offset..offset + size]);
        Ok((section, data))
    }

    fn _parse_buf<T: Sized + Copy>(buf: &[u8]) -> Vec<T> {
        let dif = std::mem::size_of::<T>();
        let mut items = Vec::<T>::new();
        for i in 0..buf.len() / dif {
            let i = i * dif;
            items.push(unsafe {
                *(&buf[i]
                    as *const u8
                    as *const T
                )
            });
        }
        items
    }
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
        Toc::_parse_buf(buf)
    }

    fn _parse_hashes(buf: &[u8]) -> Vec<u64> {
        Toc::_parse_buf(buf)
    }
    
    fn _parse_size_entries(buf: &[u8]) -> Vec<ArchiveSizeEntry> {
        Toc::_parse_buf(buf)
    }

    fn _parse_chunk_entries(buf: &[u8]) -> Vec<ArchiveChunkEntry> {
        Toc::_parse_buf(buf)
    }
}