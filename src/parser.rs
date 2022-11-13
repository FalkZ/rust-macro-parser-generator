
#[macro_export]
macro_rules! Parser {
    (      
            $(
                $rule_name:ident = 
                $((
                    $(
                        $($lex_or:ident
                            $(($lex_or_type:ty))?
                        )?
                        $(#$rule_or:ident)?
                    )|+
                ))? 
                $({
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
                })?
                // RECURSION RULE
                $([
                    $(
                        $(
                            $lex_rec_before:ident
                            $(($lex_rec_before_type:ty))?
                            $(=> $lex_rec_before_key:ident)?
                        )?
                        $(  #$rule_rec_before:ident 
                            $(=> $rule_rec_before_key:ident)?
                        )?
                    ,)*
                    *
                    $(,
                        $(
                            $lex_rec_after:ident
                            $(($lex_rec_after_type:ty))?
                            $(=> $lex_rec_after_key:ident)?
                        )?
                        $(  #$rule_rec_after:ident 
                            $(=> $rule_rec_after_key:ident)?
                        )?
                    )* | #$rec_break:ident
                ])?
            ),+
        
    ) => {
            use concat_idents::concat_idents;
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

                 // RECURSION RULE
                $(                   
                    
                concat_idents!(struct_name = $rule_name, _single {
                    #[derive(Debug, Clone)]
                    pub struct struct_name {
                    $(                     
                        $(
                            $($lex_rec_before_key:  $lex_rec_before,)?           
                        )?
                        $(    
                            $($rule_rec_before_key: Box<$rule_rec_before>,)?
                        )?
                    )*
                    $(
                        $(
                            $($lex_rec_after_key:  $lex_rec_after,)?           
                        )?
                        $(    
                            $($rule_rec_after_key: Box<$rule_rec_after>,)?
                        )?          
                    )*
                    }

                    type $rule_name = Vec<struct_name>;

                });
                    
                )?
                
                                       
            )+

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
                
                // RECURSIVE RULES
                $(
                    fn $rule_name(tokens: & Tokens<Lexer>) -> ParserResult<Box<$rule_name>> {
                        
                        let __p = tokens.pin();

                        let mut t = __p.get_pinned();
                        
                        $(
                            $(
                                let a = mat!(t, $lex_rec_before$(($lex_rec_before_type))?, $lex_rec_before);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $lex_rec_before_key = a?;)?
                            )?
                            $( 
                                let a = mat!(t, #$rule_rec_before);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_before_key = a?;)?
                            )?
                        )*

                        let __p2 = t.pin();
                        let __t2 = __p2.get_pinned();

                        let __rest = match Self::$rule_name(&__t2){
                            Ok(r) => {      
                                r
                            },
                            Err(v) => {
                                t = __p2.get_pinned();
                                println!("{:?}", v);
                                Box::new(vec![])
                            }
                        };
                    

                        $(
                            $(
                                let a = mat!(t, $lex_rec_after$(($lex_rec_after_type))?, $lex_rec_after);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $lex_rec_after_key = a?;)?
                            )?
                            $( 
                                let a = mat!(t, #$rule_rec_after);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_after_key = a?;)?
                            )?
                        )*

                        concat_idents!(struct_name = $rule_name, _single 
                        {
                        let __this = struct_name {
                            $(
                                $($(    
                                    $lex_rec_before_key,
                                )?)?
                                $($(    
                                    $rule_rec_before_key,
                                )?)?
                            )*
                            $(
                                $($(    
                                    $lex_rec_after_key,
                                )?)?
                                $($(    
                                    $rule_rec_after_key,
                                )?)?
                            )*
                        };
                        });

                        let mut __r = vec![__this];
                    
                        __r.extend(*__rest);
                     
                
                        Ok(Box::new(__r))
                    }
                )? 
            )+
            }
        };
}
