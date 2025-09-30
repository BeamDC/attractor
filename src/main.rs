use crate::expr::tokenizer::Tokenizer;

mod expr;
mod io;

fn main() {
    let mut tokenizer = Tokenizer::new(
      "   123.456 + a * (b - 78) / func(x, y)  "
    );
    let tokens = tokenizer.tokenize();
    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token);
    }
}