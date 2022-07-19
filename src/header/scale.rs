#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Scale {
    pub (super) slope       : f32,
    pub (super) intercept   : f32,
}

impl Scale {
    pub fn new(slope: f32, intercept: f32) -> Self {
        Self{slope, intercept}
    }

    pub fn slope(&self) -> &f32 {
        &self.slope
    }

    pub fn intercept(&self) -> &f32 {
        &self.intercept
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Scale {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let slope       = i32::arbitrary(g) as f32;
        let intercept   = i32::arbitrary(g) as f32;
        Self{slope, intercept}
    }
}
