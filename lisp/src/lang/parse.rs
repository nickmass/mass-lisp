use combine::*;
use combine::parser::char::{string, letter, digit};
use combine::parser::repeat::{skip_many1, skip_until};
use combine::error::{ParseError, StreamError};
use combine::stream::StreamErrorFor;
use combine::stream::state::{SourcePosition, State};

use std::io::{Read};
use std::io;

use super::*;

#[derive(Debug)]
pub enum Error {
    Internal(Option<Span>, String),
    Expression(Option<Span>, String),
    Parse(Position, String),
    IoError(io::Error)
}

impl Error {
    pub fn into_exception<T: AsRef<str>>(self, module: T) -> Exception {
        let module = module.as_ref();
        let msg = match self {
            Error::Internal(location, message) => {
                let location = location
                    .map(|l| format!(":{}:{} -> {}:{}", l.start.line, l.start.column, l.end.line, l.end.column))
                    .unwrap_or("".to_string());
                format!("Internal Error: {} \n{}{}", message, module, location)
            },
            Error::Expression(location, message) => {
                let location = location
                    .map(|l| format!(":{}:{} -> {}:{}", l.start.line, l.start.column, l.end.line, l.end.column))
                    .unwrap_or("".to_string());
                format!("Expression Error: {}\n{}{}", message, module, location)
            },
            Error::Parse(location, message) => {
                format!("Parse Error: {}\n{}:{}:{}", message, module, location.line, location.column)
            },
            Error::IoError(err) => {
                format!("IO Error: {}\n{}", err, module)
            }
        };

        Exception::message(msg)
    }
}

#[derive(Debug, Clone)]
enum TokenOrExpression {
    Token(TextToken, Span),
    Expression(Expression),
}

impl From<Expression> for TokenOrExpression {
    fn from(other: Expression) -> Self {
        TokenOrExpression::Expression(other)
    }
}

struct ExpressionBuilder<'a> {
    tokens: Vec<TokenOrExpression>,
    interner: &'a mut IdentIntern,
}

impl<'a> ExpressionBuilder<'a> {
    fn new(interner: &'a mut IdentIntern) -> Self {
        Self { tokens: Vec::new(), interner }
    }

    fn push(&mut self, token: TextToken, debug_info: Span) -> Result<(), Error> {
        use self::TextToken as TT;
        match token {
            TT::Comment => (),
            TT::Whitespace => (),
            TT::OpenBrace => (),
            TT::CloseBrace => (),
            TT::CloseParen => {
                let mut tokens = Vec::new();
                loop {
                    match self.tokens.pop() {
                        Some(TokenOrExpression::Token(TT::OpenParen, _)) => break,
                        Some(t_or_e) => tokens.push(t_or_e),
                        None => {
                            return Err(Error::Expression(Some(debug_info), "Unmatched close paren".into()));
                        }
                    }
                }
                let mut expression = Expression::new();
                loop {
                    let next_token = match tokens.pop() {
                        Some(TokenOrExpression::Token(t, debug)) => match t {
                            TT::Identifier(name) => self.interner.get_or_add(name).into(),
                            TT::Keyword(key) => key.into(),
                            TT::Literal(lit) => lit.into(),
                            TT::Symbol(sym) => self.interner.get_or_add(sym.name()).into(),
                            _ => {
                                return Err(Error::Internal(Some(debug), "Unexpected token".into()));
                            }
                        },
                        Some(TokenOrExpression::Expression(e)) => {
                            e.into()
                        },
                        None => break,
                    };
                    expression.push(next_token);
                }

                self.tokens.push(expression.into());
            },
            _ => {
                self.tokens.push(TokenOrExpression::Token(token, debug_info));
            },
        }

        Ok(())
    }

    fn build(mut self) -> Result<Expression, Error> {
        if self.tokens.len() != 1 {
            for token in self.tokens {
                match token {
                    TokenOrExpression::Token(TextToken::OpenParen, debug) => return Err(Error::Expression(Some(debug), "Unmatched open paren".into())),
                    _ => (),
                }
            }
            return Err(Error::Internal(None, "Unparsed token".into()));
        }

        match self.tokens.pop() {
            Some(TokenOrExpression::Expression(e)) => Ok(e),
            Some(TokenOrExpression::Token(_, debug)) => Err(Error::Internal(Some(debug), "Unexpected token".into())),
            _ => Err(Error::Internal(None, "Unparsed token".into())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Module;

impl Module {
    pub fn load<R: Read>(interner: &mut IdentIntern, source: R) -> Result<Expression, Error> {
        let (tokens, debug_info) = Self::tokenize(source)?;
        Self::root_expression(tokens, debug_info, interner)
    }

    fn root_expression(tokens: Vec<TextToken>, debug_info: Vec<Span>, interner: &mut IdentIntern) -> Result<Expression, Error> {
        let mut builder = ExpressionBuilder::new(interner);

        builder.push(TextToken::OpenParen, Span::start())?;
        for (token, debug_info) in tokens.into_iter().zip(debug_info) {
            builder.push(token, debug_info)?;
        }
        builder.push(TextToken::CloseParen, Span::start())?;

        builder.build()
    }

    fn tokenize<R: Read>(mut source: R) -> Result<(Vec<TextToken>, Vec<Span>), Error> {
        let mut source_buf = String::new();
        source.read_to_string(&mut source_buf).map_err(Error::IoError)?;

        let stream = State::new(&*source_buf);
        let (tokens, debug_info) = Self::parse()
            .easy_parse(stream)
            .map_err(|e| {
                let message: Vec<_> = e.errors.iter().map(ToString::to_string).collect();
                Error::Parse(e.position.into(), message.join("\n"))
            })?
            .0;

        let spans = debug_info.into_iter().map(|(start, end)| Span { start, end}).collect();

        Ok((tokens, spans))
    }

    fn parse<I, P>() -> impl Parser<Input = I, Output = (Vec<TextToken>, Vec<(Position, Position)>)>
    where
        I: Stream<Item = char, Position = P>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
        P: Into<Position> + Clone + Ord,
    {
        position().then(|start: I::Position| {
            let start = start.into();
            many((parse_text_token(), position()).map(|(t, p): (_, I::Position)| (t, p.into())))
                .skip(eof())
                .map(move |tokens: Vec<(_, _)>| {
                    tokens
                        .into_iter()
                        .scan(start, |start, (token, end)| {
                            Some((token, (*start, end)))
                        })
                        .unzip()
                })
        })
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    fn start() -> Self {
        Span {
            start: Position { line: 1, column: 1},
            end: Position { line: 1, column: 1},
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: i32,
    column: i32,
}

impl From<SourcePosition> for Position {
    fn from(other: SourcePosition) -> Self {
        Position {
            line: other.line,
            column: other.column,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextToken {
    Comment,
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Symbol(Symbol),
    Whitespace,
}

impl From<Keyword> for TextToken {
    fn from(other: Keyword) -> Self {
        TextToken::Keyword(other)
    }
}

impl From<Literal> for TextToken {
    fn from(other: Literal) -> Self {
        TextToken::Literal(other)
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Add,
    Div,
    Mod,
    Mul,
    Sub,
    Eq,
    Not,
    Gt,
    Lt,
}

impl Symbol {
    fn name(&self) -> &'static str {
        use self::Symbol::*;
        match *self {
            Add => "add",
            Div => "div",
            Mod => "mod",
            Mul => "mul",
            Sub => "sub",
            Eq => "eq",
            Not => "not",
            Gt => "gt",
            Lt => "lt",
        }.into()
    }
}

fn parse_text_token<I>() -> impl Parser<Input = I, Output = TextToken>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(between(string("/*"), string("*/").map(|_|()).or(eof()), skip_until(try(string("*/")).map(|_|()).or(eof()))).map(|_| TextToken::Comment)),
        try(between(string("//"), token('\n').map(|_|()).or(eof()), skip_until(token('\n').map(|_|()).or(eof()))).map(|_| TextToken::Comment)),
        parse_literal_string(),
        parse_literal_number(),
        parse_identifier(),
        parse_symbol(),
        token('\'').map(|_| TextToken::Keyword(Keyword::Quote)),
        token('(').map(|_| TextToken::OpenParen),
        token(')').map(|_| TextToken::CloseParen),
        token('{').map(|_| TextToken::OpenBrace),
        token('}').map(|_| TextToken::CloseBrace),
        skip_many1(satisfy(|c: char| c.is_whitespace())).map(|_| TextToken::Whitespace),
    ))
}

fn parse_identifier<I>() -> impl Parser<Input = I, Output = TextToken>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((letter(), token('_'))).then(|c| {
        many(choice((letter(), digit(), token('_')))).map(move |s: String| {
            let mut ident = String::with_capacity(s.len() + 1);
            ident.push(c);
            ident.push_str(&s);
            ident
        })
    }).map(|ident| match &*ident {
        "nil" => Literal::Nil.into(),
        "true" => Literal::Boolean(true).into(),
        "false" => Literal::Boolean(false).into(),
        "func" => Keyword::Func.into(),
        "if" => Keyword::If.into(),
        "import" => Keyword::Import.into(),
        "lambda" => Keyword::Lambda.into(),
        "let" => Keyword::Let.into(),
        "set" => Keyword::Set.into(),
        "loop" => Keyword::Loop.into(),
        "quote" => Keyword::Quote.into(),
        _ => TextToken::Identifier(ident)
    })
}

fn parse_literal_string<I>() -> impl Parser<Input = I, Output = TextToken>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    token('"').with(
        many(
            token('\\')
                .with(any())
                .and_then(|escape| match escape {
                    'n' => Ok('\n'),
                    'r' => Ok('\r'),
                    't' => Ok('\t'),
                    '\\' => Ok('\\'),
                    '\"' => Ok('\"'),
                    _ => Err(StreamErrorFor::<I>::message_message(format!("Invalid escape sequence: '\\{}'", escape)))
                })
                .or(none_of("\"\\".chars()))
        )
    ).skip(token('"'))
        .message("Unclosed string literal")
        .map(Literal::String)
        .map(TextToken::Literal)
}

fn parse_literal_number<I>() -> impl Parser<Input = I, Output = TextToken>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    // TODO: Promote large integer literals to floats?
    choice((
        try(optional(choice((token('-'), token('+'))))
            .then(|sign| {
                let sign = match sign {
                    Some('-') => -1.0,
                    _ => 1.0,
                };

                many(digit()).then(move |whole: Vec<_>| {
                    let whole = whole.iter().fold(0.0, |acc, next| {
                        let next = (*next as u32 - 48) as f64;
                        acc * 10.0 + next
                    });
                    token('.').then(move |_| {
                        many1(digit()).map(move |fraction: Vec<_>| {
                            let fraction = fraction.iter().rev().fold(0.0, |acc, next| {
                                let next = (*next as u32 - 48) as f64;
                                (next + acc) / 10.0
                            });

                            (whole + fraction) * sign
                        })
                    })
                })
            }).map(Number::Float)),
        try(optional(choice((token('-'), token('+'))))
            .then(|sign| {
                let sign = match sign {
                    Some('-') => -1,
                    _ => 1,
                };

                many1(digit()).map(move |digits: Vec<char>| {
                    let val = digits.iter().fold(0, |acc, next| {
                        let next = *next as i64 - 48;
                        acc * 10 + next
                    });

                    val * sign
                })
            }).map(Number::Integer))
    )).map(Literal::Number)
        .map(TextToken::Literal)
}

fn parse_symbol<I>() -> impl Parser<Input = I, Output = TextToken>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        token('+').map(|_| Symbol::Add),
        token('-').map(|_| Symbol::Sub),
        token('/').map(|_| Symbol::Div),
        token('*').map(|_| Symbol::Mul),
        token('%').map(|_| Symbol::Mod),
        token('=').map(|_| Symbol::Eq),
        token('!').map(|_| Symbol::Not),
        token('>').map(|_| Symbol::Gt),
        token('<').map(|_| Symbol::Lt),
    )).map(TextToken::Symbol)
}

