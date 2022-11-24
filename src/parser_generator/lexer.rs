#[macro_export]
macro_rules! Lexer {

    // default match breaks after 1 char
   (@MATCH: $matcher:pat, $it:ident) => {
       Lexer!(@MATCH: $matcher, $it, break)
   };

   // matches until other pattern is reached
   (@MATCH: $matcher:pat, $it:ident, continue, $until:pat) => {
       {
           let mut str: String = "".to_string();

           str.push($it.peek().expect("called with no chars left"));
           $it.next();

           while let Some(c) = $it.peek() {
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

   // matches 1 or multiple
   (@MATCH: $matcher:pat, $it:ident, $repetition:ident) => {
   {
       let mut str: String = "".to_string();
       while let Some(c) = $it.peek() {
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


   (@PRIMARY:
       $(
           $token_name:ident, $matcher:pat, $(=> $(($until:pat))?)?
       ),+
       $(, ($skip:pat))?
   ) => {

           fn primary_pass(input: &str) -> Result<Vec<Lexer>, String> {
               let mut result = Vec::new();

               let mut it = $crate::parser_generator::source::Source::new(input);

               while let Some(c) = it.peek() {
                   match c {
                       $(
                           $matcher => {
                               
                              let position = it.get_position();

                               let token = Lexer!(@MATCH: $matcher, it $(, continue $(, $until)?)?);

                               result.push(
                                   Lexer::$token_name(TokenContent::new(token, position))
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

   (@SECONDARY:
       $(
           $matched_type:ident => {
               $(
                   { $secondary_pattern:pat } => $secondary_token_name:ident

               ),+
           }
       ),*
   ) => {

       fn secondary_pass(input: Vec<Lexer>) -> Vec<Lexer> {

           input.into_iter().map(|l|{
               match l.clone() {
                   $(
                       Lexer::$matched_type(value) => {
                           match value.as_str() {
                               $(
                                   $secondary_pattern => {
                                       Lexer::$secondary_token_name(value)
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


   (@ENUM:
       $(
           $token_name:ident
       ),+
   ) => {

       #[derive(Debug, Clone)]
       pub enum Lexer {
           $($token_name(TokenContent)),+
       }

       type T =  $crate::parser_generator::tokens::Token;
       impl Into<T> for Lexer{
            fn into(self) -> T {
                match self {
                    $(Lexer::$token_name(v) => v),+
                }        
            }
        }

       impl std::fmt::Display for Lexer {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $crate::parser_generator::tokens::RawToken;

                let name = format!("{:?}", self);
                let name = name.split("(").next().unwrap_or(&name);
                write!(f, "{}({})", name, self.raw_token().as_str())
            }
        }

       
       $(
        #[derive(Debug, Clone)]
            pub struct $token_name(pub TokenContent);

            impl $crate::parser_generator::tokens::RawToken for $token_name {
                fn raw_token(&self) -> TokenContent { 
                    self.0.clone()
                }
            }
        )+
       
   };

   // ENTRYPOINT
   (
       { // FIRST PASS
          $(
               { $matcher:pat $(=> $($until:pat)? )? } => $token_name:ident
           ),+
       }

       // SKIP PATTERN
       $({ $skip:pat } => _)?

       { // SECOND PASS
           $(
               $matched_type:ident => {
                   $(
                       { $secondary_pattern:pat } => $secondary_token_name:ident
                   ),+
               }
           ),*
       }
   ) => {

       type TokenContent = $crate::parser_generator::tokens::Token;
       type Position = $crate::parser_generator::position::Position;

       Lexer!(
           @ENUM:
           $($token_name),+
           $(,$( $secondary_token_name),*)*
       );

       impl Lexer {

           Lexer!(@PRIMARY:
               $(
                   $token_name, $matcher, $(=> $(($until))?)?
               ),+
               $(, ($skip))?
           );

           Lexer!(@SECONDARY:
               $(
                   $matched_type => {
                       $(
                           { $secondary_pattern } => $secondary_token_name
                       ),+
                   }
               ),*
           );



           fn lex(input: &str) -> Result<Vec<Lexer>, String> {
               let first = Lexer::primary_pass(input)?;
               Ok(Lexer::secondary_pass(first))
           }
        
           
       }
   };
}
