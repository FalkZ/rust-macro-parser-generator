use crate::{grammar::{statements, statement, function, variable, maybe_arguments, arguments, argument, }, sourcemap::{RenderContext, Pos, Position, Token}, render};


#[macro_export]
macro_rules! renderer {
    ($self:ident, $context:ident, $template:expr $(,$ins:expr$(=>$arg:expr)?)*) => {
        {
            let own_pos = $self.position();
    
            let mut strings =  $template.split("{}");


            $context.add_token(strings.next().unwrap(), &own_pos);

            $(
                $ins.render($context, $($arg)?);
                $context.add_string(strings.next().unwrap());
            )*
        }     
    };
}

pub trait Render: Pos {
    fn render(&self, context: &mut RenderContext);
}



pub trait ContextRender<T>: Pos {
    fn render(&self, context: &mut RenderContext, context: T);
}


impl Render for argument {
    fn render(&self, context: &mut RenderContext) {
        renderer!(self, context, "{}", self.arg);
    }
}

impl Render for Vec<argument> {
    fn render(&self, context: &mut RenderContext) {
        self.iter().for_each(|v| renderer!(self, context, "{},", v));
    }
}

impl Render for arguments {
    fn render(&self, context: &mut RenderContext) {
        renderer!(self, context, "{}{}", self.arguments, self.last);
    }
}

impl Render for maybe_arguments {
    fn render(&self, context: &mut RenderContext) {
        match self {
            maybe_arguments::no_arguments(_) => renderer!(self, context, "()"),
            maybe_arguments::arguments(v) => renderer!(self, context, "({})", v)
        }
    }
}

impl Render for function  {
    fn render(&self, context: &mut RenderContext) {      
        renderer!(self, context, "fn{} \n",  self.arguments)
    }
}

impl Render for variable  {
    fn render(&self, context: &mut RenderContext) {
        renderer!(self, context, "var \n")
    }
}


impl Render for Vec<statements>  {
    fn render(&self, context: &mut RenderContext) {
        self
        .iter()
        .for_each(|v| match *v.statement.to_owned() {
            statement::function(v) => {
                renderer!(self, context, "statement: {}", v);
            }

            statement::variable(v) => {
                renderer!(self, context, "statement: {}", v);
            }

        });
        
    }
}



pub fn renderer(source_path: &str, source_content: &str,s: &Vec<statements>){

    let mut src = RenderContext::new(source_path);

    s.render(&mut src);


    src.write_file(Some(source_content));

}