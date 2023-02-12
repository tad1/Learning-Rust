
fn stringify_error(err : String) -> String {
    println!("Error: {}", err);
    format!("Error: {}", err)
}

enum OperationError {
    DivisionByZero,
    InvalidToken(String),
    ParseError(String),
    Other(String)
}

fn handle_error(err : OperationError) -> String{
    match err {
        OperationError::DivisionByZero => {
            format!("Error: Division by Zero")
        },
        OperationError::InvalidToken(err) => {
            format!("Found invalid token '{}'", err)
        },
        OperationError::ParseError(err) => {
            format!("Found invalid syntax: {}", err)
        },
        OperationError::Other(err) => {
            format!("Error: {}", err)
        },
    }
}

fn main(){
    // It's a trival example
    let res: Result<(), String> = Result::Err(String::from("Some kind of error!"));
    res.map_err(stringify_error);

    let res : Result<(), OperationError> = Result::Err(OperationError::InvalidToken(String::from("@")));
    if let Err(str) = res.map_err(handle_error) {
        println!("{}", str);
    }

    // I found this:
    // # Rust-Error-Cheatsheet
    // https://gist.github.com/e-t-u/70f25d4566468adc43a4df43667cedb6
    


}