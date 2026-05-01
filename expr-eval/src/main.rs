use std::{fmt::Display, iter::Peekable, str::Chars};

// 自定义错误类型
#[derive(Debug)]
enum ExprError {
    Parse(String),
}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ExprError {}

// 自定义 Result 类型
type Result<T> = std::result::Result<T, ExprError>;

#[derive(Debug, Clone, Copy, PartialEq)]

// 结合类型
enum AssocType {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]

// 约定的几种 Token
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LeftParen,  // 左括号
    RightParen, // 右括号
}

const OPERATOR_TOKEN_LIST: [Token; 5] = [
    Token::Plus,
    Token::Minus,
    Token::Multiply,
    Token::Divide,
    Token::Power,
];

impl Token {
    // 判断是不是 运算符
    fn is_operator(&self) -> bool {
        // match self {
        //     Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power => true,
        //     _ => false,
        // }
        // 实现 PartialEq trait 方便判断相等
        // OPERATOR_TOKEN_LIST.contains(self)
        // matches! 宏
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power
        )
    }
    // 判断运算符的优先级 数值越大，优先级越高
    fn get_precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Power => 3,
            _ => 0,
        }
    }
    // 获取运算符的 结合性
    fn get_assoc(&self) -> AssocType {
        match self {
            Token::Power => AssocType::Right,
            _ => AssocType::Left,
        }
    }
    // 根据当前运算符进行计算
    fn compute(&self, l: i32, r: i32) -> Option<i32> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divide => Some(l / r),
            Token::Power => Some(l.pow(r.try_into().unwrap())), // i32 -> u32
            // Token::Power => Some(l.pow(r as u32)), // i32 -> u32
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Plus => "+".to_string(),
                Token::Minus => "-".to_string(),
                Token::Multiply => "*".to_string(),
                Token::Divide => "/".to_string(),
                Token::Power => "^".to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => ")".to_string(),
            }
        )
    }
}

// 将一个算术表达式解析成连续的 Token
// 并通过 Iterator 返回，也可以通过 Peekable 接口获取

// Peekable = 给迭代器加一个「偷看一眼」的功能它允许你：不消费元素、不移动指针，先看下一个元素是什么。
// 解析 token 时，看看下一个字符是不是符号
struct Tokenizer<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(string_expr: &'a str) -> Self {
        Self {
            tokens: string_expr.chars().peekable(),
        }
    }

    // 跳过空白字符
    fn jump_whitespace(&mut self) {
        // Some(c) 和 Some(&c)得区别
        while let Some(c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }
    // 扫描数字
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();
        while let Some(&c) = self.tokens.peek() {
            if c.is_numeric() {
                num.push(c);
                self.tokens.next(); // 指针必须往后移动
            } else {
                break;
            }
        }
        // 转换为  Token::Number
        // 字符串 转数字
        match num.parse() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => None,
        }
    }
    // 扫描运算符
    fn scan_operator(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}

// 实现一个 Iterator，让解析后的 Token 可以通过迭代器进行返回
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        // 跳过 空白字符
        self.jump_whitespace();

        // 解析当前位置的 Token 类型
        match self.tokens.peek() {
            Some(c) if c.is_numeric() => self.scan_number(),
            Some(_) => self.scan_operator(),
            None => return None,
        }
    }
}

// 表达式结构体
struct Expr<'a> {
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    fn new(string_expr: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(string_expr).peekable(), // 这里因为 我们为 Tokenizer 实现了  Iterator ，所以可以直接调用  peekable
        }
    }

    // 计算表达式，获取结果
    pub fn eval(&mut self) -> Result<i32> {
        let result = self.compute_expr(1)?;
        // 如果还有 Token 没有处理，说明表达式存在错误
        if self.iter.peek().is_some() {
            return Err(ExprError::Parse("Unexpected end of expr".into()));
        }
        Ok(result)
    }

    // 计算单个 Token或者子表达式
    fn compute_atom(&mut self) -> Result<i32> {
        // println!("compute_atom {}");
        match self.iter.peek() {
            //数字直接返回
            Some(Token::Number(n)) => {
                // 提前结束 引用的生命周期，否则下一行会报错  cannot borrow `self.iter` as mutable more than once at a time second mutable borrow occurs here
                // 这里提前 拷贝，提前结束 引用的生命周期 ！！！
                let value = *n;
                self.iter.next();
                return Ok(value);
            }
            // 如果是左括号的话，递归计算括号内 表达式的值
            Some(Token::LeftParen) => {
                self.iter.next();
                // 计算
                let result = self.compute_expr(1)?; // ?向外传递   Result<i32>
                match self.iter.next() {
                    Some(Token::RightParen) => (),
                    // into  相等于 U::from(self)
                    _ => return Err(ExprError::Parse("Unexpected character".into())),
                }

                return Ok(result);
            }
            _ => {
                return Err(ExprError::Parse(
                    "Expecting a number or left parenthesis".into(),
                ));
            }
        }
    }
    //  计算表达式
    fn compute_expr(&mut self, min_prec: i32) -> Result<i32> {
        let mut atom_lhs = self.compute_atom()?;
        loop {
            let cur_token = self.iter.peek();

            if cur_token.is_none() {
                break;
            }
            // 取址符  获取token值 ， 让引用尽早结束生命周期！！！
            let token = *cur_token.expect("not a valid token");

            // 保证 以下条件
            // 1. Token  一定是运算符
            // 2. Token 得优先级必须 >= min_prec
            if !token.is_operator() || token.get_precedence() < min_prec {
                break;
            }

            let mut precedence = token.get_precedence();
            // 如果是左结合  递归计算时的 precedence 需要递增,保证左侧的先计算，除非右侧存在更高的优先级
            if token.get_assoc() == AssocType::Left {
                precedence += 1;
            }

            self.iter.next();

            // 递归计算右侧的表达式
            let atom_rhs = self.compute_expr(precedence)?;
            // 得到左右两侧的值

            match token.compute(atom_lhs, atom_rhs) {
                Some(res) => atom_lhs = res,
                None => return Err(ExprError::Parse("Unexpected expr".into())),
            }
        }
        Ok(atom_lhs)
    }
}

fn main() {
    println!("expr-eval");
    let string_expr = "92 + 5 + 5 * 27 - (92 - 12) / 4 + 26";
    let mut expr = Expr::new(string_expr);
    let result = expr.eval();
    println!("res = {:?}", result);
}
