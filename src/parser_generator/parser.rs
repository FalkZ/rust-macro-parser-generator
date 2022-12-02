#[macro_export]
macro_rules! Parser {
    (
            $(
                $rule_name:ident =
                // ENUM RULE
                $((
                    $(
                        $($lex_or:ident)?
                        $(#$rule_or:ident)?
                        $(*$rule_or_iter:ident)?
                    )|+
                ))?
                // STRUCT RULE
                $({
                    $(
                        $(
                            $lex_and:ident
                            $(=> $enum_key:ident)?
                        )?
                        $(
                            _$skip_and:ident
                            $(=> $skip_enum_key:ident)?
                        )?
                        $(  #$rule_and:ident
                            $(=> $rule_enum_key:ident)?
                        )?
                        $(  ?$rule_and_opt:ident
                            $(=> $rule_enum_key_opt:ident)?
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
                        $(  ?$rule_rec_before_opt:ident
                            $(=> $rule_rec_before_key_opt:ident)?
                        )?
                        $(  *$rule_rec_before_iter:ident
                            $(=> $rule_rec_before_key_iter:ident)?
                        )?
                    ,)*
                    $(
                        ($lex_rec_before_not_last:ident)
                    )?
                    *
                    $(
                        ($lex_rec_after_not_last:ident)
                    )?
                    $(,
                        $(
                            $lex_rec_after:ident
                            $(=> $lex_rec_after_key:ident)?
                        )?
                        $(  #$rule_rec_after:ident
                            $(=> $rule_rec_after_key:ident)?
                        )?
                        $(  ?$rule_rec_after_opt:ident
                            $(=> $rule_rec_after_key_opt:ident)?
                        )?
                        $(  *$rule_rec_after_iter:ident
                            $(=> $rule_rec_after_key_iter:ident)?
                        )?

                    )*
                ])?
            ),+

    ) => {
            type P = $crate::parser_generator::position::Position;
            use $crate::parser_generator::{position::GetPosition, tokens::Tokens, result::{ParserResult, ParserError}, traits::{OrErrString}};
            use $crate::{match_or_err, match_opt, return_if_match, return_end_if_missmatch, match_maybe};

            $(
                $(
                    #[allow(non_camel_case_types)]
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

                    impl GetPosition for $rule_name {
                        fn position(&self)-> P {
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
                    #[allow(non_camel_case_types)]
                    #[derive(Debug, Clone)]
                    pub struct $rule_name {
                        position: P,
                    $(

                        $(
                            $(pub $enum_key: $lex_and,)?
                        )?
                        $(
                            $(pub $rule_enum_key: Box<$rule_and>,)?
                        )?
                        $(
                            $(pub $rule_enum_key_opt: Option<Box<$rule_and_opt>>,)?
                        )?
                        $(
                            $(pub $rule_enum_key_iter: Vec<$rule_and_iter>,)?
                        )?
                    )+
                    }

                    impl GetPosition for $rule_name {
                        fn position(&self)-> P {
                            self.position.clone()
                        }
                    }
                )?

                 // RECURSION RULE
                $(
                    #[allow(non_camel_case_types)]
                    #[derive(Debug, Clone)]
                    pub struct $rule_name {
                        position: P,
                    $(
                        $(
                            $(pub $lex_rec_before_key:  $lex_rec_before,)?
                        )?
                        $(
                            $(pub $rule_rec_before_key: Box<$rule_rec_before>,)?
                        )?
                        $(
                            $(pub $rule_rec_before_key_opt: Option<Box<$rule_rec_before_opt>>,)?
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
                            $(pub $rule_rec_after_key_opt: Option<Box<$rule_rec_after_opt>>,)?
                        )?
                        $(
                            $(pub $rule_rec_after_key_iter: Vec<$rule_rec_after_iter>,)?
                        )?
                    )*
                    }
                    impl GetPosition for $rule_name {
                        fn position(&self)-> P {
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
                                let a = match_or_err!(pin.get_pinned(), $lex_or, $rule_name::$lex_or);
                                return_if_match!(a);
                            )?

                            $(
                                let a = match_or_err!(pin.get_pinned(), #$rule_or, $rule_name::$rule_or);
                                return_if_match!(a);
                            )?

                            $(
                                let a = match_or_err!(pin.get_pinned(), #$rule_or_iter, $rule_name::$rule_or_iter);
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
                                $(let $enum_key =)? match_or_err!(tokens, $lex_and, $lex_and)?
                            )?
                            $(
                                match_or_err!(tokens, _ $skip_and)?
                            )?
                            $(
                                $(let $rule_enum_key =)? match_or_err!(tokens, #$rule_and)?
                            )?
                            $(
                                $(let $rule_enum_key_opt =)? match_opt!(tokens, $rule_and_opt)
                            )?
                            $(
                                $(let $rule_enum_key_iter =)? match_or_err!(tokens, #$rule_and_iter)?
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
                                $rule_enum_key_opt,
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

                        let t = __p.get_pinned();

                        $(
                            $(
                                let a = match_or_err!(t, $lex_rec_before, $lex_rec_before);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $lex_rec_before_key = a?;)?
                            )?
                            $(
                                let a = match_or_err!(t, #$rule_rec_before);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_before_key = a?;)?
                            )?
                            $(
                                $(let $rule_rec_before_key_opt =)? match_opt!(t, $rule_rec_before_opt);
                            )?
                            $(
                                let a = match_or_err!(t, #$rule_rec_before_iter);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_before_key_iter = a?;)?
                            )?
                        )*

                        let __p2 = t.pin();
                        let __t2 = __p2.get_pinned();

                        let __matched =
                            $(
                                match_maybe!(t, $lex_rec_before_not_last);
                            )?
                        true;

                        let __rest = if __matched
                            {
                            match Self::$rule_name(&__t2){
                                Ok(r) => {

                                    let __matched =
                                    $(
                                        match_maybe!(t, $lex_rec_after_not_last);
                                    )?
                                    true;

                                    if __matched {
                                        r

                                    } else {
                                        __p2.get_pinned();
                                        vec![]
                                    }
                                },
                                Err(v) => {
                                    __p2.get_pinned();
                                    println!("{:?}", v);
                                    vec![]
                                }
                            }
                        } else {
                            vec![]
                        };


                        $(
                            $(
                                let a = match_or_err!(t, $lex_rec_after, $lex_rec_after);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $lex_rec_after_key = a?;)?
                            )?
                            $(
                                let a = match_or_err!(t, #$rule_rec_after);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_after_key = a?;)?
                            )?
                            $(
                                $(let $rule_rec_after_key_opt =)? match_opt!(t, $rule_rec_after_opt);
                            )?
                            $(
                                let a = match_or_err!(t, #$rule_rec_after_iter);
                                return_end_if_missmatch!($rule_name, a, __p);
                                $(let $rule_rec_after_key_iter = a?;)?
                            )?
                            $(
                               match_maybe!(t, $lex_rec_after_not_last);
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
                                $($(
                                    $rule_rec_before_key_opt,
                                )?)?
                                $($(
                                    $rule_rec_before_key_iter,
                                )?)?
                            )*
                            $(
                                $($(
                                    $lex_rec_after_key,
                                )?)?
                                $($(
                                    $rule_rec_after_key,
                                )?)?
                                $($(
                                    $rule_rec_after_key_opt,
                                )?)?
                                $($(
                                    $rule_rec_after_key_iter,
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
