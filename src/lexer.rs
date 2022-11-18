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

   // matches 1 or multiple
   (@MATCH: $matcher:pat, $it:ident, $repetition:ident) => {
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


   (@PRIMARY:
       $(
           $token_name:ident, $matcher:pat, $(=> $(($until:pat))?)?
       ),+
       $(, ($skip:pat))?
   ) => {

           fn primary_pass(input: &str) -> Result<Vec<Lexer>, String> {
               let mut result = Vec::new();

               let mut it = input.chars().peekable();
               while let Some(&c) = it.peek() {
                   match c {
                       $(
                           $matcher => {

                               let _m = Lexer!(@MATCH: $matcher, it $(, continue $(, $until)?)?);

                               result.push(
                                   Lexer::$token_name(TokenContent::new(_m))
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

       
       $(
        #[derive(Debug, Clone)]
            pub struct $token_name(pub TokenContent);
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

       type TokenContent = $crate::sourcemap::Token;

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
