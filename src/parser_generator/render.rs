use super::{position::{GetPosition, TryGetPosition}, sourcemap::RenderContext, traits::RawToken};

#[macro_export]
macro_rules! render {
    ($self:ident, $context:ident, $template:expr $(,$ins:expr$(=>$arg:expr)?)*) => {
        {
            use $crate::parser_generator::{position::GetPosition};

            /* 
            let own_pos = $self.position();

            let mut strings =  $template.split("{}");


            $context.add_token(strings.next().unwrap(), &own_pos);

            $(
                $ins.render($context, $($arg)?);
                $context.add_string(strings.next().unwrap());
            )*

            */
        }
    };
}



pub trait Render: TryGetPosition {
    fn render(&self, context: &mut RenderContext);
}





pub trait RenderWithArg<Arg>: TryGetPosition {
    fn render(&self, context: &mut RenderContext, arg: Arg);
}

