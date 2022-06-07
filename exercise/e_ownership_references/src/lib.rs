
pub fn inspect(x: &String) {
    if x.ends_with("s") {
        println!("Plural");
    } else {
        println!("Singular");
    }
}

pub fn change(x: &mut String) {
    if !x.ends_with("s") {
        x.push_str("s");
    }
}

pub fn eat(x: String) -> bool {
    if x.starts_with("b") && x.contains("a") {
        return true;
    } else {
        return false;
    }
}

pub fn add(x: &i32, y: &i32) -> i32 {
    *x+*y
}
