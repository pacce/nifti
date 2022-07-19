#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slice {
    pub (super) start       : i16,
    pub (super) end         : i16,
    pub (super) code        : Code,
    pub (super) duration    : f32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    UNKNOWN     = 0,
    SEQ_INC     = 1,
    SEQ_DEC     = 2,
    ALT_INC     = 3,
    ALT_DEC     = 4,
    ALT_INC2    = 5,
    ALT_DEC2    = 6,
}

#[cfg(test)]
impl quickcheck::Arbitrary for Slice {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let start       = i16::arbitrary(g);
        let end         = i16::arbitrary(g);
        let code        = Code::arbitrary(g);
        let duration    = i16::arbitrary(g) as f32;

        Self{start, end, code, duration}
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Code {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let code = u8::arbitrary(g);
        match code {
            1 => Code::SEQ_INC,
            2 => Code::SEQ_DEC,
            3 => Code::ALT_INC,
            4 => Code::ALT_DEC,
            5 => Code::ALT_INC2,
            6 => Code::ALT_DEC2,
            _ => Code::UNKNOWN,
        }
    }
}
