
#[macro_export]
macro_rules! return_if_match {
    ($parser_result:ident) => {
        match $parser_result {
            Ok(m) => return Ok(Box::new(m)),
            _ => (),
        }
    }
}


#[macro_export]
macro_rules! mat {
    ($pinned_tokens:expr, $name:ident, $name_out:path) => {
        {
            let tokens = $pinned_tokens;
            let next = tokens.next().or_message("next on EOF")?;
     
            match next {
                Lexer::$name => Ok($name_out),                 
                _ => Err(ParserError::Mismatch)
            }
        }
    };

    ($pinned_tokens:expr, $name:ident ($_:ty), $name_out:path) => {
        {
            let tokens = $pinned_tokens;
            let next = tokens.next().or_message("next on EOF")?;
     
            match next {
                Lexer::$name(val) =>  Ok($name_out(val.clone().into())),      
                _ =>  Err(ParserError::Mismatch)
            }
        }
    };

    ($pinned_tokens:expr, #$name:ident, $name_out:path) => {
        {
            match Self::$name($pinned_tokens) {
                Ok(val) => Ok($name_out(val)),
                _ =>  Err(ParserError::Mismatch)
            }
        }
    };

    ($pinned_tokens:expr, #$name:ident) => {
        {
           println!("#{} =>> {}", stringify!($name), $pinned_tokens);
           Self::$name($pinned_tokens)
        }
    };
}