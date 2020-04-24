
/// A value in the treewalking interpreter.
#[allow(missing_docs)]
pub enum Value {
    Integer(i64),
    Char(char),
    String(String),
    Boolean(bool)
}