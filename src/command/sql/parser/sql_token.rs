#[derive(Debug)]
pub enum SQLToken {
    Keyword(String),
    Identifier(String),
    Symbol(Symbol)
}

#[derive(Debug)]
pub enum Symbol {
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Semicolon
}


pub trait Tokenize {
    fn tokenize(&self) -> Vec<SQLToken>;
} 
impl Tokenize for str {
    fn tokenize(&self) -> Vec<SQLToken> {
        let mut tokens: Vec<SQLToken> = Vec::new();
        let binding = self.trim().replace("\n", " ");
        let components = binding.split_whitespace();
        for component in components.into_iter() {
            let mut component_tokens = match component {
                                        s if (s == "SELECT") | (s == "CREATE") | (s == "FROM") => vec![SQLToken::Keyword((*s).to_string())],
                                        s if s.len() == 1 => vec![s.chars().next().unwrap().to_sql_token()], //this is ugly as hell but rust std doesnt support character indexing🤷🏾‍♂️ 
                                        _ => tokenize_component(component)
                                        };
            tokens.append(&mut component_tokens);
        }
        return tokens;
    }
}

pub trait ToSQLToken {
    fn to_sql_token(&self) -> SQLToken;
} 
impl ToSQLToken for char {
    fn to_sql_token(&self) -> SQLToken {
        let token = match self {
                        '(' => SQLToken::Symbol(Symbol::LeftParenthesis),
                        ')' => SQLToken::Symbol(Symbol::RightParenthesis),
                        ',' => SQLToken::Symbol(Symbol::Comma),
                        ';' => SQLToken::Symbol(Symbol::Semicolon),
                        _ => SQLToken::Identifier(self.to_string())
                    };
        return token;
    }
}

pub fn tokenize_component(value: &str) -> Vec<SQLToken> {
    let mut value_copy = value.trim();
    let mut tokens: Vec<SQLToken> = Vec::new();
    
    while (!value_copy.is_empty()) {
        // consume leading symbol characters
        let mut left_symbols: Vec<SQLToken> = Vec::new();
        loop {
            let first_character = match value_copy.chars().next() {
                Some(character) => character,
                None => break,
            };

            match first_character.to_sql_token() {
                SQLToken::Symbol(symbol) => {
                    let offset = first_character.len_utf8();
                    value_copy = &value_copy[offset..];
                    left_symbols.push(SQLToken::Symbol(symbol));
                }
                _ => break,
            }
        }


        // consume trailing symbol characters
        let mut right_symbols: Vec<SQLToken> = Vec::new();
        loop {
            let last_character = match value_copy.chars().rev().next() {
                Some(character) => character,
                None => break,
            };
            match last_character.to_sql_token() {
                SQLToken::Symbol(symbol) => {
                    let offset = last_character.len_utf8();
                    let new_len = value_copy.len() - offset;
                    value_copy = &value_copy[..new_len];
                    right_symbols.insert(0, SQLToken::Symbol(symbol));
                }
                _ => break,
            }
        }

        tokens.extend(left_symbols);
        if !value_copy.is_empty() {tokens.push(SQLToken::Identifier(value_copy.to_string()))};
        tokens.extend(right_symbols);
    }


    

    return tokens
}
