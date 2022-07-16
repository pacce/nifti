mod intent;
use intent::Packet;

use std::io::Read;

#[derive(Debug)]
pub struct Header {
    size        : i32,
    dimension   : Dimension,
    intent      : Intent,
}


#[derive(Debug)]
pub struct Dimension {
    information : i8,
    values      : [i16; 8]
}

#[derive(Debug)]
pub struct Intent {
    parameters  : [f32; 3]
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


mod decode {
    use {
        nom::{
            error::ParseError
            , multi::{count, fill}
            , number::complete::*
            , IResult
        }
        , super::*
    };

    pub (crate) type Bytes = [u8];

    fn sizeof_hdr<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, i32, E> {
        let (i, v) = be_i32(i)?;

        Ok((i, v))
    }

    fn data_type<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
        let (i, _) = count(be_i8, 10)(i)?;

        Ok((i, ()))
    }

    fn db_name<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
        let (i, _) = count(be_i8, 18)(i)?;

        Ok((i, ()))
    }

    fn extents<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
        let (i, _) = be_i32(i)?;

        Ok((i, ()))
    }

    fn session_error<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
        let (i, _) = be_i16(i)?;

        Ok((i, ()))
    }

    fn regular<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
        let (i, _) = be_i8(i)?;

        Ok((i, ()))
    }

    fn dim_info<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, i8, E> {
        let (i, v) = be_i8(i)?;

        Ok((i, v))
    }

    fn dim<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, [i16; 8], E> {
        let mut xs = [0i16; 8];
        let (i, _) = fill(be_i16, &mut xs)(i)?;

        Ok((i, xs))
    }

    fn dimension<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Dimension, E> {
        let (i, information)    = dim_info(i)?;
        let (i, values)         = dim(i)?;

        Ok((i, Dimension{information, values}))
    }

    fn intent<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Intent, E> {
        let mut xs = [0f32; 3];
        let (i, _) = fill(be_f32, &mut xs)(i)?;

        Ok((i, Intent{parameters: xs}))
    }


    pub fn header<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Header, E> {
        let (i, size)       = sizeof_hdr(i)?;
        let (i, _)          = data_type(i)?;
        let (i, _)          = db_name(i)?;
        let (i, _)          = extents(i)?;
        let (i, _)          = session_error(i)?;
        let (i, _)          = regular(i)?;

        let (i, dimension)  = dimension(i)?;
        let (i, intent)     = intent(i)?;

        Ok((i, Header{size, dimension, intent}))
    }
}
