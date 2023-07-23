use std::error::Error;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TocHeader {
    pub magic: u32,
    pub hash: u32,
    pub size: u32,
    pub sections: u32
}

pub const HEADER_SIGNATURE: [u8; 4] = [0x31, 0x54, 0x41, 0x44];

pub fn parse(buf: &[u8]) -> Result<TocHeader, Box<dyn Error>> {
    if buf.len() != 16 {
        return Err("Invalid buffer for Toc Header.".into());
    }
    if buf[0..4] != HEADER_SIGNATURE {
        println!("{:#x?}", &buf[0..4]);
        return Err("Invalid signature for Toc Header.".into())
    }

    /*
        Technically unsafe but quite frankly
        it is an issue if the developer using this lib 
        somehow passes an invalid header buffer.
    */ 
    Ok(unsafe {
        *(&buf[0]
            as *const u8
            as *const TocHeader
        ) 
    })
}