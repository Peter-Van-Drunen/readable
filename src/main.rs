use std::iter::Peekable;
use std::vec::IntoIter;

//Program entry point
fn main() {

    let table = tokenizer("have value be 12 plus 23?".to_string());

    for i in &table {
        println!("{}: {}", i.id, i.val);
    }
}

//struct for holding tokens
struct Token {
    id: String,
    val: String
}

fn tokenizer(input: String) -> Vec<Token> {

    let mut current = 0;

    let vc: Vec<char> = input.chars().collect();

    let mut vt: Vec<Token> = Vec::new();

    while current < input.len() {

        let mut the_char: char = vc[current];

        if the_char.is_whitespace() {
            current += 1;
            continue;
        }

        if the_char == '?' {
            vt.push( Token { id: "?".to_string(), val: ";".to_string() });
            current += 1;
            continue;
        }

        if the_char.is_alphabetic() {

            //build the full word
            let mut value: String = "".to_string();
            while the_char.is_alphabetic() {
                value = value + &the_char.to_string();
                current += 1;
                the_char = vc[current];
            }

            if value == "plus".to_string() {
                vt.push( Token { id: "plus".to_string(), val: "+".to_string() });
                continue;
            }

            if value == "have".to_string() {
                vt.push( Token { id: "have".to_string(), val: "var".to_string() });
                continue;
            }

            if value == "be".to_string() {
                vt.push( Token { id: "be".to_string(), val: "=".to_string() });
                continue;
            }

            //Keep this at the bottom. It's the "oh this is a name" failsafe after keywords are checked.
            vt.push( Token { id: "name".to_string(), val: value });
            continue;
        }

        if the_char.is_numeric() {
            let mut value: String = "".to_string();

            while the_char.is_numeric() {
                value = value + &the_char.to_string();
                current += 1;
                the_char = vc[current];
            }

            vt.push( Token { id: "number".to_string(), val: value });
            continue;
        }

    }

    return vt;
}

//struct for holding tokens
struct Node {
    t: String,
    val: String,
    kids: Vec<Node>
}



fn parser(vt: Vec<Token>) ->  Node {

    fn walk(token: Token, token_iter: &mut Peekable<IntoIter<Token>>) -> Node {

        //If the token is a number, return a number literal node with the value of the token
        if token.id == "number".to_string() {
            return Node { t: "number_literal".to_string(), val: token.val, kids: Vec::new() };
        }

        if token.id == "".to_string() {

        }

        //If no matches are found return error_in_parser
        return Node { t: "error_in_parser".to_string(), val: "invalid token".to_string(), kids: Vec::new() };
    }


    let mut ast = Node { t: "program".to_string(), val: "".to_string(), kids: Vec::new() };

    let mut tokens_iter = vt.into_iter().peekable();
    while let Some(tok) = tokens_iter.next() {
        ast.kids.push(walk(tok, &mut tokens_iter));
    }

    return ast;
}
