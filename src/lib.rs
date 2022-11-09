pub mod lexer;

#[allow(dead_code)]
pub mod loxr_ {
    pub struct Loxr {
        pub had_error: bool,
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    impl Loxr {
        pub fn new() -> Loxr{
            return Loxr{
                had_error:false
            }
        }
        fn run_file(path: String) {}
        fn run_prompt() {}
        fn run() {} //runs single line of code

        pub fn error(&mut self, line: usize, message: String) {
            self.report(line, "", message)
        }

        fn report(&mut self, line: usize, pos: &str, message: String) {
            println!("[Line {}] Error {}: {}", line, pos, message);
            self.had_error = true;
        }
    }
}
