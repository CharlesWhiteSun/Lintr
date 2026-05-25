//! Built-in lint rules for Lintr.
//!
//! Rules are grouped into three categories:
//! - `style`    — E001 LineTooLong, E002 TrailingWhitespace, E003 MissingWhitespaceAroundOperator
//! - `bug`      — B001 MutableDefaultArgument, B002 CompareToNoneWithEq, B003 UnreachableCode
//! - `security` — S001 UseOfEval, S002 HardcodedPassword
//!
//! Phase 3 adds the rule registry; Phase 4 implements each rule.

pub mod rules {
    pub mod style {}
    pub mod bug {}
    pub mod security {}
}
