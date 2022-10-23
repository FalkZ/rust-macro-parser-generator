macro_rules! Lexer{
       
    
         // default match breaks after 1 char
        (match: $matcher:pat, $it:ident) => {
            Lexer!(match: $matcher, $it, break)
        };

        (match: $matcher:pat, $it:ident, until($until:pat)) => {
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
            $($token_name:ident, $matcher:pat, $($type:ty)?, $($repetition:ident $(($until:pat))?)?),+ 
            $(, ($skip:pat))?
        ) => {
          
                fn primary_pass(input: &String) -> Result<Vec<Lexer>, String> {
                    let mut result = Vec::new();
    
                    let mut it = input.chars().peekable();
                    while let Some(&c) = it.peek() {
                        match c {
    
                            $(
                            $matcher => {   
                                
                                let _m = Lexer!(match: $matcher, it $(, $repetition$(($until))?)?);
                                
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

        (secondary: 
            $($token_name_secondary:ident $(, $type_secondary:ty)? , $matched_type:pat),*
        ) => {
            fn secondary_pass(input: Vec<Lexer>) -> Vec<Lexer> {

                input.into_iter().map(|l|{
                    match l {
                        $(
                            
                        
                            $matched_type => {
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
                $($token_name:ident$(($type:ty))? : $matcher:pat $(=> $repetition:ident$(($until:pat))?)?),+ 
                $(,_: $skip:pat)?
            }
            {
                $($token_name_secondary:ident$(($type_secondary:ty))? : $matched_type:pat),*
            }
        )=>{
            
            
            Lexer!(type: $($token_name, $($type)?),+  $(, $token_name_secondary, $($type_secondary)?)+);

            impl Lexer {
                Lexer!(matcher: $($token_name, $matcher, $($type)?, $($repetition$(($until))?)?),+ $(, ($skip))?);
                Lexer!(secondary: 
                 $($token_name_secondary $(, $type_secondary)? , $matched_type),*
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
        NUMBER(i32): '0'..='9' => continue,
        IDENT(String): 'A'..='z' => continue,
        TEXTLITERAL(String): '"' | '\'' => until('"' | '\''),
        PLUS: '+',
        MINUS: '-',
        DIVISION: '/',
        MULTIPLICATION: '*',
        POWER: '^',
        BRACKETOPEN: '(',
        BRACKETCLOSE: ')',
        CBRACKETOPEN: '{',
        CBRACKETCLOSE: '}',    
        _: '\n' | ' '
    }
    {
        T123: Lexer::NUMBER(123 | 12)
    } 
);
 


fn main() {
    let a:String = "123 \n + 345 add 12 'text'  () { 123 }".to_string();

   
    let t = Lexer::lex(&a);

    

    println!("{:?}", t);
}
