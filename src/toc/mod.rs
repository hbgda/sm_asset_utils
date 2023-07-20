pub mod header;
pub mod section;

use std::{path::PathBuf, fs::File, error::Error, io::{BufReader, copy, Read, BufRead}};
use flate2::bufread::ZlibDecoder;

const TOC_SIGNATURE: [u8; 4] = [0xAF, 0x12, 0xAF, 0x77];

pub struct Toc {
    file: PathBuf,

}

impl Toc {
    pub fn read(path: PathBuf) -> Result<Toc, Box<dyn Error>> {
        let file_buf = BufReader::new(File::open(path)?);        
        let toc_buf = Toc::decompress(file_buf)?;
        Toc::parse(toc_buf).unwrap();
        Err("".into())
    }

    pub fn parse(buf: Vec<u8>) -> Result<Toc, Box<dyn Error>> {
        let header = header::parse(&buf[0..16])?;
        println!("Header: {header:?}");

        let data = &buf[16..];
        
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

        // let mut zlib_sig = [0u8; 2];
        // buf.read_exact(&mut zlib_sig)?;
        // if zlib_sig != ZLIB_SIGNATURE {
        //     return Err(format!("Expected zlib signature {ZLIB_SIGNATURE:#x?}, got {zlib_sig:#x?}.").into())
        // }

        // let mut compressed_buf = Vec::new();
        // buf.read_to_end(&mut compressed_buf)?;
        
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