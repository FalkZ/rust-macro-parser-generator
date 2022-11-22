#[macro_export]
macro_rules! render {
    ($template:expr $(,$ins:expr)*) => {
        {
            let val: std::vec::IntoIter<Renderable> = vec![$(Renderable::Ren(Box::new($ins))),*].into_iter();

            let i =  $template.split("{}");

            let mut r = vec![Renderable::String(i.next().unwrap().to_string())];
           
            i.for_each(|s| {
                r.push(val.next().unwrap());
                r.push(Renderable::String(s.to_string()));
            });

            r
        }  
    };
}