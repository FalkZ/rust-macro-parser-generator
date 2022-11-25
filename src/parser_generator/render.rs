use std::{path::Path, fmt::Display, fs::{File, self}, rc::Rc, cell::RefCell};

use sourcemap::SourceMapBuilder;

use super::{position::{TryGetPosition, Position}, tokens::RawToken};

pub trait Render<Context: Clone>: TryGetPosition {
    fn render(&self, context: &mut RenderContext<Context>);
}


struct StaticContext {
    sourcemap: Vec<(u32, u32, u32, u32)>,
    rows: Vec<String>
 }

 impl StaticContext {
    fn new() -> Self {
        Self {
            sourcemap: vec![],
            rows: vec!["".to_string()]
        }
    }
    fn current_column(&self)-> u32 {
        self.rows[ self.rows.len()-1].len() as u32
     }
     fn current_line(&self)-> u32 { 
        (self.rows.len() -1) as u32
     }

     fn add_posititon(&mut self, pos: Option<Position>){
        if let Some(pos) = pos { 
            self.sourcemap.push( (self.current_line(),  self.current_column() ,pos.line ,pos.column));
        }
     }

     pub fn add_string(&mut self, str: &str){ 
        let mut var: Vec<String> = str.split("\n").into_iter().map(str::to_string).collect();
  
        let first = var.remove(0);
        
        let len = self.rows.len();
  
        self.rows[len-1] += &first;
  
        self.rows.append(&mut var);
        
     }

     pub fn write_files(&self, src_file_path: &str, source_content: Option<&str>){    
        
        let src_file_name: String = Path::new(&src_file_path).file_name().unwrap().to_str().unwrap().to_string();
 
        let mut sourcemap = SourceMapBuilder::new(None);

        self.sourcemap.iter().for_each(|t| {
            sourcemap.add( t.0, t.1, t.2, t.3, Some(&src_file_name), None);
        });
      
        let out = src_file_path.replace(".m1n", ".ts");
        let out_sourcemap = format!("{}.map", &out);
       
        
         let mut s =  sourcemap.into_sourcemap();

         let content = self.rows.join("\n");
         fs::write(&out, &content).expect("couldn't read file");
         
         s.set_source_contents(0, source_content);

         let f = File::create(&out_sourcemap).expect("couldn't open map file");
         s.to_writer(f).expect("couldn't write map file");
 
     }
  
 }

#[derive(Clone)]
pub struct RenderContext<C: Clone> {
    context: Box<C>,
    static_context: Rc<RefCell<StaticContext>>,
 }
 
 impl<C: Clone> RenderContext<C> {
    pub fn new(context: C) -> Self {
       Self {
          context: Box::new(context),
          static_context: Rc::new(RefCell::new(StaticContext::new()))
       }
    }

    pub fn get_context(&self) -> &C {
        &*self.context
    }

    pub fn set_context(&mut self,  context: C) {
        self.context = Box::new(context)
    }
 
 
    fn add_string(&self, string: &str){
      self.static_context.borrow_mut().add_string(string)
    }
    fn add_posititon(&self, pos: Option<Position>){
      self.static_context.borrow_mut().add_posititon(pos)
    }

 
    pub fn str<D: Display>(&mut self, content: D) -> &mut Self {
       self.add_string(&format!("{}", content));
 
       self
    }
 
    pub fn render_raw<R: RawToken>(&mut self, content: &R) -> &mut Self {
       let t = content.raw_token();
       self.add_posititon(Some(t.position));
       self.add_string(&t.raw);
 
       self
    }
 
    pub fn render<T: Render<C>>(&mut self, content: T) -> &mut Self  {
       self.add_posititon(content.try_position());
       content.render(&mut self.clone());
 
        self
    }
 
    pub fn join<T: Render<C>, S: Display>(&mut self, content: &Vec<T>, separator: S) -> &mut Self {
       use crate::parser_generator::position::TryGetPosition;
 
       self.add_posititon(content.try_position());
 
       content.iter().for_each(|v| {v.render(&mut self.clone());
       
       self.str(format!("{}", separator));
    
       });
      
 
       self
    }
 
    fn apply(){}
 
    pub fn write_files(&self, src_file_path: &str, source_content: Option<&str>){
      self.static_context.borrow().write_files(src_file_path, source_content);
    }


 }