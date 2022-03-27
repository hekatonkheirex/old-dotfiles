// auto-generated: "lalrpop 0.19.6"
// sha3: b1eaf11bbe626ffa92ce373530a21157d280c2105a19ba248492d4a71775284d
use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
use eww_shared_util::{Span, VarName};
use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
use crate::parser::lalrpop_helpers::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 1
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 2
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 3
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 4
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -14, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 5
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, -18,
        // State 6
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 7
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 8
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 9
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 10
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 11
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 12
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 13
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 14
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 15
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 16
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 17
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 18
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 19
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 20
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 21
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 22
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 23
        2, 0, 0, 0, 3, -16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -16, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 24
        2, 0, 0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 25
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, -20,
        // State 26
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 27
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 38, 39, 40, 41, 6, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 30
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 42, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 7, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 31
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 32
        0, -44, 8, -44, 0, -44, 9, -44, -44, -44, 0, 10, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 33
        0, -52, 0, -52, 0, -52, 0, 11, -52, 12, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 34
        0, 13, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 14, 15, 16, 17, 18, 19, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 35
        0, 0, 0, 20, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 21, 22, 0, -22, 0, 0, 0, 0, 0, 0, 23, -22,
        // State 36
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 37
        0, -27, -27, -27, 25, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 38
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 39
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 40
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 43
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -13, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 51
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 54
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 55
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 56
        0, -42, 8, -42, 0, -42, 9, -42, -42, -42, 0, 10, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 57
        0, -43, 8, -43, 0, -43, 9, -43, -43, -43, 0, 10, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 58
        0, -46, 0, -46, 0, -46, 0, 11, -46, 12, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 59
        0, -50, 0, -50, 0, -50, 0, 11, -50, 12, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 60
        0, -48, 0, -48, 0, -48, 0, 11, -48, 12, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 61
        0, -45, 0, -45, 0, -45, 0, 11, -45, 12, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 62
        0, -51, 0, -51, 0, -51, 0, 11, -51, 12, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 63
        0, -49, 0, -49, 0, -49, 0, 11, -49, 12, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 64
        0, -47, 0, -47, 0, -47, 0, 11, -47, 12, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 65
        0, 13, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 14, 15, 16, 17, 18, 19, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 13, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 14, 15, 16, 17, 18, 19, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 68
        0, 13, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 14, 15, 16, 17, 18, 19, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 69
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 70
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 72
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 75
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 76
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 77
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -63,
        // State 29
        -34,
        // State 30
        -37,
        // State 31
        -41,
        // State 32
        -44,
        // State 33
        -52,
        // State 34
        -56,
        // State 35
        -22,
        // State 36
        -26,
        // State 37
        -27,
        // State 38
        -24,
        // State 39
        -23,
        // State 40
        -25,
        // State 41
        0,
        // State 42
        -35,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -33,
        // State 52
        0,
        // State 53
        -40,
        // State 54
        -38,
        // State 55
        -39,
        // State 56
        -42,
        // State 57
        -43,
        // State 58
        -46,
        // State 59
        -50,
        // State 60
        -48,
        // State 61
        -45,
        // State 62
        -51,
        // State 63
        -49,
        // State 64
        -47,
        // State 65
        -53,
        // State 66
        0,
        // State 67
        -55,
        // State 68
        -54,
        // State 69
        -28,
        // State 70
        0,
        // State 71
        -29,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        -30,
        // State 76
        0,
        // State 77
        -32,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        -21,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 23,
            5 => 25,
            8 => match state {
                24 => 73,
                _ => 46,
            },
            9 => 48,
            10 => match state {
                0 => 28,
                5 | 25 => 49,
                26 => 81,
                27 => 82,
                _ => 43,
            },
            11 => 29,
            12 => 30,
            13 => match state {
                1 => 42,
                3 => 45,
                7 => 53,
                8 => 54,
                9 => 55,
                _ => 31,
            },
            14 => match state {
                10 => 56,
                11 => 57,
                _ => 32,
            },
            15 => match state {
                12 => 58,
                13 => 59,
                14 => 60,
                15 => 61,
                16 => 62,
                17 => 63,
                18 => 64,
                _ => 33,
            },
            16 => match state {
                19 => 65,
                21 => 67,
                22 => 68,
                _ => 34,
            },
            17 => 35,
            18 => match state {
                2 => 44,
                6 => 52,
                20 => 66,
                23 => 70,
                _ => 47,
            },
            20 => match state {
                25 => 74,
                _ => 50,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                // __Expr = Expr => ActionFn(7);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr0 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 31, 32, 33, 34, 4, 0, 0,
        // State 1
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 2
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, -14, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 3
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, -18,
        // State 4
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 5
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 6
        5, 0, 0, 0, 2, -16, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, -16, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 7
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, -20,
        // State 8
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 9
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 10
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 11
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 12
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 13
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 14
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 15
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 16
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 17
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 18
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 19
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 20
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 21
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 22
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 23
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 24
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 25
        5, 0, 0, 0, 2, -14, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 26
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 27
        5, 0, 0, 0, 2, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 30, 44, 32, 33, 34, 4, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 32
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 33
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 34
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 36
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 50, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 9, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 37
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 38
        0, -44, 10, -44, 0, -44, 11, -44, -44, -44, 0, 12, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 39
        0, -52, 0, -52, 0, -52, 0, 13, -52, 14, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 40
        0, 15, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 16, 17, 18, 19, 20, 21, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 41
        0, 0, 0, 22, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 23, 24, 0, -22, 0, 0, 0, 0, 0, 0, 25, -22,
        // State 42
        0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -27, -27, -27, 26, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, -13, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0,
        // State 50
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 51
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 52
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 53
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 55
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 57
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 58
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 59
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 62
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 63
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 64
        0, -42, 10, -42, 0, -42, 11, -42, -42, -42, 0, 12, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 65
        0, -43, 10, -43, 0, -43, 11, -43, -43, -43, 0, 12, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 66
        0, -46, 0, -46, 0, -46, 0, 13, -46, 14, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 67
        0, -50, 0, -50, 0, -50, 0, 13, -50, 14, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 68
        0, -48, 0, -48, 0, -48, 0, 13, -48, 14, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 69
        0, -45, 0, -45, 0, -45, 0, 13, -45, 14, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 70
        0, -51, 0, -51, 0, -51, 0, 13, -51, 14, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 71
        0, -49, 0, -49, 0, -49, 0, 13, -49, 14, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 72
        0, -47, 0, -47, 0, -47, 0, 13, -47, 14, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 73
        0, 15, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 16, 17, 18, 19, 20, 21, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 15, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 16, 17, 18, 19, 20, 21, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 76
        0, 15, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 16, 17, 18, 19, 20, 21, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 77
        0, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 81
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 82
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 83
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -64,
        // State 29
        -26,
        // State 30
        -27,
        // State 31
        -24,
        // State 32
        -23,
        // State 33
        -25,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -28,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        -29,
        // State 55
        0,
        // State 56
        0,
        // State 57
        -30,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 6,
            5 => 7,
            8 => match state {
                25 => 77,
                _ => 44,
            },
            9 => 46,
            10 => match state {
                3 | 7 => 47,
                26 => 80,
                27 => 83,
                _ => 34,
            },
            11 => match state {
                1..=27 => 35,
                _ => 28,
            },
            12 => 36,
            13 => match state {
                4 => 51,
                5 => 52,
                9 => 61,
                10 => 62,
                11 => 63,
                _ => 37,
            },
            14 => match state {
                12 => 64,
                13 => 65,
                _ => 38,
            },
            15 => match state {
                14 => 66,
                15 => 67,
                16 => 68,
                17 => 69,
                18 => 70,
                19 => 71,
                20 => 72,
                _ => 39,
            },
            16 => match state {
                21 => 73,
                23 => 75,
                24 => 76,
                _ => 40,
            },
            17 => 41,
            18 => match state {
                1 => 42,
                6 => 53,
                8 => 60,
                22 => 74,
                _ => 45,
            },
            20 => match state {
                7 => 56,
                _ => 48,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr0Parser {
        _priv: (),
    }

    impl Expr0Parser {
        pub fn new() -> Expr0Parser {
            Expr0Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                // __Expr0 = Expr0 => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr0::Expr0Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr1 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 1
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 2
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, -14, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 3
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, -18,
        // State 4
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 5
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 6
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 7
        6, 0, 0, 0, 2, -16, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, -16, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 8
        6, 0, 0, 0, 2, -14, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 9
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, -20,
        // State 10
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 11
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 12
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 13
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 14
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 15
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 16
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 17
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 18
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 19
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 20
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 21
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 22
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 23
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 24
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 25
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 26
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 27
        6, 0, 0, 0, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 31, 32, 33, 34, 35, 4, 0, 0,
        // State 28
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 31
        0, -27, -27, -27, 9, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 32
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 33
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 34
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 36, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 5, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 38
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 39
        0, -44, 11, -44, 0, -44, 12, -44, -44, -44, 0, 13, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 40
        0, -52, 0, -52, 0, -52, 0, 14, -52, 15, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 41
        0, 16, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 17, 18, 19, 20, 21, 22, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 42
        0, 0, 0, 23, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 24, 25, 0, -22, 0, 0, 0, 0, 0, 0, 26, -22,
        // State 43
        0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, -13, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 49
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 52
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 53
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 54
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 56
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 59
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 60
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 61
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 62
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 63
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 64
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 65
        0, -42, 11, -42, 0, -42, 12, -42, -42, -42, 0, 13, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 66
        0, -43, 11, -43, 0, -43, 12, -43, -43, -43, 0, 13, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 67
        0, -46, 0, -46, 0, -46, 0, 14, -46, 15, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 68
        0, -50, 0, -50, 0, -50, 0, 14, -50, 15, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 69
        0, -48, 0, -48, 0, -48, 0, 14, -48, 15, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 70
        0, -45, 0, -45, 0, -45, 0, 14, -45, 15, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 71
        0, -51, 0, -51, 0, -51, 0, 14, -51, 15, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 72
        0, -49, 0, -49, 0, -49, 0, 14, -49, 15, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 73
        0, -47, 0, -47, 0, -47, 0, 14, -47, 15, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 74
        0, 16, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 17, 18, 19, 20, 21, 22, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 16, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 17, 18, 19, 20, 21, 22, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 77
        0, 16, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 17, 18, 19, 20, 21, 22, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -34,
        // State 29
        -65,
        // State 30
        -26,
        // State 31
        -27,
        // State 32
        -24,
        // State 33
        -23,
        // State 34
        -25,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        -33,
        // State 50
        0,
        // State 51
        -28,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        -29,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        -30,
        // State 60
        0,
        // State 61
        -32,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 7,
            5 => 9,
            8 => match state {
                8 => 57,
                _ => 44,
            },
            9 => 46,
            10 => match state {
                3 | 9 => 47,
                26 => 81,
                27 => 82,
                _ => 36,
            },
            11 => 28,
            12 => match state {
                1..=27 => 37,
                _ => 29,
            },
            13 => match state {
                5 => 52,
                6 => 53,
                10 => 62,
                11 => 63,
                12 => 64,
                _ => 38,
            },
            14 => match state {
                13 => 65,
                14 => 66,
                _ => 39,
            },
            15 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                18 => 70,
                19 => 71,
                20 => 72,
                21 => 73,
                _ => 40,
            },
            16 => match state {
                22 => 74,
                24 => 76,
                25 => 77,
                _ => 41,
            },
            17 => 42,
            18 => match state {
                1 => 43,
                4 => 50,
                7 => 54,
                23 => 75,
                _ => 45,
            },
            20 => match state {
                9 => 58,
                _ => 48,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr1Parser {
        _priv: (),
    }

    impl Expr1Parser {
        pub fn new() -> Expr1Parser {
            Expr1Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                // __Expr1 = Expr1 => ActionFn(1);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr1::Expr1Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr2 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 1
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 2
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 3
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 4
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -14, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 5
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, -18,
        // State 6
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 7
        2, 0, 0, 0, 3, -16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -16, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 8
        2, 0, 0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 9
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, -20,
        // State 10
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 11
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 12
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 13
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 14
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 15
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 16
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 17
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 18
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 19
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 20
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 21
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 22
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 23
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 24
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 25
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 26
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 27
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 32, 33, 34, 35, 36, 6, 0, 0,
        // State 28
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 29
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 7, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 32
        0, -27, -27, -27, 9, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 33
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 34
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 35
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 38
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 40
        0, -44, 11, -44, 0, -44, 12, -44, -44, -44, 0, 13, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 41
        0, -52, 0, -52, 0, -52, 0, 14, -52, 15, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 42
        0, 16, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 17, 18, 19, 20, 21, 22, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 43
        0, 0, 0, 23, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 24, 25, 0, -22, 0, 0, 0, 0, 0, 0, 26, -22,
        // State 44
        0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -13, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 51
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 54
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 56
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 59
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 60
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 61
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 62
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 63
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 64
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 65
        0, -42, 11, -42, 0, -42, 12, -42, -42, -42, 0, 13, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 66
        0, -43, 11, -43, 0, -43, 12, -43, -43, -43, 0, 13, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 67
        0, -46, 0, -46, 0, -46, 0, 14, -46, 15, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 68
        0, -50, 0, -50, 0, -50, 0, 14, -50, 15, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 69
        0, -48, 0, -48, 0, -48, 0, 14, -48, 15, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 70
        0, -45, 0, -45, 0, -45, 0, 14, -45, 15, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 71
        0, -51, 0, -51, 0, -51, 0, 14, -51, 15, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 72
        0, -49, 0, -49, 0, -49, 0, 14, -49, 15, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 73
        0, -47, 0, -47, 0, -47, 0, 14, -47, 15, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 74
        0, 16, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 17, 18, 19, 20, 21, 22, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 16, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 17, 18, 19, 20, 21, 22, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 77
        0, 16, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 17, 18, 19, 20, 21, 22, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -34,
        // State 29
        -37,
        // State 30
        -66,
        // State 31
        -26,
        // State 32
        -27,
        // State 33
        -24,
        // State 34
        -23,
        // State 35
        -25,
        // State 36
        0,
        // State 37
        -35,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -33,
        // State 52
        0,
        // State 53
        -28,
        // State 54
        0,
        // State 55
        -29,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        -30,
        // State 60
        0,
        // State 61
        -32,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 7,
            5 => 9,
            8 => match state {
                8 => 57,
                _ => 46,
            },
            9 => 48,
            10 => match state {
                5 | 9 => 49,
                26 => 81,
                27 => 82,
                _ => 38,
            },
            11 => 28,
            12 => 29,
            13 => match state {
                0 => 30,
                1 => 37,
                3 => 45,
                10 => 62,
                11 => 63,
                12 => 64,
                _ => 39,
            },
            14 => match state {
                13 => 65,
                14 => 66,
                _ => 40,
            },
            15 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                18 => 70,
                19 => 71,
                20 => 72,
                21 => 73,
                _ => 41,
            },
            16 => match state {
                22 => 74,
                24 => 76,
                25 => 77,
                _ => 42,
            },
            17 => 43,
            18 => match state {
                2 => 44,
                6 => 52,
                7 => 54,
                23 => 75,
                _ => 47,
            },
            20 => match state {
                9 => 58,
                _ => 50,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr2Parser {
        _priv: (),
    }

    impl Expr2Parser {
        pub fn new() -> Expr2Parser {
            Expr2Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                // __Expr2 = Expr2 => ActionFn(2);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr2::Expr2Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr3 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 1
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 2
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 3
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 4
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -14, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 5
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, -18,
        // State 6
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 7
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 8
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 9
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 10
        2, 0, 0, 0, 3, -16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -16, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 11
        2, 0, 0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 12
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, -20,
        // State 13
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 14
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 15
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 16
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 17
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 18
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 19
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 20
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 21
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 22
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 23
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 24
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 25
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 26
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 27
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 33, 34, 35, 36, 37, 6, 0, 0,
        // State 28
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 29
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 38, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 7, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 30
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 31
        0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 33
        0, -27, -27, -27, 12, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 34
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 35
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 36
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 39
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -44, 8, -44, 0, -44, 9, -44, -44, -44, 0, 10, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 41
        0, -52, 0, -52, 0, -52, 0, 14, -52, 15, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 42
        0, 16, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 17, 18, 19, 20, 21, 22, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 43
        0, 0, 0, 23, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 24, 25, 0, -22, 0, 0, 0, 0, 0, 0, 26, -22,
        // State 44
        0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -13, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 51
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 54
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 55
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 56
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 57
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 59
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 62
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 63
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 64
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 65
        0, -42, 8, -42, 0, -42, 9, -42, -42, -42, 0, 10, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 66
        0, -43, 8, -43, 0, -43, 9, -43, -43, -43, 0, 10, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 67
        0, -46, 0, -46, 0, -46, 0, 14, -46, 15, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 68
        0, -50, 0, -50, 0, -50, 0, 14, -50, 15, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 69
        0, -48, 0, -48, 0, -48, 0, 14, -48, 15, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 70
        0, -45, 0, -45, 0, -45, 0, 14, -45, 15, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 71
        0, -51, 0, -51, 0, -51, 0, 14, -51, 15, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 72
        0, -49, 0, -49, 0, -49, 0, 14, -49, 15, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 73
        0, -47, 0, -47, 0, -47, 0, 14, -47, 15, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 74
        0, 16, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 17, 18, 19, 20, 21, 22, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 16, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 17, 18, 19, 20, 21, 22, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 77
        0, 16, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 17, 18, 19, 20, 21, 22, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -34,
        // State 29
        -37,
        // State 30
        -41,
        // State 31
        -67,
        // State 32
        -26,
        // State 33
        -27,
        // State 34
        -24,
        // State 35
        -23,
        // State 36
        -25,
        // State 37
        0,
        // State 38
        -35,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -33,
        // State 52
        0,
        // State 53
        -40,
        // State 54
        -38,
        // State 55
        -39,
        // State 56
        -28,
        // State 57
        0,
        // State 58
        -29,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        -30,
        // State 63
        0,
        // State 64
        -32,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 10,
            5 => 12,
            8 => match state {
                11 => 60,
                _ => 46,
            },
            9 => 48,
            10 => match state {
                5 | 12 => 49,
                26 => 81,
                27 => 82,
                _ => 39,
            },
            11 => 28,
            12 => 29,
            13 => match state {
                1 => 38,
                3 => 45,
                7 => 53,
                8 => 54,
                9 => 55,
                _ => 30,
            },
            14 => match state {
                0 => 31,
                13 => 65,
                14 => 66,
                _ => 40,
            },
            15 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                18 => 70,
                19 => 71,
                20 => 72,
                21 => 73,
                _ => 41,
            },
            16 => match state {
                22 => 74,
                24 => 76,
                25 => 77,
                _ => 42,
            },
            17 => 43,
            18 => match state {
                2 => 44,
                6 => 52,
                10 => 57,
                23 => 75,
                _ => 47,
            },
            20 => match state {
                12 => 61,
                _ => 50,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr3Parser {
        _priv: (),
    }

    impl Expr3Parser {
        pub fn new() -> Expr3Parser {
            Expr3Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                // __Expr3 = Expr3 => ActionFn(3);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr3::Expr3Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr4 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 1
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 2
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 3
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 4
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -14, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 5
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, -18,
        // State 6
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 7
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 8
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 9
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 10
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 11
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 12
        2, 0, 0, 0, 3, -16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -16, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 13
        2, 0, 0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 14
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, -20,
        // State 15
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 16
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 17
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 18
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 19
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 20
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 21
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 22
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 23
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 24
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 25
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 26
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 27
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 34, 35, 36, 37, 38, 6, 0, 0,
        // State 28
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 29
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 39, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 7, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 30
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 31
        0, -44, 8, -44, 0, -44, 9, -44, -44, -44, 0, 10, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 34
        0, -27, -27, -27, 14, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 35
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 36
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 37
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 40
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -52, 0, -52, 0, -52, 0, 11, -52, 12, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 42
        0, 16, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 17, 18, 19, 20, 21, 22, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 43
        0, 0, 0, 23, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 24, 25, 0, -22, 0, 0, 0, 0, 0, 0, 26, -22,
        // State 44
        0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -13, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 51
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 54
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 55
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 56
        0, -42, 8, -42, 0, -42, 9, -42, -42, -42, 0, 10, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 57
        0, -43, 8, -43, 0, -43, 9, -43, -43, -43, 0, 10, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 58
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 59
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 61
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 64
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 65
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 66
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 67
        0, -46, 0, -46, 0, -46, 0, 11, -46, 12, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 68
        0, -50, 0, -50, 0, -50, 0, 11, -50, 12, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 69
        0, -48, 0, -48, 0, -48, 0, 11, -48, 12, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 70
        0, -45, 0, -45, 0, -45, 0, 11, -45, 12, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 71
        0, -51, 0, -51, 0, -51, 0, 11, -51, 12, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 72
        0, -49, 0, -49, 0, -49, 0, 11, -49, 12, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 73
        0, -47, 0, -47, 0, -47, 0, 11, -47, 12, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 74
        0, 16, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 17, 18, 19, 20, 21, 22, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 16, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 17, 18, 19, 20, 21, 22, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 77
        0, 16, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 17, 18, 19, 20, 21, 22, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -34,
        // State 29
        -37,
        // State 30
        -41,
        // State 31
        -44,
        // State 32
        -68,
        // State 33
        -26,
        // State 34
        -27,
        // State 35
        -24,
        // State 36
        -23,
        // State 37
        -25,
        // State 38
        0,
        // State 39
        -35,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -33,
        // State 52
        0,
        // State 53
        -40,
        // State 54
        -38,
        // State 55
        -39,
        // State 56
        -42,
        // State 57
        -43,
        // State 58
        -28,
        // State 59
        0,
        // State 60
        -29,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        -30,
        // State 65
        0,
        // State 66
        -32,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 12,
            5 => 14,
            8 => match state {
                13 => 62,
                _ => 46,
            },
            9 => 48,
            10 => match state {
                5 | 14 => 49,
                26 => 81,
                27 => 82,
                _ => 40,
            },
            11 => 28,
            12 => 29,
            13 => match state {
                1 => 39,
                3 => 45,
                7 => 53,
                8 => 54,
                9 => 55,
                _ => 30,
            },
            14 => match state {
                10 => 56,
                11 => 57,
                _ => 31,
            },
            15 => match state {
                0 => 32,
                15 => 67,
                16 => 68,
                17 => 69,
                18 => 70,
                19 => 71,
                20 => 72,
                21 => 73,
                _ => 41,
            },
            16 => match state {
                22 => 74,
                24 => 76,
                25 => 77,
                _ => 42,
            },
            17 => 43,
            18 => match state {
                2 => 44,
                6 => 52,
                12 => 59,
                23 => 75,
                _ => 47,
            },
            20 => match state {
                14 => 63,
                _ => 50,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr4Parser {
        _priv: (),
    }

    impl Expr4Parser {
        pub fn new() -> Expr4Parser {
            Expr4Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                // __Expr4 = Expr4 => ActionFn(4);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr4::Expr4Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr5 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 1
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 2
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 3
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 4
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -14, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 5
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, -18,
        // State 6
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 7
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 8
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 9
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 10
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 11
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 12
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 13
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 14
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 15
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 16
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 17
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 18
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 19
        2, 0, 0, 0, 3, -16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -16, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 20
        2, 0, 0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 21
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, -20,
        // State 22
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 23
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 24
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 25
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 26
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 27
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 35, 36, 37, 38, 39, 6, 0, 0,
        // State 28
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 29
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 40, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 7, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 30
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 31
        0, -44, 8, -44, 0, -44, 9, -44, -44, -44, 0, 10, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 32
        0, -52, 0, -52, 0, -52, 0, 11, -52, 12, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 33
        0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 35
        0, -27, -27, -27, 21, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 36
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 37
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 38
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 41
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 13, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 14, 15, 16, 17, 18, 19, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 43
        0, 0, 0, 23, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 24, 25, 0, -22, 0, 0, 0, 0, 0, 0, 26, -22,
        // State 44
        0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -13, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 51
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 54
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 55
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 56
        0, -42, 8, -42, 0, -42, 9, -42, -42, -42, 0, 10, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 57
        0, -43, 8, -43, 0, -43, 9, -43, -43, -43, 0, 10, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 58
        0, -46, 0, -46, 0, -46, 0, 11, -46, 12, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 59
        0, -50, 0, -50, 0, -50, 0, 11, -50, 12, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 60
        0, -48, 0, -48, 0, -48, 0, 11, -48, 12, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 61
        0, -45, 0, -45, 0, -45, 0, 11, -45, 12, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 62
        0, -51, 0, -51, 0, -51, 0, 11, -51, 12, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 63
        0, -49, 0, -49, 0, -49, 0, 11, -49, 12, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 64
        0, -47, 0, -47, 0, -47, 0, 11, -47, 12, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 65
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 66
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 68
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 71
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 72
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 73
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 74
        0, 13, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 14, 15, 16, 17, 18, 19, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 13, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 14, 15, 16, 17, 18, 19, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 77
        0, 13, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 14, 15, 16, 17, 18, 19, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -34,
        // State 29
        -37,
        // State 30
        -41,
        // State 31
        -44,
        // State 32
        -52,
        // State 33
        -69,
        // State 34
        -26,
        // State 35
        -27,
        // State 36
        -24,
        // State 37
        -23,
        // State 38
        -25,
        // State 39
        0,
        // State 40
        -35,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -33,
        // State 52
        0,
        // State 53
        -40,
        // State 54
        -38,
        // State 55
        -39,
        // State 56
        -42,
        // State 57
        -43,
        // State 58
        -46,
        // State 59
        -50,
        // State 60
        -48,
        // State 61
        -45,
        // State 62
        -51,
        // State 63
        -49,
        // State 64
        -47,
        // State 65
        -28,
        // State 66
        0,
        // State 67
        -29,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        -30,
        // State 72
        0,
        // State 73
        -32,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 19,
            5 => 21,
            8 => match state {
                20 => 69,
                _ => 46,
            },
            9 => 48,
            10 => match state {
                5 | 21 => 49,
                26 => 81,
                27 => 82,
                _ => 41,
            },
            11 => 28,
            12 => 29,
            13 => match state {
                1 => 40,
                3 => 45,
                7 => 53,
                8 => 54,
                9 => 55,
                _ => 30,
            },
            14 => match state {
                10 => 56,
                11 => 57,
                _ => 31,
            },
            15 => match state {
                12 => 58,
                13 => 59,
                14 => 60,
                15 => 61,
                16 => 62,
                17 => 63,
                18 => 64,
                _ => 32,
            },
            16 => match state {
                0 => 33,
                22 => 74,
                24 => 76,
                25 => 77,
                _ => 42,
            },
            17 => 43,
            18 => match state {
                2 => 44,
                6 => 52,
                19 => 66,
                23 => 75,
                _ => 47,
            },
            20 => match state {
                21 => 70,
                _ => 50,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr5Parser {
        _priv: (),
    }

    impl Expr5Parser {
        pub fn new() -> Expr5Parser {
            Expr5Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                // __Expr5 = Expr5 => ActionFn(5);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(fid, __sym0);
                return Some(Ok(__nt));
            }
            69 => {
                __reduce69(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr6 = Expr6 => ActionFn(6);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 29)
    }
}
pub use self::__parse__Expr5::Expr5Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr6 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::{SimplExpr::{self, *}, BinOp::*, UnaryOp::*};
    use eww_shared_util::{Span, VarName};
    use crate::parser::lexer::{Token, LexicalError, StrLitSegment, Sp};
    use crate::parser::lalrpop_helpers::*;
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
        Variant2(Vec<Sp<StrLitSegment>>),
        Variant3(SimplExpr),
        Variant4(alloc::vec::Vec<SimplExpr>),
        Variant5((SimplExpr, SimplExpr)),
        Variant6(alloc::vec::Vec<(SimplExpr, SimplExpr)>),
        Variant7(usize),
        Variant8(Vec<SimplExpr>),
        Variant9(Vec<(SimplExpr, SimplExpr)>),
        Variant10(core::option::Option<SimplExpr>),
        Variant11(core::option::Option<(SimplExpr, SimplExpr)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 1
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 2
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 3
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 4
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -14, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 5
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, -18,
        // State 6
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 7
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 8
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 9
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 10
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 11
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 12
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 13
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 14
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 15
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 16
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 17
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 18
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 19
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 20
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 21
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 22
        2, 0, 0, 0, 3, -16, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -16, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 23
        2, 0, 0, 0, 3, -14, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 24
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, -20,
        // State 25
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 26
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 27
        2, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 37, 38, 39, 40, 6, 0, 0,
        // State 28
        0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, -34, -34,
        // State 29
        0, -37, -37, -37, 0, -37, -37, -37, -37, -37, 41, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 7, -37, 0, 0, 0, 0, 0, 0, -37, -37,
        // State 30
        0, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41, -41,
        // State 31
        0, -44, 8, -44, 0, -44, 9, -44, -44, -44, 0, 10, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 32
        0, -52, 0, -52, 0, -52, 0, 11, -52, 12, 0, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, -52, -52,
        // State 33
        0, 13, 0, -56, 0, -56, 0, 0, -56, 0, 0, 0, -56, 14, 15, 16, 17, 18, 19, -56, -56, 0, -56, 0, 0, 0, 0, 0, 0, -56, -56,
        // State 34
        0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 35
        0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, -26,
        // State 36
        0, -27, -27, -27, 24, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 37
        0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 38
        0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 39
        0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, -25,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35, -35,
        // State 42
        0, 0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 20, 0, -22, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 26, 21, 0, -22, 0, 0, 0, 0, 0, 0, 22, -22,
        // State 44
        0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36, -36,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, -13, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 51
        0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0, 0, 0, 0, 0, -33, -33,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40, -40,
        // State 54
        0, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38,
        // State 55
        0, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, -39,
        // State 56
        0, -42, 8, -42, 0, -42, 9, -42, -42, -42, 0, 10, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42, -42,
        // State 57
        0, -43, 8, -43, 0, -43, 9, -43, -43, -43, 0, 10, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 58
        0, -46, 0, -46, 0, -46, 0, 11, -46, 12, 0, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46, -46,
        // State 59
        0, -50, 0, -50, 0, -50, 0, 11, -50, 12, 0, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50, -50,
        // State 60
        0, -48, 0, -48, 0, -48, 0, 11, -48, 12, 0, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48, -48,
        // State 61
        0, -45, 0, -45, 0, -45, 0, 11, -45, 12, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 62
        0, -51, 0, -51, 0, -51, 0, 11, -51, 12, 0, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, -51, -51,
        // State 63
        0, -49, 0, -49, 0, -49, 0, 11, -49, 12, 0, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49, -49,
        // State 64
        0, -47, 0, -47, 0, -47, 0, 11, -47, 12, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47, -47,
        // State 65
        0, 13, 0, -53, 0, -53, 0, 0, -53, 0, 0, 0, -53, 14, 15, 16, 17, 18, 19, -53, -53, 0, -53, 0, 0, 0, 0, 0, 0, -53, -53,
        // State 66
        0, 13, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, -55, 14, 15, 16, 17, 18, 19, -55, -55, 0, -55, 0, 0, 0, 0, 0, 0, -55, -55,
        // State 67
        0, 13, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, -54, 14, 15, 16, 17, 18, 19, -54, -54, 0, -54, 0, 0, 0, 0, 0, 0, -54, -54,
        // State 68
        0, -28, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 69
        0, 0, 0, 0, 0, -15, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, -29, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 71
        -4, 0, 0, 0, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, -4, -4, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 74
        0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, -30, -30,
        // State 75
        -9, 0, 0, 0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 76
        0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0, 0, 0, 0, 0, -32, -32,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        -5, 0, 0, 0, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0,
        // State 79
        0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, -31, -31,
        // State 80
        -10, 0, 0, 0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 82
        0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 31 + integer]
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
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -34,
        // State 29
        -37,
        // State 30
        -41,
        // State 31
        -44,
        // State 32
        -52,
        // State 33
        -56,
        // State 34
        -70,
        // State 35
        -26,
        // State 36
        -27,
        // State 37
        -24,
        // State 38
        -23,
        // State 39
        -25,
        // State 40
        0,
        // State 41
        -35,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -33,
        // State 52
        0,
        // State 53
        -40,
        // State 54
        -38,
        // State 55
        -39,
        // State 56
        -42,
        // State 57
        -43,
        // State 58
        -46,
        // State 59
        -50,
        // State 60
        -48,
        // State 61
        -45,
        // State 62
        -51,
        // State 63
        -49,
        // State 64
        -47,
        // State 65
        -53,
        // State 66
        -55,
        // State 67
        -54,
        // State 68
        -28,
        // State 69
        0,
        // State 70
        -29,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        -30,
        // State 75
        0,
        // State 76
        -32,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -31,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 22,
            5 => 24,
            8 => match state {
                23 => 72,
                _ => 46,
            },
            9 => 48,
            10 => match state {
                5 | 24 => 49,
                26 => 81,
                27 => 82,
                _ => 42,
            },
            11 => 28,
            12 => 29,
            13 => match state {
                1 => 41,
                3 => 45,
                7 => 53,
                8 => 54,
                9 => 55,
                _ => 30,
            },
            14 => match state {
                10 => 56,
                11 => 57,
                _ => 31,
            },
            15 => match state {
                12 => 58,
                13 => 59,
                14 => 60,
                15 => 61,
                16 => 62,
                17 => 63,
                18 => 64,
                _ => 32,
            },
            16 => match state {
                19 => 65,
                20 => 66,
                21 => 67,
                _ => 33,
            },
            17 => match state {
                0 => 34,
                _ => 43,
            },
            18 => match state {
                2 => 44,
                6 => 52,
                22 => 69,
                25 => 77,
                _ => 47,
            },
            20 => match state {
                24 => 73,
                _ => 50,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""%""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###""=~""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""?:""###,
            r###""[""###,
            r###""]""###,
            r###""false""###,
            r###""identifier""###,
            r###""number""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
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
        fid: usize,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SimplExpr;
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
            __action(state, 31 - 1)
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
                self.fid,
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
            Token::Not if true => Some(0),
            Token::NotEquals if true => Some(1),
            Token::Mod if true => Some(2),
            Token::And if true => Some(3),
            Token::LPren if true => Some(4),
            Token::RPren if true => Some(5),
            Token::Times if true => Some(6),
            Token::Plus if true => Some(7),
            Token::Comma if true => Some(8),
            Token::Minus if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Div if true => Some(11),
            Token::Colon if true => Some(12),
            Token::LT if true => Some(13),
            Token::LE if true => Some(14),
            Token::Equals if true => Some(15),
            Token::RegexMatch if true => Some(16),
            Token::GT if true => Some(17),
            Token::GE if true => Some(18),
            Token::Question if true => Some(19),
            Token::Elvis if true => Some(20),
            Token::LBrack if true => Some(21),
            Token::RBrack if true => Some(22),
            Token::False if true => Some(23),
            Token::Ident(_) if true => Some(24),
            Token::NumLit(_) if true => Some(25),
            Token::StringLit(_) if true => Some(26),
            Token::True if true => Some(27),
            Token::LCurl if true => Some(28),
            Token::Or if true => Some(29),
            Token::RCurl if true => Some(30),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 27 | 28 | 29 | 30 => __Symbol::Variant0(__token),
            24 | 25 => match __token {
                Token::Ident(__tok0) | Token::NumLit(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            26 => match __token {
                Token::StringLit(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct Expr6Parser {
        _priv: (),
    }

    impl Expr6Parser {
        pub fn new() -> Expr6Parser {
            Expr6Parser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            fid: usize,
            __tokens0: __TOKENS,
        ) -> Result<SimplExpr, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    fid,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        fid: usize,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SimplExpr,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                // Expr0 = "string" => ActionFn(101);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action101::<>(fid, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 11)
            }
            23 => {
                __reduce23(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(fid, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                // __Expr6 = Expr6 => ActionFn(6);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(fid, __sym0);
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
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (SimplExpr, SimplExpr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, SimplExpr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Sp<StrLitSegment>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<SimplExpr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",") = ExprReset, "," => ActionFn(54);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")* = (<ExprReset> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = ExprReset, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<ExprReset> ",")+ = (<ExprReset> ",")+, ExprReset, "," => ActionFn(65);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action65::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",") = JsonKeyValue, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action57::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")* = (<JsonKeyValue> ",")+ => ActionFn(58);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = JsonKeyValue, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<JsonKeyValue> ",")+ = (<JsonKeyValue> ",")+, JsonKeyValue, "," => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = ExprReset => ActionFn(128);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action129::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce14<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+, ExprReset => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action130::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<ExprReset> = (<ExprReset> ",")+ => ActionFn(131);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = JsonKeyValue => ActionFn(132);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action132::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> =  => ActionFn(133);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action133::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce18<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+, JsonKeyValue => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<JsonKeyValue> = (<JsonKeyValue> ",")+ => ActionFn(135);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action135::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6, "?", ExprReset, ":", Expr => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(fid, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce21<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "number" => ActionFn(102);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "true" => ActionFn(103);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "false" => ActionFn(104);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action104::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "identifier" => ActionFn(105);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "(", ExprReset, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce28<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "[", Comma<ExprReset>, "]" => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce29<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = "{", Comma<JsonKeyValue>, "}" => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action107::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce30<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "identifier", "(", Comma<ExprReset>, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce31<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, "[", ExprReset, "]" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(fid, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 12)
    }
    pub(crate) fn __reduce32<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr1, ".", "identifier" => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce33<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(19);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "!", Expr2 => ActionFn(111);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action111::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce35<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = "-", Expr2 => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112::<>(fid, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce36<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(22);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce37<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "*", Expr2 => ActionFn(113);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action113::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce38<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "/", Expr2 => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "%", Expr2 => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce40<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce41<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "+", Expr3 => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce42<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "-", Expr3 => ActionFn(117);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action117::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce43<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(29);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(118);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action118::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce45<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce46<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">=", Expr4 => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce47<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<=", Expr4 => ActionFn(121);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action121::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce48<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, ">", Expr4 => ActionFn(122);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action122::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "<", Expr4 => ActionFn(123);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action123::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce50<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "=~", Expr4 => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce51<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce52<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&&", Expr5 => ActionFn(125);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action125::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce53<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "||", Expr5 => ActionFn(126);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action126::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce54<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "?:", Expr5 => ActionFn(127);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action127::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce55<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce56<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset = Expr => ActionFn(44);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce57<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? = ExprReset => ActionFn(50);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce58<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprReset? =  => ActionFn(51);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action51::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce59<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue = Expr, ":", Expr => ActionFn(45);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(fid, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce60<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? = JsonKeyValue => ActionFn(55);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce61<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // JsonKeyValue? =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(fid, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce62<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(7);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce63<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr0 = Expr0 => ActionFn(0);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce64<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr1 = Expr1 => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce65<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr2 = Expr2 => ActionFn(2);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr3 = Expr3 => ActionFn(3);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr4 = Expr4 => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce68<
    >(
        fid: usize,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr5 = Expr5 => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(fid, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 28)
    }
}
pub use self::__parse__Expr6::Expr6Parser;

#[allow(unused_variables)]
fn __action0<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action1<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action2<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action3<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action4<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action5<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action6<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action7<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action8<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, Vec<Sp<StrLitSegment>>, usize),
    (_, r, _): (usize, usize, usize),
) -> Result<SimplExpr,__lalrpop_util::ParseError<usize,Token,LexicalError>>
{
    parse_stringlit(Span(l, r, fid), x)
}

#[allow(unused_variables)]
fn __action9<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    SimplExpr::literal(Span(l, r, fid), x)
}

#[allow(unused_variables)]
fn __action10<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    SimplExpr::literal(Span(l, r, fid), "true".into())
}

#[allow(unused_variables)]
fn __action11<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    SimplExpr::literal(Span(l, r, fid), "false".into())
}

#[allow(unused_variables)]
fn __action12<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, ident, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    VarRef(Span(l, r, fid), VarName(ident.to_string()))
}

#[allow(unused_variables)]
fn __action13<
>(
    fid: usize,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action14<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, values, _): (usize, Vec<SimplExpr>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    SimplExpr::JsonArray(Span(l, r, fid), values)
}

#[allow(unused_variables)]
fn __action15<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, values, _): (usize, Vec<(SimplExpr, SimplExpr)>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    SimplExpr::JsonObject(Span(l, r, fid), values)
}

#[allow(unused_variables)]
fn __action16<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, ident, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, args, _): (usize, Vec<SimplExpr>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    FunctionCall(Span(l, r, fid), ident, args)
}

#[allow(unused_variables)]
fn __action17<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, value, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, index, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    JsonAccess(Span(l, r, fid), b(value), b(index))
}

#[allow(unused_variables)]
fn __action18<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, value, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, lit_l, _): (usize, usize, usize),
    (_, index, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    {
    JsonAccess(Span(l, r, fid), b(value), b(Literal(index.into())))
  }
}

#[allow(unused_variables)]
fn __action19<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action20<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    UnaryOp(Span(l, r, fid), Not, b(e))
}

#[allow(unused_variables)]
fn __action21<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    UnaryOp(Span(l, r, fid), Negative, b(e))
}

#[allow(unused_variables)]
fn __action22<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action23<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Times,       b(re))
}

#[allow(unused_variables)]
fn __action24<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Div,         b(re))
}

#[allow(unused_variables)]
fn __action25<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Mod,         b(re))
}

#[allow(unused_variables)]
fn __action26<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action27<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Plus,        b(re))
}

#[allow(unused_variables)]
fn __action28<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Minus,       b(re))
}

#[allow(unused_variables)]
fn __action29<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action30<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Equals,     b(re))
}

#[allow(unused_variables)]
fn __action31<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), NotEquals,  b(re))
}

#[allow(unused_variables)]
fn __action32<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), GE,         b(re))
}

#[allow(unused_variables)]
fn __action33<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), LE,         b(re))
}

#[allow(unused_variables)]
fn __action34<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), GT,         b(re))
}

#[allow(unused_variables)]
fn __action35<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), LT,         b(re))
}

#[allow(unused_variables)]
fn __action36<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), RegexMatch, b(re))
}

#[allow(unused_variables)]
fn __action37<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action38<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), And,        b(re))
}

#[allow(unused_variables)]
fn __action39<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Or,         b(re))
}

#[allow(unused_variables)]
fn __action40<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, le, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, re, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    BinOp(Span(l, r, fid), b(le), Elvis,      b(re))
}

#[allow(unused_variables)]
fn __action41<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action42<
>(
    fid: usize,
    (_, l, _): (usize, usize, usize),
    (_, cond, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, then, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, els, _): (usize, SimplExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> SimplExpr
{
    {
    IfElse(Span(l, r, fid), b(cond), b(then), b(els))
  }
}

#[allow(unused_variables)]
fn __action43<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action44<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action45<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, __1, _): (usize, SimplExpr, usize),
) -> (SimplExpr, SimplExpr)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action46<
>(
    fid: usize,
    (_, mut v, _): (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
    (_, e, _): (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize),
) -> Vec<(SimplExpr, SimplExpr)>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action47<
>(
    fid: usize,
    (_, mut v, _): (usize, alloc::vec::Vec<SimplExpr>, usize),
    (_, e, _): (usize, core::option::Option<SimplExpr>, usize),
) -> Vec<SimplExpr>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action48<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action49<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action50<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> core::option::Option<SimplExpr>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action51<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<SimplExpr>
{
    None
}

#[allow(unused_variables)]
fn __action52<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<SimplExpr>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action53<
>(
    fid: usize,
    (_, v, _): (usize, alloc::vec::Vec<SimplExpr>, usize),
) -> alloc::vec::Vec<SimplExpr>
{
    v
}

#[allow(unused_variables)]
fn __action54<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
    (_, _, _): (usize, Token, usize),
) -> SimplExpr
{
    __0
}

#[allow(unused_variables)]
fn __action55<
>(
    fid: usize,
    (_, __0, _): (usize, (SimplExpr, SimplExpr), usize),
) -> core::option::Option<(SimplExpr, SimplExpr)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action56<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<(SimplExpr, SimplExpr)>
{
    None
}

#[allow(unused_variables)]
fn __action57<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(SimplExpr, SimplExpr)>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action58<
>(
    fid: usize,
    (_, v, _): (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
) -> alloc::vec::Vec<(SimplExpr, SimplExpr)>
{
    v
}

#[allow(unused_variables)]
fn __action59<
>(
    fid: usize,
    (_, __0, _): (usize, (SimplExpr, SimplExpr), usize),
    (_, _, _): (usize, Token, usize),
) -> (SimplExpr, SimplExpr)
{
    __0
}

#[allow(unused_variables)]
fn __action60<
>(
    fid: usize,
    (_, __0, _): (usize, (SimplExpr, SimplExpr), usize),
) -> alloc::vec::Vec<(SimplExpr, SimplExpr)>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action61<
>(
    fid: usize,
    (_, v, _): (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
    (_, e, _): (usize, (SimplExpr, SimplExpr), usize),
) -> alloc::vec::Vec<(SimplExpr, SimplExpr)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action62<
>(
    fid: usize,
    (_, __0, _): (usize, SimplExpr, usize),
) -> alloc::vec::Vec<SimplExpr>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action63<
>(
    fid: usize,
    (_, v, _): (usize, alloc::vec::Vec<SimplExpr>, usize),
    (_, e, _): (usize, SimplExpr, usize),
) -> alloc::vec::Vec<SimplExpr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action64<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<SimplExpr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action54(
        fid,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        fid,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<SimplExpr>, usize),
    __1: (usize, SimplExpr, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<SimplExpr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action54(
        fid,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
>(
    fid: usize,
    __0: (usize, core::option::Option<SimplExpr>, usize),
) -> Vec<SimplExpr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        fid,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action67<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<SimplExpr>, usize),
    __1: (usize, core::option::Option<SimplExpr>, usize),
) -> Vec<SimplExpr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        fid,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        fid,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action68<
>(
    fid: usize,
    __0: (usize, (SimplExpr, SimplExpr), usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action59(
        fid,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        fid,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action69<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
    __1: (usize, (SimplExpr, SimplExpr), usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action59(
        fid,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action70<
>(
    fid: usize,
    __0: (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize),
) -> Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action57(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        fid,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action71<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
    __1: (usize, core::option::Option<(SimplExpr, SimplExpr)>, usize),
) -> Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        fid,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        fid,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action72<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, Token, usize),
    __4: (usize, SimplExpr, usize),
    __5: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action73<
>(
    fid: usize,
    __0: (usize, Vec<Sp<StrLitSegment>>, usize),
    __1: (usize, usize, usize),
) -> Result<SimplExpr,__lalrpop_util::ParseError<usize,Token,LexicalError>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        fid,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action74<
>(
    fid: usize,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        fid,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action75<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        fid,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action76<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        fid,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action77<
>(
    fid: usize,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        fid,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action78<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, Vec<SimplExpr>, usize),
    __2: (usize, Token, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action79<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, Vec<(SimplExpr, SimplExpr)>, usize),
    __2: (usize, Token, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action80<
>(
    fid: usize,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<SimplExpr>, usize),
    __3: (usize, Token, usize),
    __4: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action81<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, Token, usize),
    __4: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action82<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, String, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __start1 = __1.2.clone();
    let __end1 = __2.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action49(
        fid,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action18(
        fid,
        __temp0,
        __0,
        __1,
        __temp1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action83<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, SimplExpr, usize),
    __2: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        fid,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action84<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, SimplExpr, usize),
    __2: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        fid,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action85<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action86<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action87<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action88<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action89<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action90<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action91<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action92<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action93<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action94<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action95<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action96<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action97<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action98<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action99<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, usize, usize),
) -> SimplExpr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action49(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        fid,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action100<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, Token, usize),
    __4: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __4.2.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        fid,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action101<
>(
    fid: usize,
    __0: (usize, Vec<Sp<StrLitSegment>>, usize),
) -> Result<SimplExpr,__lalrpop_util::ParseError<usize,Token,LexicalError>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action102<
>(
    fid: usize,
    __0: (usize, String, usize),
) -> SimplExpr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action103<
>(
    fid: usize,
    __0: (usize, Token, usize),
) -> SimplExpr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action104<
>(
    fid: usize,
    __0: (usize, Token, usize),
) -> SimplExpr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action105<
>(
    fid: usize,
    __0: (usize, String, usize),
) -> SimplExpr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, Vec<SimplExpr>, usize),
    __2: (usize, Token, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, Vec<(SimplExpr, SimplExpr)>, usize),
    __2: (usize, Token, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
>(
    fid: usize,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<SimplExpr>, usize),
    __3: (usize, Token, usize),
) -> SimplExpr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        fid,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
    __3: (usize, Token, usize),
) -> SimplExpr
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        fid,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, String, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        fid,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
>(
    fid: usize,
    __0: (usize, Token, usize),
    __1: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        fid,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action113<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action116<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action117<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action118<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action119<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action120<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action121<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action122<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action123<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action124<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action125<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action97(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action126<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action127<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
    __1: (usize, Token, usize),
    __2: (usize, SimplExpr, usize),
) -> SimplExpr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        fid,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action128<
>(
    fid: usize,
    __0: (usize, SimplExpr, usize),
) -> Vec<SimplExpr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action50(
        fid,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        fid,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action129<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<SimplExpr>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action51(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        fid,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action130<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<SimplExpr>, usize),
    __1: (usize, SimplExpr, usize),
) -> Vec<SimplExpr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        fid,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action131<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<SimplExpr>, usize),
) -> Vec<SimplExpr>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action51(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action132<
>(
    fid: usize,
    __0: (usize, (SimplExpr, SimplExpr), usize),
) -> Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action55(
        fid,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        fid,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action133<
>(
    fid: usize,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action56(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        fid,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action134<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
    __1: (usize, (SimplExpr, SimplExpr), usize),
) -> Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action55(
        fid,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        fid,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action135<
>(
    fid: usize,
    __0: (usize, alloc::vec::Vec<(SimplExpr, SimplExpr)>, usize),
) -> Vec<(SimplExpr, SimplExpr)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        fid,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        fid,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>>;
}

impl<> __ToTriple<> for (usize, Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), LexicalError> {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
