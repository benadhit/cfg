#[derive(Debug)]
pub struct CfgError {
    line : usize,
    message: String,

}

impl CfgError {

    pub fn error(line :usize, message: String) -> CfgError {
        CfgError{line, message}
    }

    pub fn report(&self, loc:String) {
        eprintln!("[line {}],Error{}:{}",line, loc, message);
    }
}