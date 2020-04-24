use crate::backend::treewalk::context::EvaluationContext;

/// Trait implemented by AST nodes for evaluation.
pub trait AstEval {
    /// Evaluate an AST node.
    fn eval(&self, context: &mut EvaluationContext) -> Value {

    }
}