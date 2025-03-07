use rustpython_ast::{Alias, Expr, Located, Stmt};

use super::settings::{BannedApi, Strictness};
use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::settings::hashable::HashableHashMap;
use crate::violations;

/// TID252
pub fn banned_relative_import(
    stmt: &Stmt,
    level: Option<&usize>,
    strictness: &Strictness,
) -> Option<Diagnostic> {
    let strictness_level = match strictness {
        Strictness::All => 0,
        Strictness::Parents => 1,
    };
    if level? > &strictness_level {
        Some(Diagnostic::new(
            violations::BannedRelativeImport(strictness.clone()),
            Range::from_located(stmt),
        ))
    } else {
        None
    }
}

/// TID251
pub fn name_is_banned(
    module: &str,
    name: &Alias,
    banned_apis: &HashableHashMap<String, BannedApi>,
) -> Option<Diagnostic> {
    let full_name = format!("{module}.{}", &name.node.name);
    if let Some(ban) = banned_apis.get(&full_name) {
        return Some(Diagnostic::new(
            violations::BannedApi {
                name: full_name,
                message: ban.msg.to_string(),
            },
            Range::from_located(name),
        ));
    }
    None
}

/// TID251
pub fn name_or_parent_is_banned<T>(
    located: &Located<T>,
    name: &str,
    banned_apis: &HashableHashMap<String, BannedApi>,
) -> Option<Diagnostic> {
    let mut name = name;
    loop {
        if let Some(ban) = banned_apis.get(name) {
            return Some(Diagnostic::new(
                violations::BannedApi {
                    name: name.to_string(),
                    message: ban.msg.to_string(),
                },
                Range::from_located(located),
            ));
        }
        match name.rfind('.') {
            Some(idx) => {
                name = &name[..idx];
            }
            None => return None,
        }
    }
}

/// TID251
pub fn banned_attribute_access(checker: &mut Checker, expr: &Expr) {
    if let Some(call_path) = checker.resolve_call_path(expr) {
        for (banned_path, ban) in checker.settings.flake8_tidy_imports.banned_api.iter() {
            if call_path == banned_path.split('.').collect::<Vec<_>>() {
                checker.diagnostics.push(Diagnostic::new(
                    violations::BannedApi {
                        name: banned_path.to_string(),
                        message: ban.msg.to_string(),
                    },
                    Range::from_located(expr),
                ));
                return;
            }
        }
    }
}
