use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

use crate::statement::Statement;

use crate::latex::FormatArgs;

pub mod unit;
use unit::parse_unit_expr;

pub mod expr;
use expr::parse_expr;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MathParser;

fn parse_var_dec(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::var_dec);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let rhs = inner.next().unwrap();
    Statement::VarDec {
        lhs: lhs.as_str().to_string(),
        rhs: parse_expr(rhs),
    }
}

fn parse_print_stmt(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::print_expr);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let unit_hint = inner.next().map(|n| {
        let s = n.as_str();
        FormatArgs::UnitHint {
            value: parse_unit_expr(n).eval(),
            string: s.to_string(),
        }
    });

    Statement::PrintExpr {
        expr: parse_expr(lhs),
        unit_hint,
    }
}

fn parse_dec_print_stmt(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::dec_print_expr);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let rhs = inner.next().unwrap();
    let unit_hint = inner.next().map(|n| {
        let s = n.as_str();
        FormatArgs::UnitHint {
            value: parse_unit_expr(n).eval(),
            string: s.to_string(),
        }
    });

    Statement::DecPrintExpr {
        lhs: lhs.as_str().to_string(),
        rhs: parse_expr(rhs),
        unit_hint,
    }
}

pub fn parse_block(s: &str) -> Vec<Statement> {
    let inp = MathParser::parse(Rule::program, s).unwrap();
    inp.map(|s| {
        let stmt = s.into_inner().next().unwrap();
        match stmt.as_rule() {
            Rule::expression => Statement::ExprStmt(parse_expr(stmt)),
            Rule::var_dec => parse_var_dec(stmt),
            Rule::print_expr => parse_print_stmt(stmt),
            Rule::dec_print_expr => parse_dec_print_stmt(stmt),
            _ => unreachable!(),
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        // just check if it doesn't crash rn
        parse_block(
            "
                x = 5
                5 + 10
            ",
        );
    }
}
