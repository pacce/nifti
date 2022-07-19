#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Limits(f32, f32);

impl Limits {
    pub fn new(minimum: f32, maximum: f32) -> Self {
        Self(minimum, maximum)
    }

    pub fn minimum(&self) -> &f32 {
        &self.0
    }

    pub fn maximum(&self) -> &f32 {
        &self.1
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Limits {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let minimum = i32::arbitrary(g) as f32;
        let maximum = i32::arbitrary(g) as f32;
        Self::new(minimum, maximum)
    }
}
