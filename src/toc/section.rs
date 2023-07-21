use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct SectionInfo {
    pub hash: u32,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArchiveFileEntry {
    data: [u8; 8],
    file_name_bytes: [u8; 64]
}

#[derive(Debug, Clone, Copy)]
pub struct ArchiveSizeEntry {
    pub chunks: u32,
    pub size: u32,
    pub chunk_idx: u32
}

#[derive(Debug, Clone, Copy)]
pub struct ArchiveChunkEntry {
    pub asset_file: u32,
    pub offset: u32
}

#[derive(Debug, Clone, Copy)]
pub struct ArchiveSpanEntry {
    pub offset: u32,
    pub size: u32
}

impl ArchiveFileEntry {
    pub fn get_filename(&self) -> Result<String, Box<dyn Error>> {
        Ok(
            String::from_utf8(self.file_name_bytes.to_vec())?
        )
    }
}