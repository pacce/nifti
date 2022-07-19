mod decode;
mod encode;

#[cfg(test)]
mod test;

mod datatype;
mod dimension;
mod intent;
mod slice;

use {
    cookie_factory::{gen, GenError}
    , datatype::Datatype
    , dimension::Dimension
    , intent::Packet
    , slice::{Code, Slice}
    , std::io::{Read, Write}
};

const SIZE : usize = 348;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Header {
    size        : i32,
    dimension   : Dimension,
    intent      : Packet,
    datatype    : Datatype,
    bitpix      : i16,
    slice       : Slice,
    pixdim      : [f32; 8]
}

impl Header {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let mut xs = [0u8; SIZE];
        reader.read(&mut xs)?;

        match decode::header::<nom::error::Error<&decode::Bytes>>(&xs) {
            Ok((_, h))  => Ok(h),
            Err(_)      => unimplemented!()
        }
    }

    pub fn encode<W: Write>(&self, writer: &mut W) -> Result<(), GenError> {
        if let Err(e) = gen(encode::header(*self), writer) {
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn size(&self) -> &i32 {
        &self.size
    }

    pub fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    pub fn intent(&self) -> &Packet {
        &self.intent
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Header {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let dimension   = Dimension::arbitrary(g);
        let intent      = Packet::arbitrary(g);

        let datatype    = Datatype::arbitrary(g);
        let bitpix      = i16::arbitrary(g);

        let slice       = Slice::arbitrary(g);
        let pixdim      = [i32::arbitrary(g) as f32; 8];

        Self{
            size: SIZE as i32
            , dimension
            , intent
            , datatype
            , bitpix
            , slice
            , pixdim
        }
    }
}
