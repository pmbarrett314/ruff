use rustpython_ast::{Expr, Stmt};

use super::types::DebuggerUsingType;
use crate::ast::helpers::format_call_path;
use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violations;

const DEBUGGERS: &[&[&str]] = &[
    &["pdb", "set_trace"],
    &["pudb", "set_trace"],
    &["ipdb", "set_trace"],
    &["ipdb", "sset_trace"],
    &["IPython", "terminal", "embed", "InteractiveShellEmbed"],
    &[
        "IPython",
        "frontend",
        "terminal",
        "embed",
        "InteractiveShellEmbed",
    ],
    &["celery", "contrib", "rdb", "set_trace"],
    &["builtins", "breakpoint"],
    &["", "breakpoint"],
];

/// Checks for the presence of a debugger call.
pub fn debugger_call(checker: &mut Checker, expr: &Expr, func: &Expr) {
    if let Some(call_path) = checker.resolve_call_path(func) {
        if DEBUGGERS.iter().any(|target| call_path == *target) {
            checker.diagnostics.push(Diagnostic::new(
                violations::Debugger(DebuggerUsingType::Call(format_call_path(&call_path))),
                Range::from_located(expr),
            ));
        }
    }
}

/// Checks for the presence of a debugger import.
pub fn debugger_import(stmt: &Stmt, module: Option<&str>, name: &str) -> Option<Diagnostic> {
    // Special-case: allow `import builtins`, which is far more general than (e.g.)
    // `import celery.contrib.rdb`).
    if module.is_none() && name == "builtins" {
        return None;
    }

    if let Some(module) = module {
        let mut call_path = module.split('.').collect::<Vec<_>>();
        call_path.push(name);
        if DEBUGGERS.iter().any(|target| call_path == **target) {
            return Some(Diagnostic::new(
                violations::Debugger(DebuggerUsingType::Import(format_call_path(&call_path))),
                Range::from_located(stmt),
            ));
        }
    } else {
        let parts = name.split('.').collect::<Vec<_>>();
        if DEBUGGERS
            .iter()
            .any(|call_path| call_path[..call_path.len() - 1] == parts)
        {
            return Some(Diagnostic::new(
                violations::Debugger(DebuggerUsingType::Import(name.to_string())),
                Range::from_located(stmt),
            ));
        }
    }
    None
}
