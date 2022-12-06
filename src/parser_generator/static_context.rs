use std::{
    cell::RefCell,
    fs::{self, File},
    path::Path,
    rc::Rc,
};

use sourcemap::SourceMapBuilder;

use super::position::Position;

#[derive(Clone, Default)]
struct SourceMapRowEntry {
    source_column: u32,
    source_line: u32,
    target_column: u32,
}

#[derive(Default, Clone)]
struct SourceMap {
    sourcemap_line: Vec<Vec<SourceMapRowEntry>>,
    last: Option<SourceMapRowEntry>,
}

impl SourceMap {
    fn pad(&mut self, index: u32) {
        while self.sourcemap_line.len() <= index as usize {
            self.sourcemap_line.push(vec![])
        }
    }

    pub fn insert_sourcemap(&mut self, row: usize, sourcemap: Self) {
        let s = sourcemap.sourcemap_line[row..sourcemap.sourcemap_line.len()].to_vec();

        self.sourcemap_line.splice(row..row, s);
    }

    fn add_entry(
        &mut self,
        source_line: u32,
        source_column: u32,
        target_line: u32,
        target_column: u32,
    ) {
        self.pad(target_line);

        let line = self.sourcemap_line.get_mut(target_line as usize).unwrap();

        let e = SourceMapRowEntry {
            source_column,
            source_line,
            target_column,
        };

        line.push(e.clone());

        self.last = Some(e);
    }

    fn add_following_line(&mut self) {
        let mut last = self.last.clone().unwrap_or_default();

        last.target_column = 0;

        self.sourcemap_line.push(vec![last]);
    }

    fn write_soucemap(&self, src_file_path: &str, source_content: Option<&str>) {
        let src_file_name: String = Path::new(&src_file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut sourcemap = SourceMapBuilder::new(None);

        self.sourcemap_line
            .iter()
            .enumerate()
            .for_each(|(target_line, v)| {
                for e in v.iter() {
                    sourcemap.add(
                        target_line as u32,
                        e.target_column,
                        e.source_line,
                        e.source_column,
                        Some(&src_file_name),
                        None,
                    );
                }
            });

        let out = src_file_path.replace(".m1n", ".ts");
        let out_sourcemap = format!("{}.map", &out);

        let mut s = sourcemap.into_sourcemap();

        s.set_source_contents(0, source_content);

        let f = File::create(&out_sourcemap).expect("couldn't open map file");
        s.to_writer(f).expect("couldn't write map file");
    }
}

#[derive(Clone)]
pub struct StaticContext {
    sourcemap: SourceMap,
    rows: Vec<String>,
}

impl StaticContext {
    pub fn new() -> Self {
        Self {
            sourcemap: SourceMap::default(),
            rows: vec!["".to_string()],
        }
    }
    fn current_column(&self) -> u32 {
        self.rows[self.rows.len() - 1].len() as u32
    }
    pub fn current_line(&self) -> u32 {
        (self.rows.len() - 1) as u32
    }

    pub fn insert_context(&mut self, row: usize, context: Rc<RefCell<Self>>) {
        let c = context.borrow();
        let s = c.rows[row..c.rows.len()].to_vec();
        self.sourcemap.insert_sourcemap(row, c.sourcemap.clone());
        self.rows.splice(row..row, s);
    }

    pub fn add_posititon(&mut self, pos: Option<Position>) {
        if let Some(pos) = pos {
            self.sourcemap.add_entry(
                pos.line,
                pos.column,
                self.current_line(),
                self.current_column(),
            );
        }
    }

    pub fn add_string(&mut self, str: &str) {
        let mut var: Vec<String> = str.split("\n").into_iter().map(str::to_string).collect();

        let first = var.remove(0);

        let len = self.rows.len();

        self.rows[len - 1] += &first;

        for rest in var {
            self.sourcemap.add_following_line();
            self.rows.push(rest);
        }
    }

    pub fn write_files(&self, src_file_path: &str, source_content: Option<&str>) {
        self.sourcemap.write_soucemap(src_file_path, source_content);

        let out = src_file_path.replace(".m1n", ".ts");

        let content = self.rows.join("\n");

        fs::write(&out, &content).expect("couldn't read file");
    }
}
