use ast::Value;


pub fn parse(
    tokens: Vec<Token>
    diagnostics: &mut Diagnostics
) -> Option<Value> {
    todo!()
}

struct Abort;

struct Parser {
    tokens: Vec<Token>
    curr_token: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            curr_token: 0,
        }

    }
    
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    }

    fn require_current(&self) -> Result<&Token, Abort> {
        match self.current() {
            Some(&token) => Ok(&token),
            None => Err(Abort)
        }
    }

    fn  next(&mut self) {
        self.curr_token += 1;
    }

    fn expect(&mut self, expected_type: TokenType) -> Result<(), Abort> {
        let token = self.require_current()?;

        if token.token_type == expected_type {
            self.next(); 
        } else {
            let expected_types = vec![expected_type];
            //Acrescentar Diagnostics
        }

        Ok(())
    }

    fn check_expect(&mut self, expected_type: TokenType) -> Result<bool, Abort> {
        let token = self.require_current()?;

        if token.token_type == expected_type {
            self.next();
            Ok(true) 
        } else {
            Ok(false)
        }

    }

}