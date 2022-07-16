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
        decode
        , encode
        , intent::Intent
        , Dimension
        , Header
        , Packet
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
        }
    };

    let home        = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut path    = &home.join("resources").join("avg152T1_LR_nifti.nii");

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
