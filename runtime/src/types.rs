pub type RuntimeString = String;
pub type RuntimeI64 = i64;

#[derive(Debug)]
pub enum RuntimeValue {
    RuntimeI64(RuntimeI64),
}