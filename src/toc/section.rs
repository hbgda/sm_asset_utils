use std::error::Error;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Section {
    FileEntries,
    AssetHashes,
    SizeEntries,
    KeyHashes,
    ChunkEntries,
    SpanEntries
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SectionInfo {
    pub hash: u32,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArchiveFileEntry {
    pub _padding: [u8; 3],
    pub install_bucket: u8,
    pub chunk_map: u32,
    file_name_bytes: [u8; 64]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArchiveSizeEntry {
    pub chunks: u32,
    pub size: u32,
    pub chunk_idx: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArchiveChunkEntry {
    pub asset_file: u32,
    pub offset: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArchiveSpanEntry {
    pub offset: u32,
    pub size: u32
}

impl ArchiveFileEntry {
    pub fn new(file_name: String, install_bucket: u8, chunk_map: u32) -> Result<ArchiveFileEntry, Box<dyn Error>> {
        if file_name.len() > 64 {
            return Err("Invalid file name.".into())
        }
        
        let mut name_bytes_vec = file_name.as_bytes().to_vec();
        name_bytes_vec.resize(64, 0);
        let file_name_bytes: [u8; 64] = name_bytes_vec.try_into().unwrap();
        
        Ok(ArchiveFileEntry {
            _padding: [0, 0, 0],
            install_bucket,
            chunk_map,
            file_name_bytes
        })
    }

    pub fn get_file_name(&self) -> Result<String, Box<dyn Error>> {
        Ok(
            String::from_utf8(self.file_name_bytes.to_vec())?
        )
    }
}