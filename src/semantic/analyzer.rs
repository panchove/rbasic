use crate::parser::ast::{BinaryOp, Expression, Literal, Program, Statement, UnaryOp};
use crate::semantic::errors::{SemanticError, SemanticErrorCode};
use crate::semantic::types::Type;
use std::collections::HashMap;

/// Returns true if `actual` can be assigned to `expected` (implicit widening allowed).
fn types_compatible(expected: &Type, actual: &Type) -> bool {
    if expected == actual {
        return true;
    }
    // Same-family integer handling
    let rank = |t: &Type| -> i8 {
        match t {
            Type::I8 | Type::U8 => 0,
            Type::I16 | Type::U16 => 1,
            Type::I32 | Type::U32 => 2,
            Type::I64 | Type::U64 => 3,
            _ => -1,
        }
    };
    let (er, ar) = (rank(expected), rank(actual));
    if er >= 0 && ar >= 0 {
        let same_family = expected.is_signed() == actual.is_signed();
        if same_family {
            // Allow widening (e.g., I32 -> I64)
            if ar <= er {
                return true;
            }
        }
        // Allow I32 literal to any integer type (signed or unsigned)
        if *actual == Type::I32 {
            return true;
        }
    }
    // Existing logic ...
    // Add string concatenation compatibility
    if *expected == Type::String && *actual == Type::String {
        return true;
    }
    if *expected == Type::F64 && *actual == Type::F32 {
        return true;
    }
    if *expected == Type::F32 && *actual == Type::F64 {
        return true;
    }
    false
}

/// Returns true if an explicit `AS` cast from `from` to `to` is valid.
fn can_cast_explicitly(from: &Type, to: &Type) -> bool {
    from.is_numeric() && to.is_numeric()
}

struct FuncSig {
    param_types: Vec<Type>,
    ret_type: Option<Type>,
}

pub fn analyze(prog: &Program) -> Result<(), Vec<SemanticError>> {
    let mut errors: Vec<SemanticError> = Vec::new();

    // Collect function signatures
    let mut functions: HashMap<String, FuncSig> = HashMap::new();
    // Track global variable types and whether they're declared
    let mut globals: HashMap<String, Type> = HashMap::new();

    // Helper: push error
    fn err(errors: &mut Vec<SemanticError>, code: SemanticErrorCode, msg: String) {
        errors.push(SemanticError {
            code,
            message: msg,
            span: None,
        });
    }

    // Pass 1: collect declarations
    for stmt in &prog.statements {
        match stmt {
            Statement::VarDecl { name, typ, .. } => {
                if let Some(t) = typ {
                    if Type::from_name(&t.name).is_none() {
                        err(
                            &mut errors,
                            SemanticErrorCode::E1010,
                            format!("Unknown type {}", t.name),
                        );
                    }
                }
                let key = name.to_lowercase();
                if let std::collections::hash_map::Entry::Vacant(e) = globals.entry(key) {
                    let resolved = typ
                        .as_ref()
                        .and_then(|t| Type::from_name(&t.name))
                        .unwrap_or(Type::I32);
                    e.insert(resolved);
                } else {
                    err(
                        &mut errors,
                        SemanticErrorCode::E1002,
                        format!("Duplicate global variable: {}", name),
                    );
                }
            }
            Statement::FunctionDecl {
                name,
                params,
                ret_type,
                ..
            } => {
                let key = name.to_lowercase();
                if let std::collections::hash_map::Entry::Vacant(e) = functions.entry(key) {
                    let mut param_types = Vec::new();
                    let mut param_names = std::collections::HashSet::new();
                    for p in params {
                        match Type::from_name(&p.typ.name) {
                            Some(t) => param_types.push(t),
                            None => err(
                                &mut errors,
                                SemanticErrorCode::E1010,
                                format!("Unknown type {} in parameter {}", p.typ.name, p.name),
                            ),
                        }
                        if !param_names.insert(p.name.to_lowercase()) {
                            err(
                                &mut errors,
                                SemanticErrorCode::E1011,
                                format!("Duplicate parameter {} in function {}", p.name, name),
                            );
                        }
                    }
                    let ret = ret_type.as_ref().and_then(|t| {
                        if Type::from_name(&t.name).is_none() {
                            err(
                                &mut errors,
                                SemanticErrorCode::E1010,
                                format!("Unknown return type {}", t.name),
                            );
                            None
                        } else {
                            Type::from_name(&t.name)
                        }
                    });
                    e.insert(FuncSig {
                        param_types,
                        ret_type: ret,
                    });
                } else {
                    err(
                        &mut errors,
                        SemanticErrorCode::E1004,
                        format!("Duplicate function: {}", name),
                    );
                }
            }
            _ => {}
        }
    }

    // Resolve expression type
    fn resolve_expr(
        expr: &Expression,
        locals: &HashMap<String, Type>,
        globals: &HashMap<String, Type>,
        functions: &HashMap<String, FuncSig>,
        errors: &mut Vec<SemanticError>,
    ) -> Option<Type> {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Int(_) => Some(Type::I32),
                Literal::Float(_) => Some(Type::F64),
                Literal::Bool(_) => Some(Type::Bool),
                Literal::String(_) => Some(Type::String),
            },
            Expression::Identifier(name) => {
                let key = name.to_lowercase();
                locals
                    .get(&key)
                    .or_else(|| globals.get(&key))
                    .cloned()
                    .or_else(|| {
                        err(
                            errors,
                            SemanticErrorCode::E1001,
                            format!("Unknown variable {}", name),
                        );
                        None
                    })
            }
            Expression::Unary { op, expr } => {
                let inner = resolve_expr(expr, locals, globals, functions, errors);
                match (op, &inner) {
                    (UnaryOp::Neg, Some(Type::I8)) => Some(Type::I8),
                    (UnaryOp::Neg, Some(Type::I16)) => Some(Type::I16),
                    (UnaryOp::Neg, Some(Type::I32)) => Some(Type::I32),
                    (UnaryOp::Neg, Some(Type::I64)) => Some(Type::I64),
                    (UnaryOp::Neg, Some(Type::F32)) => Some(Type::F32),
                    (UnaryOp::Neg, Some(Type::F64)) => Some(Type::F64),
                    (UnaryOp::Not, Some(Type::Bool)) => Some(Type::Bool),
                    (_, None) => None,
                    _ => {
                        let op_str = match op {
                            UnaryOp::Neg => "-",
                            UnaryOp::Not => "NOT",
                        };
                        let type_str = inner.as_ref().map(|t| t.to_rust_str()).unwrap_or("unknown");
                        err(
                            errors,
                            SemanticErrorCode::E1022,
                            format!("Invalid unary operation '{}' on type {}", op_str, type_str),
                        );
                        None
                    }
                }
            }
            Expression::Binary { left, op, right } => {
                let l = resolve_expr(left, locals, globals, functions, errors);
                let r = resolve_expr(right, locals, globals, functions, errors);
                // Integer-integer operations use automatic widening
                'typecheck: {
                    if let (Some(ref lt), Some(ref rt)) = (&l, &r) {
                        if lt.is_integer() && rt.is_integer() && lt.is_signed() == rt.is_signed() {
                            break 'typecheck Some(match op {
                                BinaryOp::Add
                                | BinaryOp::Sub
                                | BinaryOp::Mul
                                | BinaryOp::Div
                                | BinaryOp::IntDiv
                                | BinaryOp::Mod => Type::widen_int(lt, rt),
                                BinaryOp::Pow => Type::F64,
                                BinaryOp::Eq
                                | BinaryOp::NotEq
                                | BinaryOp::Lt
                                | BinaryOp::Lte
                                | BinaryOp::Gt
                                | BinaryOp::Gte => Type::Bool,
                            });
                        }
                        // Logical operations on BOOL
                        if *lt == Type::Bool && *rt == Type::Bool {
                            break 'typecheck Some(match op {
                                BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => Type::Bool,
                                _ => Type::Bool,
                            });
                        }
                            match op {
                                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                                    let result = if *lt == Type::F64 || *rt == Type::F64 {
                                        Type::F64
                                    } else {
                                        Type::F32
                                    };
                                    break 'typecheck Some(result);
                                }
                                BinaryOp::Pow => break 'typecheck Some(Type::F64),
                                BinaryOp::Eq
                                | BinaryOp::NotEq
                                | BinaryOp::Lt
                                | BinaryOp::Lte
                                | BinaryOp::Gt
                                | BinaryOp::Gte => break 'typecheck Some(Type::Bool),
                                _ => {} // IntDiv, Mod — fall through to error
                            }
                        }
                    }
                    match (op, &l, &r) {
                        // String concatenation
                        (BinaryOp::Add, Some(Type::String), Some(Type::String)) => Some(Type::String),
                        // Logical AND/OR/XOR on BOOL
                        (BinaryOp::And, Some(Type::Bool), Some(Type::Bool)) => Some(Type::Bool),
                        (BinaryOp::Or, Some(Type::Bool), Some(Type::Bool)) => Some(Type::Bool),
                        (BinaryOp::Xor, Some(Type::Bool), Some(Type::Bool)) => Some(Type::Bool),
                        (BinaryOp::NotEq, Some(Type::String), Some(Type::String)) => {
                            Some(Type::Bool)
                        }
                        (BinaryOp::Eq, Some(Type::Bool), Some(Type::Bool)) => Some(Type::Bool),
                        (BinaryOp::NotEq, Some(Type::Bool), Some(Type::Bool)) => Some(Type::Bool),
                        // Invalid
                        (_, None, _) | (_, _, None) => None,
                        _ => {
                            let op_str = match op {
                                BinaryOp::Add => "+",
                                BinaryOp::Sub => "-",
                                BinaryOp::Mul => "*",
                                BinaryOp::Div => "/",
                                BinaryOp::Pow => "^",
                                BinaryOp::IntDiv => "\\",
                                BinaryOp::Mod => "MOD",
                                BinaryOp::Eq => "==",
                                BinaryOp::NotEq => "!=",
                                BinaryOp::Lt => "<",
                                BinaryOp::Lte => "<=",
                                BinaryOp::Gt => ">",
                                BinaryOp::And => "AND",
                            BinaryOp::Or => "OR",
                            BinaryOp::Xor => "XOR",
                            };
                            let l_str = l.as_ref().map(|t| t.to_rust_str()).unwrap_or("?");
                            let r_str = r.as_ref().map(|t| t.to_rust_str()).unwrap_or("?");
                            err(
                                errors,
                                SemanticErrorCode::E1021,
                                format!(
                                    "Invalid binary operation '{}' between {} and {}",
                                    op_str, l_str, r_str
                                ),
                            );
                            None
                        }
                    }
                }
            }
            Expression::Grouping(inner) => resolve_expr(inner, locals, globals, functions, errors),
            Expression::Cast { expr, target_type } => {
                let inner =
                    resolve_expr(expr, locals, globals, functions, errors).unwrap_or(Type::I32);
                let target = Type::from_name(target_type);
                match target {
                    Some(t) => {
                        if inner != t && !can_cast_explicitly(&inner, &t) {
                            err(
                                errors,
                                SemanticErrorCode::E1020,
                                format!(
                                    "Cannot cast {} to {}",
                                    inner.to_rust_str(),
                                    t.to_rust_str(),
                                ),
                            );
                        }
                        Some(t)
                    }
                    None => {
                        err(
                            errors,
                            SemanticErrorCode::E1010,
                            format!("Unknown type {}", target_type),
                        );
                        None
                    }
                }
            }
            Expression::Call { callee, args } => {
                let key = callee.to_lowercase();
                if let Some(sig) = functions.get(&key) {
                    // Check argument count
                    if args.len() != sig.param_types.len() {
                        err(
                            errors,
                            SemanticErrorCode::E1030,
                            format!(
                                "Function {} expects {} arguments, got {}",
                                callee,
                                sig.param_types.len(),
                                args.len()
                            ),
                        );
                    }
                    // Check argument types
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = resolve_expr(arg, locals, globals, functions, errors);
                        if i < sig.param_types.len() {
                            if let Some(actual) = &arg_type {
                                if !types_compatible(&sig.param_types[i], actual) {
                                    err(
                                        errors,
                                        SemanticErrorCode::E1020,
                                        format!(
                                            "Argument {} of {} expected type {}, got {}",
                                            i + 1,
                                            callee,
                                            sig.param_types[i].to_rust_str(),
                                            actual.to_rust_str(),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                    sig.ret_type.clone()
                } else {
                    err(
                        errors,
                        SemanticErrorCode::E1003,
                        format!("Unknown function {}", callee),
                    );
                    None
                }
            }
        }
    }

    // Walk statements, tracking locals
    fn walk_stmt(
        stmt: &Statement,
        locals: &mut HashMap<String, Type>,
        globals: &HashMap<String, Type>,
        functions: &HashMap<String, FuncSig>,
        errors: &mut Vec<SemanticError>,
        inside_function: bool,
        current_ret_type: Option<&Type>,
    ) {
        match stmt {
            Statement::VarDecl { name, typ, init } => {
                let init_type = resolve_expr(init, locals, globals, functions, errors);
                let declared = typ.as_ref().and_then(|t| Type::from_name(&t.name));
                // Type mismatch check
                if let (Some(d), Some(a)) = (&declared, &init_type) {
                    if !types_compatible(d, a) {
                        err(
                            errors,
                            SemanticErrorCode::E1020,
                            format!(
                                "Type mismatch: variable {} declared as {} but initializer is {}",
                                name,
                                d.to_rust_str(),
                                a.to_rust_str(),
                            ),
                        );
                    }
                }
                let resolved = declared.or(init_type).unwrap_or(Type::I32);
                let key = name.to_lowercase();
                if let std::collections::hash_map::Entry::Vacant(e) = locals.entry(key) {
                    e.insert(resolved);
                } else {
                    err(
                        errors,
                        SemanticErrorCode::E1002,
                        format!("Duplicate local variable {}", name),
                    );
                }
            }
            Statement::Print { expr } => {
                resolve_expr(expr, locals, globals, functions, errors);
            }
            Statement::ExpressionStmt { expr } => {
                resolve_expr(expr, locals, globals, functions, errors);
            }
            Statement::Return { expr } => {
                if !inside_function {
                    err(
                        errors,
                        SemanticErrorCode::E1033,
                        "Return outside function body".to_string(),
                    );
                    return;
                }
                if let Some(expected) = current_ret_type {
                    if let Some(expr) = expr {
                        let actual = resolve_expr(expr, locals, globals, functions, errors);
                        if let Some(a) = &actual {
                            if !types_compatible(expected, a) {
                                err(
                                    errors,
                                    SemanticErrorCode::E1031,
                                    format!(
                                        "Return type mismatch: expected {}, got {}",
                                        expected.to_rust_str(),
                                        a.to_rust_str(),
                                    ),
                                );
                            }
                        }
                    } else {
                        err(
                            errors,
                            SemanticErrorCode::E1031,
                            format!(
                                "Return type mismatch: expected {}, got nothing",
                                expected.to_rust_str(),
                            ),
                        );
                    }
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_type = resolve_expr(condition, locals, globals, functions, errors);
                if let Some(t) = &cond_type {
                    if t != &Type::Bool {
                        err(
                            errors,
                            SemanticErrorCode::E1032,
                            format!("If condition must be BOOL, got {}", t.to_rust_str(),),
                        );
                    }
                }
                for s in then_branch {
                    walk_stmt(
                        s,
                        &mut locals.clone(),
                        globals,
                        functions,
                        errors,
                        inside_function,
                        current_ret_type,
                    );
                }
                if let Some(else_branch) = else_branch {
                    for s in else_branch {
                        walk_stmt(
                            s,
                            &mut locals.clone(),
                            globals,
                            functions,
                            errors,
                            inside_function,
                            current_ret_type,
                        );
                    }
                }
            }
            Statement::While { condition, body } => {
                let cond_type = resolve_expr(condition, locals, globals, functions, errors);
                if let Some(t) = &cond_type {
                    if t != &Type::Bool {
                        err(
                            errors,
                            SemanticErrorCode::E1032,
                            format!("While condition must be BOOL, got {}", t.to_rust_str(),),
                        );
                    }
                }
                for s in body {
                    walk_stmt(
                        s,
                        &mut locals.clone(),
                        globals,
                        functions,
                        errors,
                        inside_function,
                        current_ret_type,
                    );
                }
            }
            Statement::For {
                var,
                start,
                end,
                step,
                body,
            } => {
                let start_type = resolve_expr(start, locals, globals, functions, errors);
                let end_type = resolve_expr(end, locals, globals, functions, errors);
                if let (Some(s), Some(e)) = (&start_type, &end_type) {
                    if s != e && !types_compatible(s, e) && !types_compatible(e, s) {
                        err(
                            errors,
                            SemanticErrorCode::E1020,
                            format!(
                                "For loop bounds types mismatch: start is {}, end is {}",
                                s.to_rust_str(),
                                e.to_rust_str(),
                            ),
                        );
                    }
                }
                if let Some(step_expr) = step {
                    let step_type = resolve_expr(step_expr, locals, globals, functions, errors);
                    if let Some(t) = &step_type {
                        if !t.is_numeric() {
                            err(
                                errors,
                                SemanticErrorCode::E1034,
                                format!("Step value must be numeric, got {}", t.to_rust_str(),),
                            );
                        }
                    }
                    if let Some(s) = &step_type {
                        if let Some(st) = &start_type {
                            if s != st && !types_compatible(s, st) && !types_compatible(st, s) {
                                err(
                                    errors,
                                    SemanticErrorCode::E1020,
                                    format!(
                                        "Step type {} does not match loop bounds type {}",
                                        s.to_rust_str(),
                                        st.to_rust_str(),
                                    ),
                                );
                            }
                        }
                    }
                }
                let mut for_locals = locals.clone();
                let loop_var_type = start_type.or(end_type).unwrap_or(Type::I32);
                for_locals.insert(var.to_lowercase(), loop_var_type);
                for s in body {
                    walk_stmt(
                        s,
                        &mut for_locals,
                        globals,
                        functions,
                        errors,
                        inside_function,
                        current_ret_type,
                    );
                }
            }
            Statement::DoLoop {
                variant: _,
                condition,
                body,
            } => {
                if let Some(cond) = condition {
                    let cond_type = resolve_expr(cond, locals, globals, functions, errors);
                    if let Some(t) = &cond_type {
                        if t != &Type::Bool {
                            err(
                                errors,
                                SemanticErrorCode::E1032,
                                format!("DO loop condition must be BOOL, got {}", t.to_rust_str(),),
                            );
                        }
                    }
                }
                for s in body {
                    walk_stmt(
                        s,
                        &mut locals.clone(),
                        globals,
                        functions,
                        errors,
                        inside_function,
                        current_ret_type,
                    );
                }
            }
            Statement::FunctionDecl { params, body, .. } => {
                let mut func_locals: HashMap<String, Type> = HashMap::new();
                for p in params {
                    if let Some(t) = Type::from_name(&p.typ.name) {
                        func_locals.insert(p.name.to_lowercase(), t);
                    }
                }
                let ret_type = functions
                    .get(&func_name(stmt))
                    .and_then(|s| s.ret_type.clone());
                for s in body {
                    walk_stmt(
                        s,
                        &mut func_locals,
                        globals,
                        functions,
                        errors,
                        true,
                        ret_type.as_ref(),
                    );
                }
            }
        }
    }

    fn func_name(stmt: &Statement) -> String {
        if let Statement::FunctionDecl { name, .. } = stmt {
            name.to_lowercase()
        } else {
            String::new()
        }
    }

    // Walk top-level statements
    for stmt in &prog.statements {
        walk_stmt(
            stmt,
            &mut HashMap::new(),
            &globals,
            &functions,
            &mut errors,
            false,
            None,
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
