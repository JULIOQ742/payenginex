// lexer.rs

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(f64),
    Variable(String),
    Operador(char),
    Funcion(String),
    ParentesisApertura,
    ParentesisCierre,
    Coma,
    FinDeCadena,
}

pub struct Lexer {
    formula: String,
    posicion: usize,
}

impl Lexer {
    pub fn nuevo(formula: String) -> Self {
        Lexer { formula, posicion: 0 }
    }

    pub fn siguiente_token(&mut self) -> Option<Token> {
        if self.posicion >= self.formula.len() {
            return Some(Token::FinDeCadena);
        }
        let caracter = self.formula.chars().nth(self.posicion).unwrap();
        match caracter {
            '0'..='9' => {
                let mut numero = String::new();
                while self.posicion < self.formula.len() && self.formula.chars().nth(self.posicion).unwrap().is_digit(10) {
                    numero.push(self.formula.chars().nth(self.posicion).unwrap());
                    self.posicion += 1;
                }
                if self.posicion < self.formula.len() && self.formula.chars().nth(self.posicion).unwrap() == '.' {
                    numero.push('.');
                    self.posicion += 1;
                    while self.posicion < self.formula.len() && self.formula.chars().nth(self.posicion).unwrap().is_digit(10) {
                        numero.push(self.formula.chars().nth(self.posicion).unwrap());
                        self.posicion += 1;
                    }
                }
                Some(Token::Literal(numero.parse().unwrap()))
            },
            'a'..='z' | 'A'..='Z' => {
                let mut variable = String::new();
                while self.posicion < self.formula.len() && self.formula.chars().nth(self.posicion).unwrap().is_alphabetic() {
                    variable.push(self.formula.chars().nth(self.posicion).unwrap());
                    self.posicion += 1;
                }
                Some(Token::Variable(variable))
            },
            '+' | '-' | '*' | '/' => {
                self.posicion += 1;
                Some(Token::Operador(caracter))
            },
            '(' => {
                self.posicion += 1;
                Some(Token::ParentesisApertura)
            },
            ')' => {
                self.posicion += 1;
                Some(Token::ParentesisCierre)
            },
            ',' => {
                self.posicion += 1;
                Some(Token::Coma)
            },
            _ => {
                self.posicion += 1;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_literal() {
        let mut lexer = Lexer::nuevo("123.45".to_string());
        assert_eq!(lexer.siguiente_token(), Some(Token::Literal(123.45)));
    }

    #[test]
    fn test_lexer_variable() {
        let mut lexer = Lexer::nuevo("salario".to_string());
        assert_eq!(lexer.siguiente_token(), Some(Token::Variable("salario".to_string())));
    }

    // más pruebas...
}
