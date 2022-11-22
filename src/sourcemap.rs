use std::fmt::Display;
use std::fs::{File, self};

use std::path::Path;

use sourcemap::SourceMapBuilder;


pub struct RenderContext {
   src_file_path: String,
   src_file_name: String,
   sourcemap: SourceMapBuilder,
   rows: Vec<String>
}

impl RenderContext {
   pub fn new(src_file_path: &str) -> Self{

      let src_file_name: String = Path::new(&src_file_path).file_name().unwrap().to_str().unwrap().to_string();

      let mut sourcemap = SourceMapBuilder::new(None);

      Self {
         src_file_path: src_file_path.to_string(),
         src_file_name,
         sourcemap,
         rows: vec!["".to_string()]
      }
   }

   pub fn add_soucemap_entry(&mut self, pos: &Position){
      self.sourcemap.add( self.current_line(),  self.current_column() ,pos.line,pos.column, Some(&self.src_file_name), None);

   }

   pub fn add_token(&mut self, str: &str, pos: &Position){
      self.add_soucemap_entry(pos);
      self.add_string(str);

   }

   pub fn add_string(&mut self, str: &str){

      let mut var: Vec<String> = str.split("\n").into_iter().map(str::to_string).collect();

      let first = var.remove(0);
      
      let len = self.rows.len();

      self.rows[len-1] += &first;

      self.rows.append(&mut var);
      
   }

   fn current_column(&self)-> u32 {
      self.rows[ self.rows.len()-1].len() as u32
   }
   fn current_line(&self)-> u32 {
      (self.rows.len() -1) as u32
   }

   pub fn write_file(self, source_content: Option<&str>){

      let out = self.src_file_path.replace(".m1n", ".ts");

      let out_sourcemap = format!("{}.map", &out);

      let mut f = File::create(&out_sourcemap).expect("couldn't open map file");
      
      let content = self.rows.join("\n");
      
      let mut s = self.sourcemap.into_sourcemap();

      fs::write(&out, &content).expect("couldn't read file");

   
      s.set_source_contents(0, source_content);
      
      s.to_writer(f).expect("couldn't write map file");

   }
}


#[derive(Default, Clone, Debug)]
pub struct Position{
   pub column: u32,
   pub line: u32,
}

#[derive(Default, Clone, Debug)]
pub struct Token {
   pub raw: String,
   pub position: Position
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl Token{
   pub fn new(raw: String, position: Position)-> Self {
      Self { raw, position }
   }
   pub fn as_str<'a>(&'a self) -> &'a str {
      &self.raw
   }
}

pub trait Pos {
   fn position(&self)-> Position;
}

impl Pos for Token {
    fn position(&self)-> Position {
        self.position.clone()
    }
}


impl <T: Pos> Pos for Vec<T>{
   fn position(&self)-> Position {
         // TODO sensible default
         self.get(0).map(|v| v.position()).unwrap_or_default()
  }
}


