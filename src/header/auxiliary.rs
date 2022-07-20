use std::str::Utf8Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Auxiliary([u8; 24]);

impl Auxiliary {
    fn str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.0)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Auxiliary {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let xs = [u8::arbitrary(g); 24];
        Self(xs)
    }
}

impl From<&str> for Auxiliary {
    fn from(xs: &str) -> Self {
        let xs  = xs.as_bytes();
        let mut ys  = [0u8; 24];
        for (y, x) in ys.iter_mut().zip(xs) { *y = x.clone(); }

        Self(ys)
    }
}

impl From<&[u8; 24]> for Auxiliary {
    fn from(xs: &[u8; 24]) -> Self {
        Self(*xs)
    }
}

impl From<&[u8]> for Auxiliary {
    fn from(xs: &[u8]) -> Self {
        let mut ys  = [0u8; 24];
        for (y, x) in ys.iter_mut().zip(xs) { *y = x.clone(); }

        Self(ys)
    }
}

impl Into<[u8; 24]> for Auxiliary {
    fn into(self) -> [u8; 24] {
        self.0
    }
}

impl Into<[u8; 24]> for &Auxiliary {
    fn into(self) -> [u8; 24] {
        self.0
    }
}
