#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatype {
    NONE        =    0,
    BINARY      =    1,
    UINT8       =    2,
    INT16       =    4,
    INT32       =    8,
    FLOAT32     =   16,
    COMPLEX64   =   32,
    FLOAT64     =   64,
    RGB24       =  128,
    ALL         =  255,
    INT8        =  256,
    UINT16      =  512,
    UINT32      =  768,
    INT64       = 1024,
    UINT64      = 1280,
    FLOAT128    = 1536,
    COMPLEX128  = 1792,
    COMPLEX256  = 2048,
    RGBA32      = 2304,
}

#[cfg(test)]
impl quickcheck::Arbitrary for Datatype {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let code = i16::arbitrary(g);
        match code {
            1       => Datatype::BINARY,
            2       => Datatype::UINT8,
            4       => Datatype::INT16,
            8       => Datatype::INT32,
            16      => Datatype::FLOAT32,
            32      => Datatype::COMPLEX64,
            64      => Datatype::FLOAT64,
            128     => Datatype::RGB24,
            255     => Datatype::ALL,
            256     => Datatype::INT8,
            512     => Datatype::UINT16,
            768     => Datatype::UINT32,
            1024    => Datatype::INT64,
            1280    => Datatype::UINT64,
            1536    => Datatype::FLOAT128,
            1792    => Datatype::COMPLEX128,
            2048    => Datatype::COMPLEX256,
            2304    => Datatype::RGBA32,
            _       => Datatype::NONE,
        }
    }
}
