use std::fs::{File, self};

use std::path::Path;

use sourcemap::SourceMapBuilder;


struct RenderContext {
   src_file_path: String,
   src_file_name: String,
   sourcemap: SourceMapBuilder,
   rows: Vec<String>
}

impl RenderContext {
   fn new(src_file_path: &str) -> Self{

      let src_file_name: String = Path::new(&src_file_path).file_name().unwrap().to_str().unwrap().to_string();

      let mut sourcemap = SourceMapBuilder::new(None);

      Self {
         src_file_path: src_file_path.to_string(),
         src_file_name,
         sourcemap,
         rows: vec!["".to_string()]
      }
   }

   fn add_soucemap_entry(&mut self, pos: &Position){
      self.sourcemap.add( self.current_line(),  self.current_column() ,pos.line,pos.column, Some(&self.src_file_name), None);

   }

   fn add_token(&mut self, str: &str, pos: &Position){
      self.add_soucemap_entry(pos);
      self.add_string(str);

   }

   fn add_string(&mut self, str: &str){

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

   fn write_file(self, source_content: Option<&str>){

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


struct Position{
   column: u32,
   line: u32,
}
struct Token {
   token: String,
   position: Position
}


trait SourceEntry {
   fn render(&self, context: &mut RenderContext);
}

impl SourceEntry for &str {
    fn render(&self, context: &mut RenderContext) {
        context.add_string(self);
    }
}

impl SourceEntry for Vec<Box<dyn SourceEntry>> {
   fn render(&self, context: &mut RenderContext) {
      self.iter().for_each(|s|{s.render(context)});
   }
}

pub fn test_source_map(file_name: &str, c: &str){
   let mut r = RenderContext::new(file_name);

   let p =  Position {column: 2, line: 3};

   r.add_string("No mappings\nline");
   r.add_token(" test token", &p);

   r.add_string("No mappings\nline\n");

   let p =  Position {column: 4, line: 5};

   r.add_token(" test token 2", &p);

   r.write_file(Some(&c));
}

struct Source {
   template: Vec<Box<dyn SourceEntry>>
}

pub fn create_source_map(file_name: &str, c: &str){
   let mut s = SourceMapBuilder::new(Some(&file_name.replace(".m1n", ".ts")));

   let name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();

   s.add(10, 2,12,4, Some(&name), None);
   s.add(12, 6,8,20, Some(&name), None);


  
  

   let p = format!("{}.2.map", file_name);

   let mut f = File::create(&p).expect("msg");
   
   
   let mut s = s.into_sourcemap();

   s.set_source_contents(0, Some(c));
   
   s.to_writer(f).expect("fdkl");
   
}
