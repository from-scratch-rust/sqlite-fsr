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

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::LeftParenthesis => "(",
            Symbol::RightParenthesis => ")",
            Symbol::Comma => ",",
            Symbol::Semicolon => ";"
        }
    }

    pub const ALL: [Symbol; 4] = [
        Symbol::LeftParenthesis,
        Symbol::RightParenthesis,
        Symbol::Comma,
        Symbol::Semicolon,
    ];
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
    let mut tokens: Vec<SQLToken> = Vec::new();
    let mut current = String::new();

    for character in value.chars() {
        // Check if character is any known symbol
        if Symbol::ALL.iter().any(|s| s.as_str() == character.to_string()) {
            if !current.is_empty() {
                tokens.push(SQLToken::Identifier(current.clone()));
                current.clear();
            }
            tokens.push(character.to_sql_token());
        } else {
            current.push(character);
        }
    }

    if !current.is_empty() {
        tokens.push(SQLToken::Identifier(current));
    }

    

    return tokens
}
