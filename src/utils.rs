

macro_rules! replace {
    { $result: expr, $( $key:expr => $value:expr ),* } => {
        replace!( $result, $(
            $key => $value,
        )* )
    };
    { $result: expr, $( $key:expr => $value:expr, )* } => ({
        let a = $result.clone();
        $(
            a = str::replace(&a, $key, $value);
        )*
        a
    })
}

pub fn toEscape(str: String) -> String {
    let mut result = str.clone();
    result = str::replace(&result, "\\", "\\\\");
    // result = str::replace(&result, "\"",   "\\\"");
     result = str::replace(&result, "r\"", "\"");
    // result = str::replace(&result, "\"",   "\\\"");
    
    
    // println!("toEscape: {} => {}",&str, result);
    result
}