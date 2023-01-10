#[macro_export]
macro_rules! log_err {
    ($result: expr) => {
        if let Err(err) = $result {
            println!("err: {}", err);
        }
    };

    ($result: expr, $err_str: expr) => {
        if let Err(_) = $result {
            println!("err: {}", $err_str);
        }
    };
}
