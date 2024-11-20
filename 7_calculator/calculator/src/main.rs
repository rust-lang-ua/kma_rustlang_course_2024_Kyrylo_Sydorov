use pest::Parser;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest_derive::Parser;
use std::io::{self, BufRead};
use lazy_static::lazy_static;

#[derive(Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " }

integer = @{ ASCII_DIGIT+ }

add = { "+" }
subtract = { "-" }
multiply = { "*" }
divide = { "/" }
modulo = { "%" }
unary_minus = { "-" }

bin_op = _{ add | subtract | multiply | divide | modulo }

atom = _{ unary_minus? ~ (integer | "(" ~ expr ~ ")") }

expr = { atom ~ (bin_op ~ atom)* }

equation = _{ SOI ~ expr ~ EOI }
"#]
struct CalculatorParser;

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: OpType,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        PrattParser::new()
            // Define precedence from lowest to highest
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
            .op(Op::infix(Rule::multiply, Assoc::Left) | Op::infix(Rule::divide, Assoc::Left) | Op::infix(Rule::modulo, Assoc::Left))
            .op(Op::prefix(Rule::unary_minus))
    };
}

pub fn parse_expr(pairs: pest::iterators::Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => OpType::Add,
                Rule::subtract => OpType::Subtract,
                Rule::multiply => OpType::Multiply,
                Rule::divide => OpType::Divide,
                Rule::modulo => OpType::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

impl Expr {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expr::Integer(value) => *value,
            Expr::UnaryMinus(expr) => -expr.evaluate(),
            Expr::BinOp { lhs, op, rhs } => {
                let left = lhs.evaluate();
                let right = rhs.evaluate();
                match op {
                    OpType::Add => left + right,
                    OpType::Subtract => left - right,
                    OpType::Multiply => left * right,
                    OpType::Divide => left / right,
                    OpType::Modulo => left % right,
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Result: {}", expr.evaluate());
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}
