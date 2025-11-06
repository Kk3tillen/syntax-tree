use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Expression {
    Number(i64),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Remainder(Box<Expression>, Box<Expression>),
    Negation(Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> Option<i64> {
        match self {
            Expression::Number(n) => Some(*n),
            Expression::Addition(left, right) => {
                let v_left = left.evaluate()?;
                let v_right = right.evaluate()?;
                v_left.checked_add(v_right)
            }
            Expression::Subtraction(left, right) => {
                let v_left = left.evaluate()?;
                let v_right = right.evaluate()?;
                v_left.checked_sub(v_right)
            }
            Expression::Multiplication(left, right) => {
                let v_left = left.evaluate()?;
                let v_right = right.evaluate()?;
                v_left.checked_mul(v_right)
            }
            Expression::Division(left, right) => {
                let v_left = left.evaluate()?;
                let v_right = right.evaluate()?;
                if v_right == 0 {
                    return None;
                }
                v_left.checked_div(v_right)
            }
            Expression::Remainder(left, right) => {
                let v_left = left.evaluate()?;
                let v_right = right.evaluate()?;
                if v_right == 0 {
                    return None;
                }
                v_left.checked_rem(v_right)
            }
            Expression::Negation(expr) => {
                let v = expr.evaluate()?;
                v.checked_neg()
            }
        }
    }

    fn print(&self) {
        print!("{}", self.to_string_prec(0));
    }

    fn to_string_prec(&self, parent_prec: u8) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Negation(expr) => {
                let s = format!("-{}", expr.to_string_prec(5));
                if parent_prec > 5 {
                    format!("({})", s)
                } else {
                    s
                }
            }
            Expression::Multiplication(left, right) => {
                let s = format!("{} * {}", left.to_string_prec(3), right.to_string_prec(4));
                if parent_prec > 3 {
                    format!("({})", s)
                } else {
                    s
                }
            }
            Expression::Division(left, right) => {
                let s = format!("{} / {}", left.to_string_prec(3), right.to_string_prec(4));
                if parent_prec > 3 {
                    format!("({})", s)
                } else {
                    s
                }
            }
            Expression::Remainder(left, right) => {
                let s = format!("{} % {}", left.to_string_prec(3), right.to_string_prec(4));
                if parent_prec > 3 {
                    format!("({})", s)
                } else {
                    s
                }
            }
            Expression::Addition(left, right) => {
                let s = format!("{} + {}", left.to_string_prec(1), right.to_string_prec(2));
                if parent_prec > 1 {
                    format!("({})", s)
                } else {
                    s
                }
            }
            Expression::Subtraction(left, right) => {
                let s = format!("{} - {}", left.to_string_prec(1), right.to_string_prec(2));
                if parent_prec > 1 {
                    format!("({})", s)
                } else {
                    s
                }
            }
        }
    }

    fn print_tree(&self) {
        self.print_tree_recursive("", true);
    }

    fn print_tree_recursive(&self, prefix: &str, is_last: bool) {
        let current_symbol = if is_last { "└" } else { "├" };
        let child_prefix = if is_last { "  " } else { "│ " };

        match self {
            Expression::Number(n) => {
                if !prefix.is_empty() {
                    println!("{}{} {}", prefix, current_symbol, n);
                } else {
                    println!("{}", n);
                }
            }
            Expression::Negation(expr) => {
                if !prefix.is_empty() {
                    println!("{}{} -", prefix, current_symbol);
                } else {
                    println!("-");
                }
                expr.print_tree_recursive(&format!("{}{}", prefix, child_prefix), true);
            }
            Expression::Addition(left, right) => {
                if !prefix.is_empty() {
                    println!("{}{} +", prefix, current_symbol);
                } else {
                    println!("+");
                }
                left.print_tree_recursive(&format!("{}{}", prefix, child_prefix), false);
                right.print_tree_recursive(&format!("{}{}", prefix, child_prefix), true);
            }
            Expression::Subtraction(left, right) => {
                if !prefix.is_empty() {
                    println!("{}{} -", prefix, current_symbol);
                } else {
                    println!("-");
                }
                left.print_tree_recursive(&format!("{}{}", prefix, child_prefix), false);
                right.print_tree_recursive(&format!("{}{}", prefix, child_prefix), true);
            }
            Expression::Multiplication(left, right) => {
                if !prefix.is_empty() {
                    println!("{}{} *", prefix, current_symbol);
                } else {
                    println!("*");
                }
                left.print_tree_recursive(&format!("{}{}", prefix, child_prefix), false);
                right.print_tree_recursive(&format!("{}{}", prefix, child_prefix), true);
            }
            Expression::Division(left, right) => {
                if !prefix.is_empty() {
                    println!("{}{} /", prefix, current_symbol);
                } else {
                    println!("/");
                }
                left.print_tree_recursive(&format!("{}{}", prefix, child_prefix), false);
                right.print_tree_recursive(&format!("{}{}", prefix, child_prefix), true);
            }
            Expression::Remainder(left, right) => {
                if !prefix.is_empty() {
                    println!("{}{} %", prefix, current_symbol);
                } else {
                    println!("%");
                }
                left.print_tree_recursive(&format!("{}{}", prefix, child_prefix), false);
                right.print_tree_recursive(&format!("{}{}", prefix, child_prefix), true);
            }
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    LeftParen,
    RightParen,
}

impl Parser {
    fn new(input: &str) -> Result<Self, String> {
        let tokens = Self::tokenize(input)?;
        Ok(Parser { tokens, pos: 0 })
    }

    fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\n' => {
                    chars.next();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Times);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '%' => {
                    tokens.push(Token::Modulo);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let num = num_str.parse::<i64>()
                        .map_err(|_| "Invalid number")?;
                    tokens.push(Token::Number(num));
                }
                _ => return Err(format!("Invalid character: '{}'", c)),
            }
        }

        Ok(tokens)
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.current() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expression::Addition(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expression::Subtraction(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.current() {
            match token {
                Token::Times => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::Multiplication(Box::new(left), Box::new(right));
                }
                Token::Divide => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::Division(Box::new(left), Box::new(right));
                }
                Token::Modulo => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::Remainder(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        match self.current() {
            Some(Token::Number(n)) => {
                let num = *n;
                self.advance();
                Ok(Expression::Number(num))
            }
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_factor()?;
                Ok(Expression::Negation(Box::new(expr)))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                match self.current() {
                    Some(Token::RightParen) => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err("Expected ')'".to_string()),
                }
            }
            _ => Err("Invalid expression".to_string()),
        }
    }
}

fn main() {
    println!("=== Calculadora de Expressões ===");
    println!("Digite uma expressão matemática (ou 'sair' para encerrar)");
    println!("Exemplos: 10 + 20, (10 + 20) * 30, -5 + 3\n");

    loop {
        print!("Expressão: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "sair" || input == "exit" {
            break;
        }

        match Parser::new(input) {
            Ok(mut parser) => {
                match parser.parse() {
                    Ok(expr) => {
                        println!("\nExpressão simplificada:");
                        expr.print();
                        println!("\n");

                        println!("Árvore sintática:");
                        expr.print_tree();
                        println!();

                        match expr.evaluate() {
                            Some(result) => println!("Resultado: {}\n", result),
                            None => println!("Erro: Divisão por zero ou overflow\n"),
                        }
                    }
                    Err(e) => println!("Erro ao fazer parse: {}\n", e),
                }
            }
            Err(e) => println!("Erro: {}\n", e),
        }
    }
}