pub fn hello(name: Option<&str>) -> String {
    let n = match name {
        Some(s) => s,
        None => "World",
    };
    format!("Hello, {}!", n)
}