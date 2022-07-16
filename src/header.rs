mod decode;
mod encode;

mod intent;
use intent::Packet;

use std::io::Read;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct Header {
    size        : i32,
    dimension   : Dimension,
    intent      : Packet,
}


#[derive(Debug, PartialEq)]
pub struct Dimension {
    information : i8,
    values      : [i16; 8]
}

impl Header {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let mut xs = [0u8; 348];
        reader.read(&mut xs)?;

        match decode::header::<nom::error::Error<&decode::Bytes>>(&xs) {
            Ok((_, h))  => Ok(h),
            Err(_)      => unimplemented!()
        }
    }
}
