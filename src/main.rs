use std::iter::Peekable;
use std::vec::IntoIter;

//Program entry point
fn main() {

    let table = tokenizer("have value be 12 plus 23?".to_string());

    for i in &table {
        println!("{}: {}", i.id, i.val);
    }

    let ast_result = parser(table);


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
            vt.push( Token { id: "?".to_string(), val: "?".to_string() });
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
                vt.push( Token { id: "plus".to_string(), val: "plus".to_string() });
                continue;
            }

            if value == "have".to_string() {
                vt.push( Token { id: "have".to_string(), val: "have".to_string() });
                continue;
            }

            if value == "be".to_string() {
                vt.push( Token { id: "be".to_string(), val: "be".to_string() });
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

    fn walk(token: Token, mut token_iter: &mut Peekable<IntoIter<Token>>) -> Node {

        //Number literal
        if token.id == "number".to_string() {

            if token_iter.peek().unwrap().val == "?".to_string() {
                return Node { t: "number_literal".to_string(), val: token.val, kids: Vec::new() };
            } else {

                // same our number for later
                let num_node = Node { t: "number_literal".to_string(), val: token.val, kids: Vec::new() };
                //If the number literal isn't on it's own, we know it is an operator expression
                let mut exp_node = Node { t: "OperatorExpression".to_string(), val: token_iter.next().unwrap().val, kids: Vec::new() };

                //add our current token numlit to the kids of the expression
                exp_node.kids.push( num_node );

                //Recursively loop to gather the rest of the kids.
                while let Some(next_token) = token_iter.next() {
                    if next_token.id == "?".to_string() {
                        break;
                    }
                    //Skip the expression operator and then recurse on the follow token.
                    exp_node.kids.push(walk(next_token, &mut token_iter));
                }

                return exp_node;

            }




        } else

        //Variable dec and assignment
        if token.id == "have".to_string() {

            let mut have_node = Node { t: "DeclarationExpression".to_string(), val: token.val, kids: Vec::new() };

            //Put the name as a kid
            let name_token = token_iter.next().unwrap();
            have_node.kids.push(walk(name_token, &mut token_iter));

            //See if there's assignment inline. if there is we need to loop
            if token_iter.peek().unwrap().val == "be".to_string() {
                //If there is, build the rest of the expression tree for the current statement (statements end with ?)
                while let Some(next_token) = token_iter.next() {
                    if next_token.id == "?".to_string() {
                        break;
                    }
                    have_node.kids[0].kids.push(walk(next_token, &mut token_iter));
                }
            }
            return have_node;
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
