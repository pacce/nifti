#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xform {
    UNKNOWN         = 0,
    SCANNER_ANAT    = 1,
    ALIGNED_ANAT    = 2,
    TALAIRACH       = 3,
    MNI_152         = 4,
    TEMPLATE_OTHER  = 5,
}

#[cfg(test)]
impl quickcheck::Arbitrary for Xform {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let code = i16::arbitrary(g);
        match code {
            0       => Xform::UNKNOWN,
            1       => Xform::SCANNER_ANAT,
            2       => Xform::ALIGNED_ANAT,
            3       => Xform::TALAIRACH,
            4       => Xform::MNI_152,
            _       => Xform::TEMPLATE_OTHER,
        }
    }
}
