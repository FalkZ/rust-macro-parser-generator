
#[macro_export]
macro_rules! Parser {
    (      
            $(
                $rule_name:ident = 
                $({
                    $(
                        $($lex_or:ident
                            $(($lex_or_type:ty))?
                        )?
                        $(#$rule_or:ident)?
                    )|+
                })? 
                $((
                    $(
                        $(
                            $lex_and:ident
                            $(($lex_and_type:ty))?
                            $(=> $enum_key:ident)?
                        )?
                        $(  #$rule_and:ident 
                            $(=> $rule_enum_key:ident)?
                        )?
                    ),+
                ))?
            ),+
        
    ) => {
            $(
                $(
                    #[derive(Debug, Clone)]
                    pub enum $rule_name {
                    $(
                        $(
                            $lex_or$(($lex_or_type))?
                        
                        )?
                        $(    
                            $rule_or(Box<$rule_or>)
                        )?
                    ,)+
                    } 
                )? 
                $(
                    #[derive(Debug, Clone)]
                    pub struct $rule_name {
                    $(
                        
                        $(
                            $($enum_key: $lex_and,)?           
                        )?
                        $(    
                            $($rule_enum_key: Box<$rule_and>,)?
                        )?         
                    )+
                    }
                )?
                
                impl_visit!($rule_name);                       
            )+

            pub trait Visitor<R> {
                $(impl_visitor!($rule_name);)+
            }

            pub trait Visit {
                fn visit<R, V: Visitor<R>>(&self, visitor: &V)-> R;
            }

            #[derive(Debug)]
            struct Parser {
                tokens: Tokens<Lexer>,
            }
            
            impl Parser {
                fn new(str: &str) -> ParserResult<Self> {
                    let tokens = Lexer::lex(str).or_err()?;
            
                    
                    return Ok(Self {
                        tokens: Tokens::new(tokens),
                    });
                }
            $(

                // OR RULES
                $(
                    fn $rule_name(tokens: & Tokens<Lexer>) -> ParserResult<Box<$rule_name>> {
                        let pin = tokens.pin();
                        $(
                            $(
                                let a = mat!(pin.get_pinned(), $lex_or$(($lex_or_type))?, $rule_name::$lex_or);
                                return_if_match!(a);
                            )?
                            
                            $(   
                                let a = mat!(pin.get_pinned(), #$rule_or, $rule_name::$rule_or);
                                return_if_match!(a); 
                            )?
                        )+

                        Err(ParserError::UnreachableAt(stringify!($rule_name).to_string()))
                    }    
                )?
                // AND RULES
                $(
                    fn $rule_name(tokens: & Tokens<Lexer>) -> ParserResult<Box<$rule_name>> {
                        $(
                            $(
                                $(let $enum_key =)? mat!(tokens, $lex_and$(($lex_and_type))?, $lex_and)? 
                            )?
                            $( 
                                $(let $rule_enum_key =)? mat!(tokens, #$rule_and)?
                            )?
                        ;)+


                        let __r = $rule_name{
                        $(
                            $($(    
                                $enum_key,
                            )?)?
                            $($(    
                                $rule_enum_key,
                            )?)?
                        )+
                        };
                     
                
                        Ok(Box::new(__r))
                    }
                )? 
            )+
            }
        };
}
