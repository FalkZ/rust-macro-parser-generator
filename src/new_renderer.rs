use crate::{source::Source, grammar::{statements, statement, function, variable, maybe_arguments, arguments, argument_single}, sourcemap::{RenderContext, Pos, Position, Token}, render};


#[macro_export]
macro_rules! renderer {
    ($self:ident, $context:ident, $template:expr $(,$ins:expr$(=>$arg:expr)?)*) => {
        {
            let own_pos = $self.position();
    
            let mut strings =  $template.split("{}");


            $context.add_token(strings.next().unwrap(), &own_pos);

            $(
                $ins.ren($context, $($arg)?);
                $context.add_token(strings.next().unwrap(), &own_pos);
            )*
        }     
    };
}

pub trait Render: Pos {
    fn ren(&self, s: &mut RenderContext);
}



pub trait ContextRender<T>: Pos {
    fn ren(&self, s: &mut RenderContext, context: T);
}



impl Pos for maybe_arguments{
    fn position(&self)-> Position {
        Position::default()
    }
}



impl Render for argument_single {
    fn ren(&self, s: &mut RenderContext) {
        renderer!(self, s, "{}", self.arg.0);
    }
}

impl Render for Token{
    fn ren(&self, s: &mut RenderContext) {
        renderer!(self, s, self.raw);
    }
}

impl Render for arguments {
    fn ren(&self, s: &mut RenderContext) {
        self.arguments.iter().for_each(|v| renderer!(self, s, "{},", v));

        renderer!(self, s, "{}", self.last.0);
    }
}

impl Render for maybe_arguments {
    fn ren(&self, s: &mut RenderContext) {
        match self {
            maybe_arguments::no_arguments(_) => renderer!(self, s, "()"),
            maybe_arguments::arguments(v) => renderer!(self, s, "({})", v)
        }
    }
}

impl Render for function  {
    fn ren(&self, s: &mut RenderContext) {      
        renderer!(self, s, "fn{} \n",  self.arguments)
    }
}

impl Render for variable  {
    fn ren(&self, s: &mut RenderContext) {
        renderer!(self, s, "var \n")
    }
}


impl Render for statements  {
    fn ren(&self, s: &mut RenderContext) {
        self
        .iter()
        .for_each(|v| match *v.statement.to_owned() {
            statement::function(v) => {
                renderer!(self, s, "statement: {}", v);
            }

            statement::variable(v) => {
                renderer!(self, s, "statement: {}", v);
            }

        });
        
    }
}



pub fn renderer(source_path: &str, source_content: &str,s: &statements){

    let mut src = RenderContext::new(source_path);

    s.ren(&mut src);


    src.write_file(Some(source_content));

}