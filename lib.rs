// &'static is a "lifetime specifier", something you'll learn more about later
//

struct Program;

impl Program {
    pub fn hello() -> &'static str {
        "Hello, World!"
    }
}

#[cfg(test)]
mod tests {
    use crate::Program;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, World!", Program::hello());
    }
}
