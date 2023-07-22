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
}