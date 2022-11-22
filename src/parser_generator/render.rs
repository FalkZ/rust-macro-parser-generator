use super::{position::GetPosition, sourcemap::RenderContext, traits::RawToken};

#[macro_export]
macro_rules! render {
    ($self:ident, $context:ident, $template:expr $(,$ins:expr$(=>$arg:expr)?)*) => {
        {
            use $crate::parser_generator::{position::GetPosition};

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

pub trait RenderDefault: GetPosition {
    fn render_default(&self, context: &mut RenderContext);
}

impl<T: Render> RenderDefault for Vec<T> {
    fn render_default(&self, context: &mut RenderContext) {
        self.iter().for_each(|v| {
            render!(self, context, "{}", v);
        });
    }
}

pub trait Render: GetPosition {
    fn render(&self, context: &mut RenderContext);
}

pub trait RenderWithArg<Arg>: GetPosition {
    fn render(&self, context: &mut RenderContext, arg: Arg);
}

impl<T: RawToken> Render for T {
    fn render(&self, context: &mut RenderContext) {
        let raw = self.raw_token();
        context.add_token(raw.as_str(), &raw.position)
    }
}
