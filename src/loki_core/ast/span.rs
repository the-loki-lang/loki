mod location;
pub use location::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    #[inline]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn to_str<'src>(&'src self, src: &'src str) -> &'src str {
        src.get(self.start..self.end).unwrap()
    }
}
