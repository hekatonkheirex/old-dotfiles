// auto-generated: "lalrpop 0.19.6"
// sha3: d46db4d8c154536dd0a2378c446dca75b5a172b7196c593e131703bc595e0
use std::str::FromStr;
use crate::parser::{lexer::Token, ast::Ast, parse_error};
use eww_shared_util::Span;
use simplexpr::ast::SimplExpr;
use simplexpr;
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Ast {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::parser::{lexer::Token, ast::Ast, parse_error};
    use eww_shared_util::Span;
    use simplexpr::ast::SimplExpr;
    use simplexpr;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(Vec<(usize, simplexpr::parser::lexer::Token, usize)>),
        Variant3(Ast),
        Variant4(alloc::vec::Vec<Ast>),
        Variant5(usize),
        Variant6(SimplExpr),
        Variant7((Span, Vec<Ast>)),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 3, 0, 13, 14, 15, 16, 17, 18, 19,
        // State 1
        2, 21, 3, 0, 13, 14, 15, 16, 17, 18, 19,
        // State 2
        2, 0, 3, 22, 13, 14, 15, 16, 17, 18, 19,
        // State 3
        2, 24, 3, 0, 13, 14, 15, 16, 17, 18, 19,
        // State 4
        2, 0, 3, 25, 13, 14, 15, 16, 17, 18, 19,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 7
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 8
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 9
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 10
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 11
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 12
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 13
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 14
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 15
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 16
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 17
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 18
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 19
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 20
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 21
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 22
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 23
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 24
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 11 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -27,
        // State 6
        -21,
        // State 7
        -13,
        // State 8
        -15,
        // State 9
        -20,
        // State 10
        -12,
        // State 11
        -14,
        // State 12
        -16,
        // State 13
        -18,
        // State 14
        -19,
        // State 15
        -22,
        // State 16
        -23,
        // State 17
        -24,
        // State 18
        -17,
        // State 19
        0,
        // State 20
        -8,
        // State 21
        -10,
        // State 22
        0,
        // State 23
        -9,
        // State 24
        -11,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => match state {
                2 => 4,
                _ => 3,
            },
            5 => match state {
                1..=2 => 19,
                3..=4 => 22,
                _ => 5,
            },
            6 => 6,
            7 => 7,
            8 => 8,
            9 => 9,
            10 => 10,
            11 => 11,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""[""###,
            r###""]""###,
            r###""comment""###,
            r###""false""###,
            r###""keyword""###,
            r###""number""###,
            r###""simplexpr""###,
            r###""symbol""###,
            r###""true""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        file_id: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = parse_error::ParseError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Ast;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 11 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.file_id,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Token::LPren if true => Some(0),
            Token::RPren if true => Some(1),
            Token::LBrack if true => Some(2),
            Token::RBrack if true => Some(3),
            Token::Comment if true => Some(4),
            Token::False if true => Some(5),
            Token::Keyword(_) if true => Some(6),
            Token::NumLit(_) if true => Some(7),
            Token::SimplExpr(_) if true => Some(8),
            Token::Symbol(_) if true => Some(9),
            Token::True if true => Some(10),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 10 => __Symbol::Variant0(__token),
            6 | 7 | 9 => match __token {
                Token::Keyword(__tok0) | Token::NumLit(__tok0) | Token::Symbol(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            8 => match __token {
                Token::SimplExpr(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct AstParser {
        _priv: (),
    }

    impl AstParser {
        pub fn new() -> AstParser {
            AstParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            file_id: usize,
            __tokens0: __TOKENS,
        ) -> Result<Ast, __lalrpop_util::ParseError<usize, Token, parse_error::ParseError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    file_id,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        file_id: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Ast,__lalrpop_util::ParseError<usize, Token, parse_error::ParseError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // SimplExpr = "simplexpr" => ActionFn(41);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action41::<>(file_id, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            23 => {
                __reduce23(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                // __Ast = Ast => ActionFn(1);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(file_id, __sym0);
                return Some(Ok(__nt));
            }
            27 => {
                __reduce27(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Span, Vec<Ast>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Ast, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(usize, simplexpr::parser::lexer::Token, usize)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Ast>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>) = Ast => ActionFn(21);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)* =  => ActionFn(19);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action19::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)* = (<Ast>)+ => ActionFn(20);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)+ = Ast => ActionFn(25);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)+ = (<Ast>)+, Ast => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(file_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(18);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action18::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "(", ")" => ActionFn(45);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action45::<>(file_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce8<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "(", (<Ast>)+, ")" => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(file_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "[", "]" => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(file_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce10<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "[", (<Ast>)+, "]" => ActionFn(48);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(file_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce11<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = SimplExpr => ActionFn(49);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = Keyword => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce13<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = Symbol => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce14<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = Literal => ActionFn(50);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce15<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "comment" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce16<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce17<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce18<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Keyword = "keyword" => ActionFn(52);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Num => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce20<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Bool => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce21<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Num = "number" => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce23<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Symbol = "symbol" => ActionFn(53);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Toplevel =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce25<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Toplevel = (<Ast>)+ => ActionFn(55);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce27<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Toplevel = Toplevel => ActionFn(0);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
}
pub use self::__parse__Ast::AstParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Toplevel {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::parser::{lexer::Token, ast::Ast, parse_error};
    use eww_shared_util::Span;
    use simplexpr::ast::SimplExpr;
    use simplexpr;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(Vec<(usize, simplexpr::parser::lexer::Token, usize)>),
        Variant3(Ast),
        Variant4(alloc::vec::Vec<Ast>),
        Variant5(usize),
        Variant6(SimplExpr),
        Variant7((Span, Vec<Ast>)),
    }
    const __ACTION: &[i8] = &[
        // State 0
        3, 0, 4, 0, 15, 16, 17, 18, 19, 20, 21,
        // State 1
        3, 0, 4, 0, 15, 16, 17, 18, 19, 20, 21,
        // State 2
        3, 23, 4, 0, 15, 16, 17, 18, 19, 20, 21,
        // State 3
        3, 0, 4, 24, 15, 16, 17, 18, 19, 20, 21,
        // State 4
        3, 25, 4, 0, 15, 16, 17, 18, 19, 20, 21,
        // State 5
        3, 0, 4, 26, 15, 16, 17, 18, 19, 20, 21,
        // State 6
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 7
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 8
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 9
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 10
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 11
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 12
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 15
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 16
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 17
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 18
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 19
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 20
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 21
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 22
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 23
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 24
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 25
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 11 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -25,
        // State 1
        -26,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -4,
        // State 7
        -21,
        // State 8
        -13,
        // State 9
        -15,
        // State 10
        -20,
        // State 11
        -12,
        // State 12
        -14,
        // State 13
        -28,
        // State 14
        -16,
        // State 15
        -18,
        // State 16
        -19,
        // State 17
        -22,
        // State 18
        -23,
        // State 19
        -24,
        // State 20
        -17,
        // State 21
        -5,
        // State 22
        -8,
        // State 23
        -10,
        // State 24
        -9,
        // State 25
        -11,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => match state {
                2 => 4,
                3 => 5,
                _ => 1,
            },
            5 => match state {
                1 | 4..=5 => 21,
                _ => 6,
            },
            6 => 7,
            7 => 8,
            8 => 9,
            9 => 10,
            10 => 11,
            11 => 12,
            12 => 13,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""[""###,
            r###""]""###,
            r###""comment""###,
            r###""false""###,
            r###""keyword""###,
            r###""number""###,
            r###""simplexpr""###,
            r###""symbol""###,
            r###""true""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        file_id: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = parse_error::ParseError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = (Span, Vec<Ast>);
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 11 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.file_id,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Token::LPren if true => Some(0),
            Token::RPren if true => Some(1),
            Token::LBrack if true => Some(2),
            Token::RBrack if true => Some(3),
            Token::Comment if true => Some(4),
            Token::False if true => Some(5),
            Token::Keyword(_) if true => Some(6),
            Token::NumLit(_) if true => Some(7),
            Token::SimplExpr(_) if true => Some(8),
            Token::Symbol(_) if true => Some(9),
            Token::True if true => Some(10),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 10 => __Symbol::Variant0(__token),
            6 | 7 | 9 => match __token {
                Token::Keyword(__tok0) | Token::NumLit(__tok0) | Token::Symbol(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            8 => match __token {
                Token::SimplExpr(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ToplevelParser {
        _priv: (),
    }

    impl ToplevelParser {
        pub fn new() -> ToplevelParser {
            ToplevelParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            file_id: usize,
            __tokens0: __TOKENS,
        ) -> Result<(Span, Vec<Ast>), __lalrpop_util::ParseError<usize, Token, parse_error::ParseError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    file_id,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        file_id: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<(Span, Vec<Ast>),__lalrpop_util::ParseError<usize, Token, parse_error::ParseError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // SimplExpr = "simplexpr" => ActionFn(41);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action41::<>(file_id, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            23 => {
                __reduce23(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(file_id, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                // __Toplevel = Toplevel => ActionFn(0);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(file_id, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Span, Vec<Ast>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Ast, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(usize, simplexpr::parser::lexer::Token, usize)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Ast>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>) = Ast => ActionFn(21);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)* =  => ActionFn(19);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action19::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)* = (<Ast>)+ => ActionFn(20);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)+ = Ast => ActionFn(25);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Ast>)+ = (<Ast>)+, Ast => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(file_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(18);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action18::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "(", ")" => ActionFn(45);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action45::<>(file_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce8<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "(", (<Ast>)+, ")" => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(file_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "[", "]" => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(file_id, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce10<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "[", (<Ast>)+, "]" => ActionFn(48);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(file_id, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce11<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = SimplExpr => ActionFn(49);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = Keyword => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce13<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = Symbol => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce14<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = Literal => ActionFn(50);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce15<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Ast = "comment" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce16<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce17<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce18<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Keyword = "keyword" => ActionFn(52);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Num => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce20<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Bool => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce21<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Num = "number" => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce23<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Symbol = "symbol" => ActionFn(53);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Toplevel =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(file_id, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce25<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Toplevel = (<Ast>)+ => ActionFn(55);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        file_id: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Ast = Ast => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(file_id, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
}
pub use self::__parse__Toplevel::ToplevelParser;

#[allow(unused_variables)]
fn __action0<
>(
    file_id: usize,
    (_, __0, _): (usize, (Span, Vec<Ast>), usize),
) -> (Span, Vec<Ast>)
{
    __0
}

#[allow(unused_variables)]
fn __action1<
>(
    file_id: usize,
    (_, __0, _): (usize, Ast, usize),
) -> Ast
{
    __0
}

#[allow(unused_variables)]
fn __action2<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, elems, _): (usize, alloc::vec::Vec<Ast>, usize),
    (_, r, _): (usize, usize, usize),
) -> (Span, Vec<Ast>)
{
    (Span(l, r, file_id), elems)
}

#[allow(unused_variables)]
fn __action3<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, elems, _): (usize, alloc::vec::Vec<Ast>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::List(Span(l, r, file_id), elems)
}

#[allow(unused_variables)]
fn __action4<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, elems, _): (usize, alloc::vec::Vec<Ast>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::Array(Span(l, r, file_id), elems)
}

#[allow(unused_variables)]
fn __action5<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, expr, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::SimplExpr(Span(l, r, file_id), expr)
}

#[allow(unused_variables)]
fn __action6<
>(
    file_id: usize,
    (_, x, _): (usize, Ast, usize),
) -> Ast
{
    x
}

#[allow(unused_variables)]
fn __action7<
>(
    file_id: usize,
    (_, x, _): (usize, Ast, usize),
) -> Ast
{
    x
}

#[allow(unused_variables)]
fn __action8<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::SimplExpr(Span(l, r, file_id), SimplExpr::literal(Span(l, r, file_id), x.into()))
}

#[allow(unused_variables)]
fn __action9<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::Comment(Span(l, r, file_id))
}

#[allow(unused_variables)]
fn __action10<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::Keyword(Span(l, r, file_id), x[1..].to_string())
}

#[allow(unused_variables)]
fn __action11<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> Ast
{
    Ast::Symbol(Span(l, r, file_id), x.to_string())
}

#[allow(unused_variables)]
fn __action12<
>(
    file_id: usize,
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action13<
>(
    file_id: usize,
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action14<
>(
    file_id: usize,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, Vec<(usize, simplexpr::parser::lexer::Token, usize)>, usize),
) -> Result<SimplExpr,__lalrpop_util::ParseError<usize,Token,parse_error::ParseError>>
{
    {
        let parser = simplexpr::simplexpr_parser::ExprParser::new();
        parser.parse(file_id, x.into_iter().map(Ok))
            .map_err(|e| ParseError::User {
                error: parse_error::ParseError::SimplExpr(simplexpr::error::Error::from_parse_error(file_id, e))
            })
   }
}

#[allow(unused_variables)]
fn __action15<
>(
    file_id: usize,
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
fn __action16<
>(
    file_id: usize,
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "true".to_string()
}

#[allow(unused_variables)]
fn __action17<
>(
    file_id: usize,
    (_, __0, _): (usize, Token, usize),
) -> String
{
    "false".to_string()
}

#[allow(unused_variables)]
fn __action18<
>(
    file_id: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action19<
>(
    file_id: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Ast>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action20<
>(
    file_id: usize,
    (_, v, _): (usize, alloc::vec::Vec<Ast>, usize),
) -> alloc::vec::Vec<Ast>
{
    v
}

#[allow(unused_variables)]
fn __action21<
>(
    file_id: usize,
    (_, __0, _): (usize, Ast, usize),
) -> Ast
{
    __0
}

#[allow(unused_variables)]
fn __action22<
>(
    file_id: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action23<
>(
    file_id: usize,
    (_, __0, _): (usize, Ast, usize),
) -> alloc::vec::Vec<Ast>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action24<
>(
    file_id: usize,
    (_, v, _): (usize, alloc::vec::Vec<Ast>, usize),
    (_, e, _): (usize, Ast, usize),
) -> alloc::vec::Vec<Ast>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action25<
>(
    file_id: usize,
    __0: (usize, Ast, usize),
) -> alloc::vec::Vec<Ast>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action21(
        file_id,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        file_id,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
>(
    file_id: usize,
    __0: (usize, alloc::vec::Vec<Ast>, usize),
    __1: (usize, Ast, usize),
) -> alloc::vec::Vec<Ast>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        file_id,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        file_id,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
>(
    file_id: usize,
    __0: (usize, usize, usize),
    __1: (usize, Token, usize),
    __2: (usize, Token, usize),
    __3: (usize, usize, usize),
) -> Ast
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action19(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        file_id,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action28<
>(
    file_id: usize,
    __0: (usize, usize, usize),
    __1: (usize, Token, usize),
    __2: (usize, alloc::vec::Vec<Ast>, usize),
    __3: (usize, Token, usize),
    __4: (usize, usize, usize),
) -> Ast
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action20(
        file_id,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        file_id,
        __0,
        __1,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action29<
>(
    file_id: usize,
    __0: (usize, usize, usize),
    __1: (usize, Token, usize),
    __2: (usize, Token, usize),
    __3: (usize, usize, usize),
) -> Ast
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action19(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        file_id,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action30<
>(
    file_id: usize,
    __0: (usize, usize, usize),
    __1: (usize, Token, usize),
    __2: (usize, alloc::vec::Vec<Ast>, usize),
    __3: (usize, Token, usize),
    __4: (usize, usize, usize),
) -> Ast
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action20(
        file_id,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        file_id,
        __0,
        __1,
        __temp0,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action31<
>(
    file_id: usize,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
) -> (Span, Vec<Ast>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action19(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        file_id,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action32<
>(
    file_id: usize,
    __0: (usize, usize, usize),
    __1: (usize, alloc::vec::Vec<Ast>, usize),
    __2: (usize, usize, usize),
) -> (Span, Vec<Ast>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action20(
        file_id,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        file_id,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action33<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
    __2: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        file_id,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action34<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<Ast>, usize),
    __2: (usize, Token, usize),
    __3: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        file_id,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action35<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
    __2: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        file_id,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action36<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<Ast>, usize),
    __2: (usize, Token, usize),
    __3: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        file_id,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action37<
>(
    file_id: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        file_id,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action38<
>(
    file_id: usize,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        file_id,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action39<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        file_id,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action40<
>(
    file_id: usize,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        file_id,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action41<
>(
    file_id: usize,
    __0: (usize, Vec<(usize, simplexpr::parser::lexer::Token, usize)>, usize),
) -> Result<SimplExpr,__lalrpop_util::ParseError<usize,Token,parse_error::ParseError>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        file_id,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action42<
>(
    file_id: usize,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> Ast
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        file_id,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action43<
>(
    file_id: usize,
    __0: (usize, usize, usize),
) -> (Span, Vec<Ast>)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        file_id,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action44<
>(
    file_id: usize,
    __0: (usize, alloc::vec::Vec<Ast>, usize),
    __1: (usize, usize, usize),
) -> (Span, Vec<Ast>)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action22(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        file_id,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action45<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> Ast
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        file_id,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action46<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<Ast>, usize),
    __2: (usize, Token, usize),
) -> Ast
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        file_id,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action47<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> Ast
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        file_id,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action48<
>(
    file_id: usize,
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<Ast>, usize),
    __2: (usize, Token, usize),
) -> Ast
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        file_id,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action49<
>(
    file_id: usize,
    __0: (usize, SimplExpr, usize),
) -> Ast
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        file_id,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action50<
>(
    file_id: usize,
    __0: (usize, String, usize),
) -> Ast
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        file_id,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action51<
>(
    file_id: usize,
    __0: (usize, Token, usize),
) -> Ast
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        file_id,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action52<
>(
    file_id: usize,
    __0: (usize, String, usize),
) -> Ast
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        file_id,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action53<
>(
    file_id: usize,
    __0: (usize, String, usize),
) -> Ast
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        file_id,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action54<
>(
    file_id: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> (Span, Vec<Ast>)
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        file_id,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action55<
>(
    file_id: usize,
    __0: (usize, alloc::vec::Vec<Ast>, usize),
) -> (Span, Vec<Ast>)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        file_id,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        file_id,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, parse_error::ParseError>>;
}

impl<> __ToTriple<> for (usize, Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, parse_error::ParseError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), parse_error::ParseError> {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, parse_error::ParseError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
