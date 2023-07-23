pub mod header;
pub mod section;

use std::{path::PathBuf, fs::File, error::Error, io::{BufReader, copy, Read, BufRead, BufWriter}, hash::{Hash, self}, collections::HashMap, ops::Index};
use flate2::{bufread::ZlibDecoder, bufread::ZlibEncoder, Compression};

use crate::{toc::section::ArchiveSpanEntry, utils};

use self::section::{SectionInfo, ArchiveFileEntry, ArchiveSizeEntry, ArchiveChunkEntry, Section};

const TOC_SIGNATURE: [u8; 4] = [0xAF, 0x12, 0xAF, 0x77];

pub struct Toc {
    header: header::TocHeader,
    sections: HashMap<Section, SectionInfo>,
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

        let (file_section, file_entries) = Toc::_parse_section::<ArchiveFileEntry>(&buf, &mut off)?;
        println!("File Entries  : {}", file_entries.len());

        let (asset_section, asset_hashes) = Toc::_parse_section::<u64>(&buf, &mut off)?;
        println!("Asset Hashes  : {}", asset_hashes.len());

        let (size_section, size_entries) = Toc::_parse_section::<ArchiveSizeEntry>(&buf, &mut off)?;
        println!("Size Entries  : {}", size_entries.len());

        if size_entries.len() != asset_hashes.len() {
            return Err("Failed to properly parse toc file.".into());
        }

        let (key_section, key_hashes) = Toc::_parse_section::<u64>(&buf, &mut off)?;
        println!("Key Hashes    : {}", key_hashes.len());

        let (chunk_section, chunk_entries) = Toc::_parse_section::<ArchiveChunkEntry>(&buf, &mut off)?;
        println!("Chunk Entries : {}", chunk_entries.len());
        
        let (span_section, span_entries) = Toc::_parse_section::<ArchiveSpanEntry>(&buf, &mut off)?;
        println!("Span Entries  : {}", span_entries.len());

        let sections = HashMap::from([
            (Section::FileEntries, file_section),
            (Section::AssetHashes, asset_section),
            (Section::SizeEntries, size_section),
            (Section::KeyHashes, key_section),
            (Section::ChunkEntries, chunk_section),
            (Section::SpanEntries, span_section)
        ]);

        Ok(Toc {
            header,
            sections,
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
        
        copy(&mut decoder, &mut out)?;
        
        Ok(out)
    }

    pub unsafe fn compress(buf: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut compressed = Vec::new();

        compressed.extend(&[0xAF, 0x12, 0xAF, 0x77]);
        compressed.extend(utils::as_buf(&(buf.len() as u32)));
        let mut encoder = ZlibEncoder::new(&*buf, Compression::best());
        copy(&mut encoder, &mut compressed)?;

        Ok(compressed)
    }
}

impl Toc {
    pub fn save_to(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub unsafe fn as_buf(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Mutate later to update size
        buf.extend(utils::as_buf(&self.header));
        // Empty slot for section data
        buf.extend(&vec![0; 72]);

        let mut string_buf = b"ArchiveTOC".to_vec();
        string_buf.resize(24, 0);
        buf.extend(string_buf);


        let mut sections = self.sections.iter().collect::<Vec<_>>();
        sections.sort_by(|a, b| a.1.offset.cmp(&b.1.offset));

        let mut section_idx = 0;
        for (section, mut section_info) in sections.iter_mut().map(|(s, i)| (s.clone(), i.clone())) {
            section_info.offset = buf.len() as u32;
            match section {
                Section::FileEntries => {
                    section_idx = 0;
                    section_info.size = (std::mem::size_of::<ArchiveFileEntry>() * self.file_entries.len()) as u32;
                    self.file_entries.iter().for_each(|e| buf.extend(utils::as_buf(e)));
                },
                Section::AssetHashes => {
                    section_idx = 1;
                    section_info.size = (std::mem::size_of::<u64>() * self.asset_hashes.len()) as u32;
                    self.asset_hashes.iter().for_each(|e| buf.extend(utils::as_buf(e)));
                },
                Section::SizeEntries => {
                    section_idx = 2;
                    section_info.size = (std::mem::size_of::<ArchiveSizeEntry>() * self.size_entries.len()) as u32;
                    self.size_entries.iter().for_each(|e| buf.extend(utils::as_buf(e)));
                },
                Section::KeyHashes => {
                    section_idx = 3;
                    section_info.size = (std::mem::size_of::<u64>() * self.key_hashes.len()) as u32;
                    self.key_hashes.iter().for_each(|e| buf.extend(utils::as_buf(e)));
                },
                Section::ChunkEntries => {
                    section_idx = 4;
                    section_info.size = (std::mem::size_of::<ArchiveChunkEntry>() * self.chunk_entries.len()) as u32;
                    self.chunk_entries.iter().for_each(|e| buf.extend(utils::as_buf(e)));
                },
                Section::SpanEntries => {
                    section_idx = 5;
                    section_info.size = (std::mem::size_of::<ArchiveSpanEntry>() * self.span_entries.len()) as u32;
                    self.span_entries.iter().for_each(|e| buf.extend(utils::as_buf(e)));
                },
            }
            let idx = 16 + (section_idx * 12);
            buf[idx..idx + 12].copy_from_slice(utils::as_buf(&section_info));
        }
        let len = buf.len().clone() as u32;
        let len_buf = utils::as_buf(&len);
        buf[8..12].copy_from_slice(len_buf);
        buf
    }

    pub fn add_entry(&mut self, file_name: String, offset: u32, size: u32, hash: u64, span_idx: usize) -> Result<(), Box<dyn Error>> {
        let span = match self.span_entries.get(span_idx as usize) {
            Some(entry) => entry,
            None => return  Err("Invalid group.".into())
        }; 

        let mut asset_offset = span.offset as usize;
        while asset_offset < (self.span_entries[span_idx].offset + self.span_entries[span_idx].size) as usize && hash > self.asset_hashes[asset_offset] {
            asset_offset += 1;
        }

        if hash == self.asset_hashes[asset_offset] {
            let idx = self.index_of_entry(file_name).unwrap();
            println!("Hash already exists, updating asset.");
            self.size_entries[asset_offset] = ArchiveSizeEntry {
                chunks: 1,
                size,
                chunk_idx: asset_offset as u32
            };
            self.chunk_entries[asset_offset] = ArchiveChunkEntry {
                asset_file: idx as u32,
                offset
            };
            return Ok(());
        }
        
        let span = self.span_entries.get_mut(span_idx).unwrap();
        span.size += 1;

        for entry in self.span_entries[span_idx + 1..].iter_mut() {
            entry.offset += 1;
        }

        self.asset_hashes.insert(asset_offset, hash);
        let chunk_map = 10000 + self.file_entries.len() as u32;
        self.file_entries.push(
            ArchiveFileEntry::new(file_name, 0, chunk_map)?
        );
        self.chunk_entries.insert(asset_offset, ArchiveChunkEntry {
            asset_file: self.file_entries.len() as u32 - 1,
            offset
        });
        self.size_entries.insert(asset_offset, ArchiveSizeEntry {
            chunks: 1,
            size,
            chunk_idx: asset_offset as u32           
        });

        for entry in self.size_entries[asset_offset + 1..].iter_mut() {
            entry.chunk_idx += 1;
        } 

        Ok(())
    }

    pub fn find_entry(&self, file_name: String) -> Option<&ArchiveFileEntry> {
        self.file_entries.iter().find(|e| e.get_file_name().unwrap() == file_name)
    }

    pub fn index_of_entry(&self, file_name: String) -> Option<usize> {
        self.file_entries.iter().position(|e| e.get_file_name().unwrap() == file_name)
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
}