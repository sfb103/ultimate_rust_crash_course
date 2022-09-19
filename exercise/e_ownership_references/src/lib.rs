pub fn inspect( arg: &String ) {
    if is_plural(arg) {
        println!("{} is plural", arg);
    } else {
        println!("{} is singular", arg);
    }
}

pub fn change( arg: &mut String ) {
    if !is_plural( arg ) {
        arg.push_str("s")
    }
}

fn is_plural( arg: &String ) -> bool {
    if arg.ends_with("s") {
        true
    } else {
        false
    }
}

pub fn eat( arg: String ) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

pub fn add( a: &i32, b: &i32 ) -> i32 {
    *a + *b
}