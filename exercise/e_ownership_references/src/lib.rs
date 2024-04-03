pub fn inspect(str: &String) {
    //let is_plural = str.ends_with("s");
    if is_plural(&str) {
        print!("is plural: {}", str);
    } else {
        print!("is singular: {}", str);
    }
    println!();
}

pub fn change(str: &mut String) {
    //let is_plural = str.ends_with("s");
    if !is_plural(&str) {
        *str += "s";
    }
}

pub fn eat(str: String) -> bool {
    str.starts_with("b") && str.contains("a")
}

pub fn bedazzle(str: &mut String) {
    *str = "sparkly".to_string();
}

fn is_plural(str: &String) -> bool {
    str.ends_with("s")
}