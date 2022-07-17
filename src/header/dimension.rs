#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimension {
    pub (super) information : i8,
    pub (super) values      : [i16; 8]
}

impl Dimension {
    pub fn information(&self) -> &i8 {
        &self.information
    }

    pub fn values(&self) -> &[i16; 8] {
        &self.values
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Dimension {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let information = i8::arbitrary(g);
        let values      = i16::arbitrary(g);

        let values      = [values; 8];
        Self{information, values}
    }
}
