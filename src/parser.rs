
#[macro_export]
macro_rules! Parser {
    (@OMIT: omit:tt) => {};
    (
        {
            $(
                $token_name:ident
                $(
                    ($type:ty)
                )?
            ),+
        }{
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
                        $($lex_and:ident
                            $(($lex_and_type:ty))?
                        )?
                        $(#$rule_and:ident)?
                    ),+
                ))?
            ),+
        }
    ) => {
            $( 
                $(
                    #[derive(Debug, Clone, Copy)]
                    struct $token_name($type);               
                )?
            )+
            $(
                $(
                    #[derive(Debug, Clone)]
                    enum $rule_name {
                    $(
                        $(
                            $lex_or$(($lex_or_type)
                        )?
                            
                        )?
                        $(    
                            $rule_or(Box<$rule_or>)
                        )?
                    ,)+
                    }
                )? 
                $(
                    #[derive(Debug, Clone)]
                    struct $rule_name (
                    $(
                        $(
                            $lex_and   
                        )?
                        $(    
                            Box<$rule_and>
                        )?
                    ),+
                    );
                )? 
              
            )+

            #[derive(Debug)]
            struct Par {
                tokens: Vec<Lexer>,
                stack: Vec<Lexer>,
            }

            impl Par{
                fn new(str: &str) -> ParserResult<Self> {
                    let tokens = Lexer::lex(str).or_err()?;
                    return Ok(Self {
                        tokens,
                        stack: vec![],
                    });
                }

               

              

                fn start(self) -> ParserResult<()> {
                    let iter: Iter<Lexer> = self.tokens.iter();

                   // Self::parse_term(iter.clone())?;

                    Ok(())
                }
            }
        };
}
