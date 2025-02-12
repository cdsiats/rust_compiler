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

    let line_break_regex = Regex::new(r"^(\n|\r|\r\n)").unwrap();
    //Matches keywords as whole words
    let keywords_regex = Regex::new(r"^(plugin|use|prop|enum|type|model|String|Number|Boolean|Text|Date)\b").unwrap();
    let identifier_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let string_literal_regex = Regex::new(r#"^"([^"]+)""#).unwrap();

    while index < source_code.len() {
        let curr_substring = &source_code[index..];

        if let Some(m) = line_break_regex.find(curr_substring) {
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

        } else if let Some(m) = string_literal_regex.captures(curr_substring) { // Check STRING LITERALS FIRST
            let matched_literal = m.get(0).unwrap().as_str();

            println!("String Literal Match: {:?}", matched_literal);

            tokens.push(
                create_token(TokenType::StringLiteral, matched_literal, index, index + matched_literal.len())
            );
            index += matched_literal.len();
        } else if let Some(m) = keywords_regex.find(curr_substring) {
            let matched_keyword = m.as_str();
            let token_type = match matched_keyword {
                "plugin" => TokenType::PluginKeyword,
                "use" => TokenType::UseKeyword,
                "prop" => TokenType::PropKeyword,
                "enum" => TokenType::EnumKeyword,
                "type" => TokenType::TypeKeyword,
                "model" => TokenType::ModelKeyword,
                "String" => TokenType::StringType,
                "Number" => TokenType::NumberType,
                "Boolean" => TokenType::BooleanType,
                "Text" => TokenType::TextType,
                "Date" => TokenType::DateType,
                _ => TokenType::Identifier,
            };
            tokens.push(
                create_token(token_type, matched_keyword, index, index + matched_keyword.len())
            );
            index += matched_keyword.len();
        } else if let Some(m) = identifier_regex.find(curr_substring) {
            let matched_identifier = m.as_str();
            tokens.push(
                create_token(TokenType::Identifier, matched_identifier, index, index + matched_identifier.len())
            );
            index += matched_identifier.len();
        } 
        else {
            println!("No match, incrementing index");
            index += 1;
        }
    }
    //Return tokens vector
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn tokenize_keywords() {
        let source = "plugin use prop enum type model String Number Boolean Text Date";
        let tokens = tokenize(source);

        println!("{:#?}", tokens);

        assert_eq!(tokens.len(), 11);
        if tokens.len() >= 11 {
            assert_eq!(tokens[0].token_type, TokenType::PluginKeyword);
            assert_eq!(tokens[0].value, "plugin");

            assert_eq!(tokens[1].token_type, TokenType::UseKeyword);
            assert_eq!(tokens[1].value, "use");

            assert_eq!(tokens[2].token_type, TokenType::PropKeyword);
            assert_eq!(tokens[2].value, "prop");

            assert_eq!(tokens[3].token_type, TokenType::EnumKeyword);
            assert_eq!(tokens[3].value, "enum");

            assert_eq!(tokens[4].token_type, TokenType::TypeKeyword);
            assert_eq!(tokens[4].value, "type");

            assert_eq!(tokens[5].token_type, TokenType::ModelKeyword);
            assert_eq!(tokens[5].value, "model");

            assert_eq!(tokens[6].token_type, TokenType::StringType);
            assert_eq!(tokens[6].value, "String");

            assert_eq!(tokens[7].token_type, TokenType::NumberType);
            assert_eq!(tokens[7].value, "Number");

            assert_eq!(tokens[8].token_type, TokenType::BooleanType);
            assert_eq!(tokens[8].value, "Boolean");

            assert_eq!(tokens[9].token_type, TokenType::TextType);
            assert_eq!(tokens[9].value, "Text");

            assert_eq!(tokens[10].token_type, TokenType::DateType);
            assert_eq!(tokens[10].value, "Date");
        }
    }

    #[test]
    fn tokenize_identifiers() {
        let source = "Shirts sizes street city User Address customType";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 7);
        if tokens.len() >= 7 {
            assert_eq!(tokens[0].token_type, TokenType::Identifier);
            assert_eq!(tokens[0].value, "Shirts");

            assert_eq!(tokens[1].token_type, TokenType::Identifier);
            assert_eq!(tokens[1].value, "sizes");

            assert_eq!(tokens[2].token_type, TokenType::Identifier);
            assert_eq!(tokens[2].value, "street");

            assert_eq!(tokens[3].token_type, TokenType::Identifier);
            assert_eq!(tokens[3].value, "city");

            assert_eq!(tokens[4].token_type, TokenType::Identifier);
            assert_eq!(tokens[4].value, "User");

            assert_eq!(tokens[5].token_type, TokenType::Identifier);
            assert_eq!(tokens[5].value, "Address");

            assert_eq!(tokens[6].token_type, TokenType::Identifier);
            assert_eq!(tokens[6].value, "customType");
        }
    }

    #[test]
    fn tokenize_string_literals() {
        let source = "\"hello\" \"world\" \"This is a string with spaces\"";
        let tokens = tokenize(source);

        println!("{:#?}", source);
        println!("{:#?}", tokens);

        assert_eq!(tokens.len(), 3);
        if tokens.len() >= 2 {
            assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
            assert_eq!(tokens[0].value, "\"hello\"");

            assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
            assert_eq!(tokens[1].value, "\"world\"");

            assert_eq!(tokens[2].token_type, TokenType::StringLiteral);
            assert_eq!(tokens[2].value, "\"This is a string with spaces\"");
        }
    }

    #[test]
    fn tokenize_integers() {
        let source = "4 44 444 -4 -44";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 4);
        if tokens.len() >= 4 {
            assert_eq!(tokens[0].token_type, TokenType::NumberLiteral);
            assert_eq!(tokens[0].value, "4");

            assert_eq!(tokens[1].token_type, TokenType::NumberLiteral);
            assert_eq!(tokens[1].value, "44");

            assert_eq!(tokens[2].token_type, TokenType::NumberLiteral);
            assert_eq!(tokens[2].value, "444");

            assert_eq!(tokens[3].token_type, TokenType::NumberLiteral);
            assert_eq!(tokens[3].value, "-4");

            assert_eq!(tokens[4].token_type, TokenType::NumberLiteral);
            assert_eq!(tokens[4].value, "-44");
        }
    }
}