use {
    cookie_factory::gen
    , nom::error::ErrorKind
    , std::{
        io::Cursor
        , mem::size_of
    }
    , super::{decode, encode}
};

#[quickcheck]
fn sizeof_hdr(expected: i32) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::sizeof_hdr(expected), &mut buffer) {
        false
    } else {
        match decode::sizeof_hdr::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn dim_info(expected: i8) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::dim_info(expected), &mut buffer) {
        false
    } else {
        match decode::dim_info::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn dim(expected: i16) -> bool {
    let expected = [expected; 8];

    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::dim(expected), &mut buffer) {
        false
    } else {
        match decode::dim::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}
