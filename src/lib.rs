pub mod config;
pub mod localization;
pub mod toc;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{io::BufReader, fs::File, path::PathBuf};

    use crate::toc::Toc;

    #[test]
    fn test_decompress() {
        let file = BufReader::new(File::open("test/toc").unwrap());
        let toc_buf = Toc::decompress(file).unwrap();
        std::fs::write("test/toc.dec", toc_buf).unwrap();
    }

    #[test]
    fn test_read() {
        let file = PathBuf::from("test/toc");
        Toc::read(file).unwrap();
    }

    #[test]
    fn test_encode_buf() {
        let file = PathBuf::from("test/toc");
        let toc = Toc::read(file).unwrap();
        let buf = unsafe { toc.as_buf() };
        let encoded = unsafe { Toc::compress(buf) }.unwrap();
        std::fs::write("test/toc.test", encoded).unwrap();
    }

    #[test]
    fn test_as_buf() {
        let file = PathBuf::from("test/toc");
        let toc = Toc::read(file).unwrap();
        let buf = unsafe { toc.as_buf() };
        std::fs::write("test/toc.buf", buf).unwrap();
    }
}