use std::iter::Peekable;
use std::vec::IntoIter;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//Program entry point
fn main() {
    let file_arg: String = env::args().nth(1).unwrap();
    let path = Path::new(&file_arg);
    let display = path.display();

    let mut in_file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       why.description()),
            Ok(in_file) => in_file
    };

    let mut s = String::new();

    in_file.read_to_string(&mut s);

    println!("Made it thru file input");
    let table = tokenizer(s);
    println!("made it thru tokenizer");
    let ast = parser(table);
    println!("made it thru parser");
    let new_ast = transformer(ast);
    println!("made it thru transformer");
    let output: String = code_gen(&new_ast);
    println!("made it thru code_gen");

    let out_path = Path::new("out.js");
    let display = out_path.display();

    let mut file = match File::create(&out_path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(output.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display)
    }

    println!("Success!");
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

//struct for holding ast nodes
struct Node {
    t: String,
    val: String,
    kids: Vec<Node>
}

fn parser(vt: Vec<Token>) ->  Node {

    fn walk(token: Token, mut token_iter: &mut Peekable<IntoIter<Token>>) -> Node {

        match token.id.as_str() {
            //number literals and operator expressions
            "number" => {

                if token_iter.peek().unwrap().val == "?".to_string() {
                    token_iter.next();
                    return Node { t: "number_literal".to_string(), val: token.val, kids: Vec::new() };
                } else {

                    // same our number for later
                    let num_node = Node { t: "number_literal".to_string(), val: token.val, kids: Vec::new() };
                    //If the number literal isn't on it's own, we know it is an operator expression
                    let mut exp_node = Node { t: "operator_expression".to_string(), val: token_iter.next().unwrap().val, kids: Vec::new() };

                    //add our current token numlit to the kids of the expression
                    exp_node.kids.push( num_node );

                    //Recursively loop to gather the rest of the kids.
                    while let Some(next_token) = token_iter.next() {
                        if next_token.id == "?".to_string() {
                            return exp_node;
                        }
                        //Skip the expression operator and then recurse on the follow token.
                        exp_node.kids.push(walk(next_token, &mut token_iter));
                    }

                    return exp_node;

                }
            },
            //declaration and assignment, name
            "have" => {

                let mut have_node = Node { t: "declaration_expression".to_string(), val: token.val, kids: Vec::new() };

                //Put the name as a kid
                let name_token = token_iter.next().unwrap();
                have_node.kids.push( Node { t: "name".to_string(), val: name_token.val, kids: Vec::new() });

                //See if there's assignment inline. if there is we need to loop
                if token_iter.peek().unwrap().val == "be".to_string() {
                    token_iter.next();
                    have_node.kids.push( Node { t: "assignment_expression".to_string(), val: "be".to_string(), kids: Vec::new() });
                    //build the rest of the expression tree for the current statement (statements end with ?)
                    while let Some(next_token) = token_iter.next() {
                        if next_token.id == "?".to_string() {
                            return have_node;
                        }
                        have_node.kids[1].kids.push(walk(next_token, &mut token_iter));
                    }
                }
                return have_node;
            },
            _ => return Node { t: "error_in_parser".to_string(), val: token.val, kids: Vec::new() }
        }

    }


    let mut ast = Node { t: "program".to_string(), val: "".to_string(), kids: Vec::new() };

    let mut tokens_iter = vt.into_iter().peekable();
    while let Some(tok) = tokens_iter.next() {
        ast.kids.push(walk(tok, &mut tokens_iter));
    }

    return ast;
}

fn traverse (node_slice: Node) -> Node {
    match node_slice.t.as_str(){
        "number_literal" | "name" => {
            return node_slice;
        },
        "declaration_expression" => {
            let mut exp_node = Node { t: "declaration_expression".to_string(), val: "var".to_string(), kids: Vec::new() };

            for kid in node_slice.kids {
                exp_node.kids.push(traverse(kid));
            }

            return exp_node;
        },
        "assignment_expression" => {
            let mut ass_node = Node { t: "assignment_expression".to_string(), val: "=".to_string(), kids: Vec::new() };

            for kid in node_slice.kids {
                ass_node.kids.push(traverse(kid));
            }

            return ass_node;
        },
        "operator_expression" => {
            let mut op_node: Node = Node { t: "error_in_transformer".to_string(), val: "".to_string(), kids: Vec::new() };
            match node_slice.val.as_str() {
                "plus" => {
                    op_node = Node { t: "operator_expression".to_string(), val: "+".to_string(), kids: Vec::new() };
                },
                "minus" => {
                    op_node = Node { t: "operator_expression".to_string(), val: "-".to_string(), kids: Vec::new() };
                },
                "times" => {
                    op_node = Node { t: "operator_expression".to_string(), val: "*".to_string(), kids: Vec::new() };
                },
                "into" => {
                    op_node = Node { t: "operator_expression".to_string(), val: "/".to_string(), kids: Vec::new() };
                }
                _ => return Node { t: "error_in_transformer".to_string(), val: node_slice.val, kids: Vec::new() }
            }
            for kid in node_slice.kids {
                op_node.kids.push(traverse(kid));
            }

            return op_node;
        },
        "end_line" => return Node { t: "end_line".to_string(), val: ";".to_string(), kids: Vec::new() },
        _ => return Node { t: "error_in_transformer".to_string(), val: node_slice.val, kids: Vec::new() }

    }
}

fn transformer (ast: Node) -> Node {
    let mut new_ast = Node { t: "program".to_string(), val: "".to_string(), kids: Vec::new() };

    //Do something for each child node
    for kid in ast.kids {
        new_ast.kids.push(traverse(kid));
    }

    return new_ast;
}


fn code_gen (new_ast: &Node) -> String {
    match new_ast.t.as_str() {
        "program" => {
            let mut s: String = "".to_string();

            for kid in &new_ast.kids {
                s.push_str(code_gen(&kid).as_str());
                s.push_str(";\n");
            }

            return s;
        },
        "declaration_expression" => {
            let mut s: String = "".to_string();

            s.push_str(new_ast.val.as_str());
            for kid in &new_ast.kids {
                s.push_str(" ");
                s.push_str(code_gen(&kid).as_str());
            }

            return s;
        },
        "assignment_expression" => {
            let mut s: String = "".to_string();

            s.push_str(new_ast.val.as_str());
            for kid in &new_ast.kids {
                s.push_str(" ");
                s.push_str(code_gen(&kid).as_str());
            }
            return s;
        },
        "operator_expression" => {
            let mut s: String = "".to_string();

            s.push_str(code_gen(&new_ast.kids[0]).as_str());
            s.push_str(" ");
            s.push_str(new_ast.val.as_str());
            s.push_str(" ");
            s.push_str(code_gen(&new_ast.kids[1]).as_str());
            return s;
        },
        "name" | "number_literal" => {
            return new_ast.val.clone();
        },
        _ => {
            return "error in code_gen, value was: ".to_string() + new_ast.t.as_str() + new_ast.val.as_str();
        }
    }
}
