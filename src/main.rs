use regex::Regex;

fn get_match<T: std::convert::From<String>>(re: Regex, text: &str) -> Option<T> {
    if re.is_match(text) {
        let m: String = re
            .captures(text)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string();

        Some(m.into())
    } else {
        None
    }
}

macro_rules! Lexer{
       (while, $m:pat, $it:ident) => {
        {
            let mut str: String = "".to_string();
            while let Some(&c) = $it.peek() {
                match c {
                    $m => {
                        $it.next();
                        str.push(c);
                    }
                    _ => {
                        break
                    }
    
                }
           }                
                str
        }
        };

        (matcher: $($e:ident, $matcher:pat, $($type:ty)?, $($cont:ident)?),+) => {

            impl Lexer{
                fn lex(input: &String) -> Result<Vec<Lexer>, String> {
                    let mut result = Vec::new();
    
                    let mut it = input.chars().peekable();
                    while let Some(&c) = it.peek() {
                        match c {
    
                            $(
                            $matcher => {   
                                $(
                                let mut m = Lexer!($cont, $matcher, it);
                                result.push(Lexer::$e(m.parse().unwrap()));
                                continue;
                                )?
    
                                it.next();
                                result.push(Lexer::$e(c.to_string().parse().unwrap()));
    
                                
                            }
    
                            )+
    
    
    
                            ' ' | '\n' => {
                                it.next();
                            }
                            _ => {
                                return Err(format!("unexpected character {}", c));
                            }
                        }
                    }
            Ok(result)
                }
            }
    
        };


        (type: $($e:ident, $($type:ty)?),+) => {
            #[derive(Debug, Clone)]
            enum Lexer {
                $($e$(($type))?),+
            }
        };
        
        ($($e:ident$(($type:ty))? : $matcher:pat $(=> $cont:ident)?), +)=>{
            Lexer!(type: $($e, $($type)?),+);

            Lexer!(matcher: $($e, $matcher, $($type)?, $($cont)?),+);

        };

       
   // Second arm macth add!(1), add!(2) etc
       (old: $($e:ident $(($type:ty))? : $m:pat $(=> $cont:ident)?), +)=> {

        #[derive(Debug, Clone)]
        enum Lexer {
            $($e$(($type))?),+
        }

        impl Lexer{
            fn lex(input: &String) -> Result<Vec<Lexer>, String> {
                let mut result = Vec::new();

                let mut it = input.chars().peekable();
                while let Some(&c) = it.peek() {
                    match c {

                        $(

                        $m => {   
                            $(
                            let mut m = Lexer!($cont, $m, it);
                            result.push(Lexer::$e(m.parse().unwrap()));
                            continue;
                            )?

                            it.next();
                            result.push(Lexer::$e(c.to_string().parse().unwrap()));

                            
                        }

                        )+



                        ' ' | '\n' => {
                            it.next();
                        }
                        _ => {
                            return Err(format!("unexpected character {}", c));
                        }
                    }
                }
        Ok(result)
            }
        }

       }
   }

Lexer!(
    NUMBER(i32): '0'..='9' => while,
    E(char): 'e'
);

fn main() {
    let a:String = "123 \n 345e".to_string();

    let r = Lexer::lex(&a);

    

    println!("{:?}", r);
}
