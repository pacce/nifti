use {
    super::{
        intent::Intent
        , Dimension
        , Header
        , Packet
    }
    , std::{
        fs::File
        , io::Read
        , path::Path
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
