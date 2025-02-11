use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PluginKeyword,
    UseKeyword,
    PropKeyword,
    EnumKeyword,
    TypeKeyword,
    ModelKeyword,
    StringType,
    NumberType,
    BooleanType,
    TextType,
    DateType,

    //Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    //Symbols
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    AtSymbol,
    Optional,

    Identifier,

    Whitespace,
    LineBreak,
    Comment,
    Error,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

fn create_token(token_type: TokenType, value: &str, start: usize, end: usize) -> Token {
    Token {
        token_type,
        value: value.to_string(),
        start,
        end,
    }
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index: usize = 0;

    let whitespace_regex = Regex::new(r"^[ \t]+").unwrap();
    let line_break_regex = Regex::new(r"^(\n|\r|\r\n)").unwrap();

    while index < source_code.len() {
        let curr_substring = &source_code[index..];

        if let Some(m) = whitespace_regex.find(curr_substring) {
            let matched_str = m.as_str();
            tokens.push(
                create_token(
                    TokenType::Whitespace,
                    matched_str,
                    index,
                    index + matched_str.len()
                )
            );
            index += matched_str.len();
        } else if let Some(m) = line_break_regex.find(curr_substring) {
            let matched_str = m.as_str();
            tokens.push(
                create_token(
                    TokenType::LineBreak,
                    matched_str,
                    index, 
                    index + matched_str.len()
                )
            );
            index += matched_str.len();
        } 
        else {
            index += 1;
        }
    }

    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_whitespace() {
        let source = " \t ";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 1);
        if tokens.len() > 0 {
            assert_eq!(tokens[0].token_type, TokenType::Whitespace);
            assert_eq!(tokens[0].value, source);
        }
    }

    #[test]
    fn tokenize_line_break() {
        let source = "\n\r\n\r";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 4);
        if tokens.len() >= 2 {
            assert_eq!(tokens[0].token_type, TokenType::LineBreak);
            assert_eq!(tokens[0].value, "\n");

            assert_eq!(tokens[1].token_type, TokenType::LineBreak);
            assert_eq!(tokens[1].value, "\r");

            assert_eq!(tokens[2].token_type, TokenType::LineBreak);
            assert_eq!(tokens[2].value, "\n");

            assert_eq!(tokens[3].token_type, TokenType::LineBreak);
            assert_eq!(tokens[3].value, "\r");
        }
    }
}