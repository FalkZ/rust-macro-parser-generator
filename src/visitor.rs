#[macro_export]
macro_rules! impl_visitor {
    ($name:ident) => {
        fn $name(&self, _: &$name) -> R {
            todo!("visited '{}' but was not implemented in visitor", stringify!($name))
        }
    };
}

#[macro_export]
macro_rules! impl_visit {
    ($name:ident) => {
        impl Visit for $name {
            fn visit<R, V: Visitor<R>>(&self, v: &V)-> R {
                v.$name(&self)
            }
        }
    };
}



