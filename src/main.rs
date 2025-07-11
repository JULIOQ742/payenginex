mod lexer; // Importamos el módulo lexer

use lexer::{Lexer, Token}; // Importamos las estructuras y enums necesarias

fn main() {
    let formula = "salario * 0.15 + bonificacion"; // Fórmula de prueba
    let mut lexer = Lexer::nuevo(formula.to_string()); // Creamos una instancia del lexer

    // Obtenemos tokens uno por uno
    while let Some(token) = lexer.siguiente_token() {
        println!("{:?}", token); // Imprimimos cada token
    }
}
