use {
    cookie_factory::gen
    , nom::error::ErrorKind
    , std::{
        fs::File
        , io::Cursor
        , mem::size_of
        , path::Path
    }
    , super::{
        Code
        , Datatype
        , Dimension
        , decode
        , encode
        , intent::Intent
        , Header
        , Packet
        , Scale
        , Slice
    }
};

#[test]
fn nifti1() {
    let expected = Header{
        size            : 348,
        dimension       : Dimension{
            information : 0,
            values      : [3, 91, 109, 91, 1, 1, 1, 1]
        },
        intent          : Packet {
            parameters  : [0.0, 0.0, 0.0],
            intent      : Intent::NONE
        },
        datatype        : Datatype::UINT8,
        bitpix          : 8,
        slice           : Slice {
            start       : 0,
            end         : 0,
            code        : Code::UNKNOWN,
            duration    : 0.0
        },
        pixdim          : [0.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0],
        offset          : 352.0f32,
        scale           : Scale {
            slope       : 0.0f32,
            intercept   : 0.0f32,
        }
    };

    let home    = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path    = &home.join("resources").join("avg152T1_LR_nifti.nii");

    let mut file    = File::open(&path).unwrap();

    match Header::decode(&mut file) {
        Err(_) => assert!(false),
        Ok(actual) => assert_eq!(actual, expected),
    }
}

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

#[quickcheck]
fn dimension(expected: Dimension) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::dimension(expected), &mut buffer) {
        false
    } else {
        match decode::dimension::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn packet(expected: Packet) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::packet(expected), &mut buffer) {
        false
    } else {
        match decode::packet::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn datatype(expected: Datatype) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::datatype(expected), &mut buffer) {
        false
    } else {
        match decode::datatype::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn bitpix(expected: i16) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::bitpix(expected), &mut buffer) {
        false
    } else {
        match decode::bitpix::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn slice_start(expected: i16) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::slice_start(expected), &mut buffer) {
        false
    } else {
        match decode::slice_start::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn slice_end(expected: i16) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::slice_end(expected), &mut buffer) {
        false
    } else {
        match decode::slice_end::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn slice_code(expected: Code) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i16>()]);
    if let Err(_) = gen(encode::slice_code(expected), &mut buffer) {
        false
    } else {
        match decode::slice_code::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}

#[quickcheck]
fn header(expected: Header) -> bool {
    let mut buffer = Cursor::new(vec![0u8; size_of::<i32>()]);
    if let Err(_) = gen(encode::header(expected), &mut buffer) {
        false
    } else {
        match decode::header::<(&[u8], ErrorKind)>(buffer.get_ref()) {
            Ok((_, actual)) => actual == expected,
            Err(_)          => false,
        }
    }
}
