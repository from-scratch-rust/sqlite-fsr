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
        let components = self.trim().split(" ");
        for component in components.into_iter() {
            let mut component_tokens = match component {
                                        s if (s == "SELECT") | (s == "CREATE") | (s == "FROM") => vec![SQLToken::Keyword((*s).to_string())],
                                        s if s.len() == 1 => vec![s.chars().next().unwrap().to_sql_token()], //this is ugly as hell but rust std doesnt support character indexing🤷🏾‍♂️ 
                                        _ => {
                                            let characters: Vec<char> = component.chars().collect(); 
                                            let mut component_copy = component.trim();

                                            let beginning_character = characters[0];
                                            let left_symbol: Option<SQLToken> = match beginning_character.to_sql_token() {
                                                                                    SQLToken::Symbol(char) => {
                                                                                        component_copy = &component_copy[1..];
                                                                                        Some(SQLToken::Symbol(char))
                                                                                    }
                                                                                    _ => None
                                                                                };


                                            let ending_character = characters[characters.len()-1];
                                            let right_symbol: Option<SQLToken> = match ending_character.to_sql_token() {
                                                                                        SQLToken::Symbol(char) => {
                                                                                            component_copy = &component_copy[..component_copy.len() - 1];
                                                                                            Some(SQLToken::Symbol(char))
                                                                                        }
                                                                                        _ => None
                                                                                    };

                                            let identifier = Some(SQLToken::Identifier(component_copy.to_string()));

                                            [left_symbol, identifier, right_symbol].into_iter().filter_map(|t| t).collect()
                                            }
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
