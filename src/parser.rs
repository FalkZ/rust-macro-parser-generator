
#[macro_export]
macro_rules! Parser {
    (      
            $(
                $rule_name:ident = 
                $((
                    $(
                        $($lex_or:ident)?
                        $(#$rule_or:ident)?
                        $(*$rule_or_iter:ident)?
                    )|+
                ))? 
                $({
                    $(
                        $(
                            $lex_and:ident
                            $(=> $enum_key:ident)?
                        )?
                        $(  #$rule_and:ident 
                            $(=> $rule_enum_key:ident)?
                        )?
                        $(  *$rule_and_iter:ident 
                            $(=> $rule_enum_key_iter:ident)?
                        )?
                    ),+
                })?
                // RECURSION RULE
                $([
                    $(
                        $(
                            $lex_rec_before:ident
                            $(=> $lex_rec_before_key:ident)?
                        )?
                        $(  #$rule_rec_before:ident 
                            $(=> $rule_rec_before_key:ident)?
                        )?
                        $(  *$rule_rec_before_iter:ident 
                            $(=> $rule_rec_before_key_iter:ident)?
                        )?
                    ,)*
                    *
                    $(,
                        $(
                            $lex_rec_after:ident
                            $(=> $lex_rec_after_key:ident)?
                        )?
                        $(  #$rule_rec_after:ident 
                            $(=> $rule_rec_after_key:ident)?
                        )?
                        $(  *$rule_rec_after_iter:ident 
                            $(=> $rule_rec_after_key_iter:ident)?
                        )?
                    )*
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
                            $lex_or(TokenContent)
                        
                        )?
                        $(    
                            $rule_or(Box<$rule_or>)
                        )?
                        $(    
                            $rule_or_iter(Vec<$rule_or_iter>)
                        )?
                    ,)+
                    } 

                    impl $crate::sourcemap::Pos for $rule_name {
                        fn position(&self)-> $crate::sourcemap::Position {
                            match self {
                                $(
                                    $(
                                        $rule_name::$lex_or(v) => v.position()
                                    )?
                                    $(    
                                        $rule_name::$rule_or(v) => v.position()
                                    )?
                                    $(    
                                        $rule_name::$rule_or_iter(v) => v.position()
                                    )?
                                ,)+
                            }
                        }
                    }
                )? 

                $(
                    #[derive(Debug, Clone)]
                    pub struct $rule_name {
                        position: $crate::sourcemap::Position,
                    $(
                        
                        $(
                            $(pub $enum_key: $lex_and,)?           
                        )?
                        $(    
                            $(pub $rule_enum_key: Box<$rule_and>,)?
                        )?
                        $(    
                            $(pub $rule_enum_key_iter: Vec<$rule_and_iter>,)?
                        )?         
                    )+
                    }

                    impl $crate::sourcemap::Pos for $rule_name {
                        fn position(&self)-> $crate::sourcemap::Position {
                            self.position.clone()
                        }
                    }
                )?

                 // RECURSION RULE
                $(                            
                    #[derive(Debug, Clone)]
                    pub struct $rule_name {
                        position: $crate::sourcemap::Position,
                    $(                     
                        $(
                            $(pub $lex_rec_before_key:  $lex_rec_before,)?           
                        )?
                        $(    
                            $(pub $rule_rec_before_key: Box<$rule_rec_before>,)?
                        )?
                        $(    
                            $(pub $rule_rec_before_key_iter: Vec<$rule_rec_before_iter>,)?
                        )?
                    )*
                    $(
                        $(
                            $(pub $lex_rec_after_key:  $lex_rec_after,)?           
                        )?
                        $(    
                            $(pub $rule_rec_after_key: Box<$rule_rec_after>,)?
                        )?  
                        $(    
                            $(pub $rule_rec_after_key_iter: Vec<$rule_rec_after_iter>,)?
                        )?            
                    )*
                    }
                    impl $crate::sourcemap::Pos for $rule_name {
                        fn position(&self)-> $crate::sourcemap::Position {
                            self.position.clone()
                        }
                    }
                    
                )?                                                
            )+

            #[derive(Debug)]
            pub struct Parser {
                pub tokens: Tokens<Lexer>,
            }
            
            impl Parser {
                pub fn new(str: &str) -> ParserResult<Self> {
                    let tokens = Lexer::lex(str).or_err()?;
            
                    
                    return Ok(Self {
                        tokens: Tokens::new(tokens),
                    });
                }
            $(

                // OR RULES
                $(
                    pub fn $rule_name(tokens: &Tokens<Lexer>) -> ParserResult<Box<$rule_name>> {
                        let pin = tokens.pin();
                        $(
                            $(
                                let a = mat!(pin.get_pinned(), $lex_or, $rule_name::$lex_or);
                                return_if_match!(a);
                            )?
                            
                            $(   
                                let a = mat!(pin.get_pinned(), #$rule_or, $rule_name::$rule_or);
                                return_if_match!(a); 
                            )?

                            $(   
                                let a = mat!(pin.get_pinned(), #$rule_or_iter, $rule_name::$rule_or_iter);
                                return_if_match!(a); 
                            )?
                        )+

                        Err(ParserError::UnreachableAt(stringify!($rule_name).to_string()))
                    }    
                )?
                // AND RULES
                $(
                    pub fn $rule_name(tokens: &Tokens<Lexer>) -> ParserResult<Box<$rule_name>> {
                        let __p: Option<Position> = tokens.position();
                        $(
                            $(
                                $(let $enum_key =)? mat!(tokens, $lex_and, $lex_and)? 
                            )?
                            $( 
                                $(let $rule_enum_key =)? mat!(tokens, #$rule_and)?
                            )?
                            $( 
                                $(let $rule_enum_key_iter =)? mat!(tokens, #$rule_and_iter)?
                            )?
                        ;)+


                        let __r = $rule_name{
                            position: __p.expect("couldn't get position for token"),
                        $(
                            $($(    
                                $enum_key,
                            )?)?
                            $($(    
                                $rule_enum_key,
                            )?)?
                            $($(    
                                $rule_enum_key_iter,
                            )?)?
                        )+
                        };
                     
                
                        Ok(Box::new(__r))
                    }
                )?
                
                // RECURSIVE RULES
                $(
                    pub fn $rule_name(tokens: & Tokens<Lexer>) -> ParserResult<Vec<$rule_name>> {
                        let __pos = tokens.position();
                        let __p = tokens.pin();

                        let mut t = __p.get_pinned();
                        
                        $(
                            $(
                                let a = mat!(t, $lex_rec_before, $lex_rec_before);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $lex_rec_before_key = a?;)?
                            )?
                            $( 
                                let a = mat!(t, #$rule_rec_before);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_before_key = a?;)?
                            )?
                            $( 
                                let a = mat!(t, #$rule_rec_before_iter);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_before_key_iter = a?;)?
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
                                vec![]
                            }
                        };
                    

                        $(
                            $(
                                let a = mat!(t, $lex_rec_after, $lex_rec_after);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $lex_rec_after_key = a?;)?
                            )?
                            $( 
                                let a = mat!(t, #$rule_rec_after);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_after_key = a?;)?
                            )?
                            $( 
                                let a = mat!(t, #$rule_rec_after_iter);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_after_key_iter = a?;)?
                            )?
                        )*

                        
                        let __this = $rule_name {
                            position: __pos.expect("couldn't get position for token"),
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

                        let mut __r = vec![__this];
                    
                        __r.extend(__rest);
                     
                
                        Ok(__r)
                    }
                )? 
            )+
            }
        };
}
