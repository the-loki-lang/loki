#[derive(Clone, Copy, Debug)]
pub struct Location {
    start: usize,
    end: usize,
    line: usize,
}

impl Location {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Self { start, end, line }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn advance(&mut self, current: char) {
        if current == '\n' {
            self.line += 1;
            self.start = 1;
            self.end = 1;
        } else {
            self.start += 1;
            self.end += 1;
        }
    }
}
