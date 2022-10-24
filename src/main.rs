macro_rules! Lexer{
       
    
         // default match breaks after 1 char
        (match: $matcher:pat, $it:ident) => {
            Lexer!(match: $matcher, $it, break)
        };

        (match: $matcher:pat, $it:ident, continue, $until:pat) => {
            {
                let mut str: String = "".to_string();           
               
                str.push(*$it.peek().expect("called with no chars left"));
                $it.next();

                while let Some(&c) = $it.peek() {
                    match c {
                        $until => {
                            $it.next();
                            str.push(c);
                            break
                        }
                        _ => {
                            $it.next();
                            str.push(c);    
                        }
        
                    }
                }                
                str
            }
        };


        (match: $matcher:pat, $it:ident, $repetition:ident) => {
        {
            let mut str: String = "".to_string();
            while let Some(&c) = $it.peek() {
                match c {
                    $matcher => {
                        $it.next();
                        str.push(c);
                        $repetition;
                    }
                    _ => {
                        break
                    }
    
                }
            }                
            str
        }
        };

       
        (matcher: 
            $($token_name:ident, $matcher:pat, $($type:ty)?, $(=> $(($until:pat))?)?),+ 
            $(, ($skip:pat))?
        ) => {
          
                fn primary_pass(input: &String) -> Result<Vec<Lexer>, String> {
                    let mut result = Vec::new();
    
                    let mut it = input.chars().peekable();
                    while let Some(&c) = it.peek() {
                        match c {
    
                            $(
                            $matcher => {   
                                
                                let _m = Lexer!(match: $matcher, it $(, continue $(, $until)?)?);
                                
                                result.push(
                                Lexer::$token_name$(({
                                    let val: $type = _m.parse().unwrap();
                                    val
                                }))?
                                );
                                continue;   
                                
                            }
    
                            )+
    
                            $(
                                $skip => {
                                    it.next();
                                }
                            )?
    
                           
                            _ => {
                                return Err(format!("unexpected character {:?}", c));
                            }
                        }
                    }
            Ok(result)
                }              
        };

        (@secondary: 
            $(
                $matched_type:ident => {
                    $(
                        { $secondary_pattern:pat } => $secondary_token_name:ident$(($secondary_type:ty))?

                    ),+
                }
            ),*
        ) => {

            fn secondary_pass(input: Vec<Lexer>) -> Vec<Lexer> {  

                input.into_iter().map(|l|{        
                    match l {
                        $(                                                 
                            Lexer::$matched_type(value) => {
                                match value {
                                    $( 
                                        $secondary_pattern => {
                                            Lexer::$secondary_token_name$(({
                                                let val: $secondary_type = value.parse().unwrap();
                                                val
                                            }))?
                                        }
                                    )+
                                    _ => {l}
                                }
                            }
                       )*
                        _ => {l}
                    }

                }).collect()
            }

        };


        (secondary: 
            $($token_name_secondary:ident $(, $type_secondary:ty)? , $({$matched_string:ident })? $matched_type:pat ),*
        ) => {
            fn secondary_pass(input: Vec<Lexer>) -> Vec<Lexer> {

              

                input.into_iter().map(|l|{

                    $($(
                        let $matched_string: String = stringify!($matched_string).to_string();
                        
                      
                    )?)*
                    
                    match l {
                        $(                                                 
                            $matched_type$((_/* $type_secondary */))? => {
                                Lexer::$token_name_secondary$(({
                                    let val: $type_secondary = m.parse().unwrap();
                                    val
                                }))?

                            }
                       )*
                        _ => {l}
                    }

                }).collect()
            }


        };


        (type: $($token_name:ident, $($type:ty)?),+) => {
            #[derive(Debug, Clone)]
            enum Lexer {
                $($token_name$(($type))?),+
            }
        };
        
        // Entrypoint
        (
            {
               $({$matcher:pat $(=> $($until:pat)?)? } => $token_name:ident$(($type:ty))? ),+ 
              
            }
            
            $({$skip:pat} => _)?
            
            {
                $(
                    $matched_type:ident => {
                        $(
                            { $secondary_pattern:pat } => $secondary_token_name:ident$(($secondary_type:ty))?

                        ),+
                    }
                ),*
            }
        )=>{
            
            
            Lexer!(type: 
                $($token_name, $($type)?),+  
                $(,$( $secondary_token_name, $($secondary_type)?),*)*
            );

            impl Lexer {
                Lexer!(matcher: $($token_name, $matcher, $($type)?, $(=> $(($until))?)?),+ $(, ($skip))?);
                /* 
                Lexer!(secondary: 
                 $($token_name_secondary $(, $type_secondary)? , $({ $matched_string })?  $matched_type ),*
                );*/

                Lexer!(@secondary: 
                    $(
                        $matched_type => {
                            $(
                                { $secondary_pattern } => $secondary_token_name$(($secondary_type))?
    
                            ),+
                        }
                    ),*
                );



                fn lex(input: &String) -> Result<Vec<Lexer>, String> {

                    let first = Lexer::primary_pass(input)?;

                   
                    Ok(Lexer::secondary_pass(first))
                 
                }
            
            }

        };

       
 }



 Lexer!(
    {
        {'0'..='9' =>} => NUMBER(i32),
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL(String),
        {'+'} => PLUS
    
    }
    
    { ' ' | '\n' } => _
    
    {
        NUMBER => { {123} => T123 }
    } 
);
 

macro_rules! Test{
    ($({$start:pat $(=> $($end:pat)?)?} => $target:path),+) => {
       $({ println!("{}", stringify!($start));
        $(
            println!("=>");
            $(
                println!("{}", stringify!($end));
            )?
        )?
        println!("{}", stringify!($target));
    };)+
};
}


fn main() {
    let a:String = "123 \n + 345 12 'text' ".to_string();

   
    let t = Lexer::lex(&a).unwrap();

 



    Test!(
        {'0'..='9'} => Lexer::NUMBER,
        {'0'..='9' =>} => Lexer::NUMBER,
        {'0'..='9' => '0'..='9'} => Lexer::NUMBER
    );
    

    println!("{:?}", t);
}
