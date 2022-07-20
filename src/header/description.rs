use std::str::Utf8Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Description([u8; 80]);

impl Description {
    fn str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.0)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Description {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let xs = [u8::arbitrary(g); 80];
        Self(xs)
    }
}

impl From<&str> for Description {
    fn from(xs: &str) -> Self {
        let xs  = xs.as_bytes();
        let mut ys  = [0u8; 80];
        for (y, x) in ys.iter_mut().zip(xs) { *y = x.clone(); }

        Self(ys)
    }
}

impl From<&[u8; 80]> for Description {
    fn from(xs: &[u8; 80]) -> Self {
        Self(*xs)
    }
}

impl From<&[u8]> for Description {
    fn from(xs: &[u8]) -> Self {
        let mut ys  = [0u8; 80];
        for (y, x) in ys.iter_mut().zip(xs) { *y = x.clone(); }

        Self(ys)
    }
}

impl Into<[u8; 80]> for Description {
    fn into(self) -> [u8; 80] {
        self.0
    }
}

impl Into<[u8; 80]> for &Description {
    fn into(self) -> [u8; 80] {
        self.0
    }
}
