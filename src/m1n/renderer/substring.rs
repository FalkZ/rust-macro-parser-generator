pub trait Substring {
    fn substring(&self, start: isize, end: isize) -> String;
}

impl Substring for &str {
    fn substring(&self, start: isize, end: isize) -> String {
        let start = if start < 0 {
            self.len() as isize + start
        } else {
            start
        } as usize;

        let end = if end < 0 {
            self.len() as isize + end
        } else {
            end
        } as usize;

        format!("{}", &self[start..end])
    }
}
