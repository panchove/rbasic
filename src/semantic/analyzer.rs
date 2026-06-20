use crate::parser::ast::{
    BinaryOp, CaseValue, CompoundAssignOp, Expression, FileMode, Literal, PrintItem, Program,
    Statement, UnaryOp,
};
use crate::semantic::errors::{SemanticError, SemanticErrorCode};
use crate::semantic::types::Type;
use std::collections::HashMap;

/// Information about a declared array: (base_type, dimension_count).
pub type ArrayInfo = HashMap<String, (Type, usize)>;

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
    // Float widening
    if *expected == Type::F64 && *actual == Type::F32 {
        return true;
    }
    if *expected == Type::F32 && *actual == Type::F64 {
        return true;
    }
    // Integer to float widening
    if *expected == Type::F64 && actual.is_integer() {
        return true;
    }
    if *expected == Type::F32 && actual.is_integer() {
        return true;
    }
    // String concatenation compatibility
    if *expected == Type::String && *actual == Type::String {
        return true;
    }
    false
}

/// Checks if a binary operation between two types is valid.
fn binary_op_valid(op: &BinaryOp, left: &Type, right: &Type) -> bool {
    match op {
        BinaryOp::Add
        | BinaryOp::Sub
        | BinaryOp::Mul
        | BinaryOp::Div
        | BinaryOp::IntDiv
        | BinaryOp::Mod => {
            // Integer operations
            if left.is_integer() && right.is_integer() {
                if left.is_signed() != right.is_signed() {
                    return false; // E1020: signed/unsigned mismatch
                }
                return true;
            }
            // Float operations
            if left.is_numeric() && right.is_numeric() {
                if left.is_integer() != right.is_integer() {
                    return false; // E1021: cross-family
                }
                return true;
            }
            // String concatenation
            if left == &Type::String && right == &Type::String {
                return true;
            }
            // Boolean logic
            if left == &Type::Bool && right == &Type::Bool {
                return true;
            }
            false
        }
        BinaryOp::Pow => {
            // Power operation requires numeric types
            if left.is_numeric() && right.is_numeric() {
                return true;
            }
            false
        }
        BinaryOp::Eq
        | BinaryOp::NotEq
        | BinaryOp::Lt
        | BinaryOp::Lte
        | BinaryOp::Gt
        | BinaryOp::Gte => {
            // Comparison operations require compatible types
            if left.is_numeric() && right.is_numeric() {
                return true;
            }
            if left == &Type::String && right == &Type::String {
                return true;
            }
            if left == &Type::Bool && right == &Type::Bool {
                return true;
            }
            false
        }
        BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => {
            // Logical operations require boolean types
            if left == &Type::Bool && right == &Type::Bool {
                return true;
            }
            false
        }
        BinaryOp::Shl | BinaryOp::Shr => {
            // Bit shift operations require integer types
            if left.is_integer() && right.is_integer() {
                return true;
            }
            false
        }
    }
}

/// Checks if an explicit AS cast between two types is valid.
fn can_cast_explicitly(from: &Type, to: &Type) -> bool {
    from.is_numeric() && to.is_numeric()
}

/// Checks if a unary operation on a type is valid.
fn unary_op_valid(op: &UnaryOp, operand: &Type) -> bool {
    match op {
        UnaryOp::Neg => {
            // Negative operation: only signed integers and floats
            operand.is_numeric() && !operand.is_unsigned()
        }
        UnaryOp::Not => {
            // Logical NOT requires boolean type
            operand == &Type::Bool
        }
    }
}

struct FuncSig {
    param_types: Vec<Type>,
    ret_type: Option<Type>,
}

pub fn analyze(prog: &Program) -> Result<ArrayInfo, Vec<SemanticError>> {
    let mut errors: Vec<SemanticError> = Vec::new();

    // Collect function signatures
    let mut functions: HashMap<String, FuncSig> = HashMap::new();
    // Track global variable types and whether they're declared
    let mut globals: HashMap<String, Type> = HashMap::new();
    // Track declared arrays: name -> (base_type, dimension_count)
    let mut arrays: ArrayInfo = HashMap::new();
    // ON ERROR / RESUME labels are tracked at runtime; no semantic validation in v0.1.

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
    #[allow(clippy::too_many_arguments)]
    fn check_function_call(
        callee: &str,
        args: &[Expression],
        param_types: &[Type],
        ret_type: Option<Type>,
        locals: &HashMap<String, (Type, bool)>,
        globals: &HashMap<String, Type>,
        functions: &HashMap<String, FuncSig>,
        arrays: &ArrayInfo,
        errors: &mut Vec<SemanticError>,
    ) -> Option<Type> {
        if args.len() != param_types.len() {
            err(
                errors,
                SemanticErrorCode::E1030,
                format!(
                    "Function {} expects {} arguments, got {}",
                    callee,
                    param_types.len(),
                    args.len()
                ),
            );
        }
        for (i, arg) in args.iter().enumerate() {
            let arg_type = resolve_expr(arg, locals, globals, functions, arrays, errors);
            if i < param_types.len() {
                if let Some(actual) = &arg_type {
                    if !types_compatible(&param_types[i], actual) {
                        err(
                            errors,
                            SemanticErrorCode::E1020,
                            format!(
                                "Argument {} of {} expected type {}, got {}",
                                i + 1,
                                callee,
                                param_types[i].to_rust_str(),
                                actual.to_rust_str(),
                            ),
                        );
                    }
                }
            }
        }
        ret_type
    }

    /// Check if the callee is a known built-in function and validate its arguments.
    /// Returns the return type if valid, or None if not a built-in.
    fn check_builtin_call(
        callee: &str,
        args: &[Expression],
        locals: &HashMap<String, (Type, bool)>,
        globals: &HashMap<String, Type>,
        functions: &HashMap<String, FuncSig>,
        arrays: &ArrayInfo,
        errors: &mut Vec<SemanticError>,
    ) -> Option<Type> {
        let key = callee.to_lowercase();
        // INSTR has two overloads (2-arg and 3-arg)
        if key == "instr" {
            return match args.len() {
                2 => {
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type =
                            resolve_expr(arg, locals, globals, functions, arrays, errors);
                        if let Some(actual) = &arg_type {
                            if !types_compatible(&Type::String, actual) {
                                err(
                                    errors,
                                    SemanticErrorCode::E1020,
                                    format!(
                                        "Argument {} of INSTR expected type STRING, got {}",
                                        i + 1,
                                        actual.to_rust_str(),
                                    ),
                                );
                            }
                        }
                    }
                    Some(Type::I32)
                }
                3 => {
                    // INSTR(start, s, search) → (I32, STRING, STRING) → I32
                    let param_types = [Type::I32, Type::String, Type::String];
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type =
                            resolve_expr(arg, locals, globals, functions, arrays, errors);
                        if i < param_types.len() {
                            if let Some(actual) = &arg_type {
                                if !types_compatible(&param_types[i], actual) {
                                    err(
                                        errors,
                                        SemanticErrorCode::E1020,
                                        format!(
                                            "Argument {} of INSTR expected type {}, got {}",
                                            i + 1,
                                            param_types[i].to_rust_str(),
                                            actual.to_rust_str(),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                    Some(Type::I32)
                }
                _ => {
                    err(
                        errors,
                        SemanticErrorCode::E1030,
                        format!("INSTR expects 2 or 3 arguments, got {}", args.len()),
                    );
                    Some(Type::I32)
                }
            };
        }
        // All other built-in functions have fixed signatures
        let sig = builtin_sig(&key)?;
        check_function_call(
            callee,
            args,
            &sig.0,
            Some(sig.1.clone()),
            locals,
            globals,
            functions,
            arrays,
            errors,
        )
    }

    /// Return the parameter types and return type for a known built-in function.
    fn builtin_sig(key: &str) -> Option<(Vec<Type>, Type)> {
        match key {
            "len" => Some((vec![Type::String], Type::I32)),
            "mid$" | "mid" => Some((vec![Type::String, Type::I32, Type::I32], Type::String)),
            "left$" | "left" => Some((vec![Type::String, Type::I32], Type::String)),
            "right$" | "right" => Some((vec![Type::String, Type::I32], Type::String)),
            "chr$" | "chr" => Some((vec![Type::I32], Type::String)),
            "asc" => Some((vec![Type::String], Type::I32)),
            "val" => Some((vec![Type::String], Type::F64)),
            "str$" | "str" => Some((vec![Type::F64], Type::String)),
            "ucase$" | "ucase" => Some((vec![Type::String], Type::String)),
            "lcase$" | "lcase" => Some((vec![Type::String], Type::String)),
            "trim$" | "trim" => Some((vec![Type::String], Type::String)),
            "ltrim$" | "ltrim" => Some((vec![Type::String], Type::String)),
            "rtrim$" | "rtrim" => Some((vec![Type::String], Type::String)),
            "space$" | "space" => Some((vec![Type::I32], Type::String)),
            "string$" | "string" => Some((vec![Type::I32, Type::String], Type::String)),
            _ => None,
        }
    }

    fn resolve_expr(
        expr: &Expression,
        locals: &HashMap<String, (Type, bool)>,
        globals: &HashMap<String, Type>,
        functions: &HashMap<String, FuncSig>,
        arrays: &ArrayInfo,
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
                    .map(|(t, _)| t)
                    .cloned()
                    .or_else(|| globals.get(&key).cloned())
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
                let inner = resolve_expr(expr, locals, globals, functions, arrays, errors);
                match (op, &inner) {
                    (_, None) => None,
                    (op, Some(typ)) => {
                        if unary_op_valid(op, typ) {
                            match op {
                                UnaryOp::Neg => Some(typ.clone()),
                                UnaryOp::Not => Some(Type::Bool),
                            }
                        } else {
                            let op_str = match op {
                                UnaryOp::Neg => "-",
                                UnaryOp::Not => "NOT",
                            };
                            let type_str = typ.to_rust_str();
                            err(
                                errors,
                                SemanticErrorCode::E1022,
                                format!(
                                    "Invalid unary operation '{}' on type {}",
                                    op_str, type_str
                                ),
                            );
                            None
                        }
                    }
                }
            }
            Expression::Binary { left, op, right } => {
                let l = resolve_expr(left, locals, globals, functions, arrays, errors);
                let r = resolve_expr(right, locals, globals, functions, arrays, errors);
                match (op, &l, &r) {
                    (_, None, _) | (_, _, None) => None,
                    (op, Some(ref lt), Some(ref rt)) => {
                        if binary_op_valid(op, lt, rt) {
                            // Integer-integer operations use automatic widening
                            if lt.is_integer()
                                && rt.is_integer()
                                && lt.is_signed() == rt.is_signed()
                            {
                                return Some(match op {
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
                                    _ => unreachable!(),
                                });
                            }

                            // Logical operations on BOOL
                            if *lt == Type::Bool && *rt == Type::Bool {
                                return Some(match op {
                                    BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => Type::Bool,
                                    _ => Type::Bool,
                                });
                            }

                            // Other numeric/float operations
                            if lt.is_numeric()
                                && rt.is_numeric()
                                && !(lt.is_integer()
                                    && rt.is_integer()
                                    && lt.is_signed() != rt.is_signed())
                            {
                                match op {
                                    BinaryOp::Add
                                    | BinaryOp::Sub
                                    | BinaryOp::Mul
                                    | BinaryOp::Div => {
                                        let result = if *lt == Type::F64 || *rt == Type::F64 {
                                            Type::F64
                                        } else {
                                            Type::F32
                                        };
                                        return Some(result);
                                    }
                                    BinaryOp::Pow => return Some(Type::F64),
                                    BinaryOp::Eq
                                    | BinaryOp::NotEq
                                    | BinaryOp::Lt
                                    | BinaryOp::Lte
                                    | BinaryOp::Gt
                                    | BinaryOp::Gte => return Some(Type::Bool),

                                    _ => {} // IntDiv, Mod — fall through to error
                                }
                            }

                            // String concatenation
                            if *lt == Type::String && *rt == Type::String {
                                return Some(Type::String);
                            }

                            // Bit shift operations
                            if lt.is_integer() && rt.is_integer() {
                                match op {
                                    BinaryOp::Shl | BinaryOp::Shr => return Some(lt.clone()),
                                    _ => {}
                                }
                            }

                            // This should not happen if binary_op_valid returned true
                            let op_str = match op {
                                BinaryOp::Add => "+",
                                BinaryOp::Sub => "-",
                                BinaryOp::Mul => "*",
                                BinaryOp::Div => "/",
                                BinaryOp::Pow => "^",
                                BinaryOp::IntDiv => "\\",
                                BinaryOp::Mod => "MOD",
                                BinaryOp::Eq => "=",
                                BinaryOp::NotEq => "<>",
                                BinaryOp::Lt => "<",
                                BinaryOp::Lte => "<=",
                                BinaryOp::Gt => ">",
                                BinaryOp::Gte => ">=",
                                BinaryOp::And => "AND",
                                BinaryOp::Or => "OR",
                                BinaryOp::Xor => "XOR",
                                BinaryOp::Shl => "SHL",
                                BinaryOp::Shr => "SHR",
                            };
                            let left_str = lt.to_rust_str();
                            let right_str = rt.to_rust_str();
                            err(
                                errors,
                                SemanticErrorCode::E1021,
                                format!(
                                    "Invalid operation '{}' between {} and {}",
                                    op_str, left_str, right_str
                                ),
                            );
                            None
                        } else {
                            // Binary operation is invalid
                            let op_str = match op {
                                BinaryOp::Add => "+",
                                BinaryOp::Sub => "-",
                                BinaryOp::Mul => "*",
                                BinaryOp::Div => "/",
                                BinaryOp::Pow => "^",
                                BinaryOp::IntDiv => "\\",
                                BinaryOp::Mod => "MOD",
                                BinaryOp::Eq => "=",
                                BinaryOp::NotEq => "<>",
                                BinaryOp::Lt => "<",
                                BinaryOp::Lte => "<=",
                                BinaryOp::Gt => ">",
                                BinaryOp::Gte => ">=",
                                BinaryOp::And => "AND",
                                BinaryOp::Or => "OR",
                                BinaryOp::Xor => "XOR",
                                BinaryOp::Shl => "SHL",
                                BinaryOp::Shr => "SHR",
                            };
                            let left_str = lt.to_rust_str();
                            let right_str = rt.to_rust_str();
                            // Determine specific error code
                            if lt.is_integer()
                                && rt.is_integer()
                                && lt.is_signed() != rt.is_signed()
                            {
                                err(
                                    errors,
                                    SemanticErrorCode::E1021,
                                    format!(
                                        "Signed/unsigned type mismatch in operation '{}'",
                                        op_str
                                    ),
                                );
                            } else {
                                err(
                                    errors,
                                    SemanticErrorCode::E1021,
                                    format!(
                                        "Invalid operation '{}' between {} and {}",
                                        op_str, left_str, right_str
                                    ),
                                );
                            }
                            None
                        }
                    }
                }
            }
            Expression::Grouping(inner) => {
                resolve_expr(inner, locals, globals, functions, arrays, errors)
            }
            Expression::Cast { expr, target_type } => {
                let inner = resolve_expr(expr, locals, globals, functions, arrays, errors)
                    .unwrap_or(Type::I32);
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
                // Check if callee is a DIM'd array (array element read)
                if let Some((base_type, expected_dims)) = arrays.get(&key) {
                    if args.len() != *expected_dims {
                        err(
                            errors,
                            SemanticErrorCode::E1062,
                            format!(
                                "Array {} expects {} dimensions, got {}",
                                callee,
                                expected_dims,
                                args.len()
                            ),
                        );
                    }
                    for (i, idx) in args.iter().enumerate() {
                        let idx_type =
                            resolve_expr(idx, locals, globals, functions, arrays, errors);
                        if let Some(t) = &idx_type {
                            if !t.is_integer() {
                                err(
                                    errors,
                                    SemanticErrorCode::E1061,
                                    format!(
                                        "Array index {} of {} must be integer, got {}",
                                        i + 1,
                                        callee,
                                        t.to_rust_str()
                                    ),
                                );
                            }
                        }
                    }
                    return Some(base_type.clone());
                }
                if let Some(sig) = functions.get(&key) {
                    check_function_call(
                        callee,
                        args,
                        &sig.param_types,
                        sig.ret_type.clone(),
                        locals,
                        globals,
                        functions,
                        arrays,
                        errors,
                    )
                } else if let Some(ret) =
                    check_builtin_call(callee, args, locals, globals, functions, arrays, errors)
                {
                    Some(ret)
                } else {
                    err(
                        errors,
                        SemanticErrorCode::E1003,
                        format!("Unknown function {}", callee),
                    );
                    None
                }
            }
            Expression::ArrayAccess { name, indices } => {
                // Handled via Call resolution above; fallback for completeness
                let key = name.to_lowercase();
                if let Some((base_type, expected_dims)) = arrays.get(&key) {
                    if indices.len() != *expected_dims {
                        err(
                            errors,
                            SemanticErrorCode::E1062,
                            format!(
                                "Array {} expects {} dimensions, got {}",
                                name,
                                expected_dims,
                                indices.len()
                            ),
                        );
                    }
                    Some(base_type.clone())
                } else {
                    err(
                        errors,
                        SemanticErrorCode::E1060,
                        format!("Unknown array {}", name),
                    );
                    None
                }
            }
        }
    }

    // Walk statements, tracking locals
    #[allow(clippy::too_many_arguments)]
    fn walk_stmt(
        stmt: &Statement,
        locals: &mut HashMap<String, (Type, bool)>,
        globals: &HashMap<String, Type>,
        functions: &HashMap<String, FuncSig>,
        arrays: &mut ArrayInfo,
        errors: &mut Vec<SemanticError>,
        inside_function: bool,
        current_ret_type: Option<&Type>,
    ) {
        match stmt {
            Statement::VarDecl {
                name,
                is_mut,
                typ,
                init,
            } => {
                let init_type = resolve_expr(init, locals, globals, functions, arrays, errors);
                if let Some(ref t) = typ {
                    if Type::from_name(&t.name).is_none() {
                        err(
                            errors,
                            SemanticErrorCode::E1010,
                            format!("Unknown type {}", t.name),
                        );
                    }
                }
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
                    e.insert((resolved, *is_mut));
                } else {
                    err(
                        errors,
                        SemanticErrorCode::E1002,
                        format!("Duplicate local variable {}", name),
                    );
                }
            }
            Statement::Print { expr } => {
                resolve_expr(expr, locals, globals, functions, arrays, errors);
            }
            Statement::ExpressionStmt { expr } => {
                resolve_expr(expr, locals, globals, functions, arrays, errors);
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
                        let actual = resolve_expr(expr, locals, globals, functions, arrays, errors);
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
                let cond_type = resolve_expr(condition, locals, globals, functions, arrays, errors);
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
                        arrays,
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
                            arrays,
                            errors,
                            inside_function,
                            current_ret_type,
                        );
                    }
                }
            }
            Statement::While { condition, body } => {
                let cond_type = resolve_expr(condition, locals, globals, functions, arrays, errors);
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
                        arrays,
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
                let start_type = resolve_expr(start, locals, globals, functions, arrays, errors);
                let end_type = resolve_expr(end, locals, globals, functions, arrays, errors);
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
                    let step_type =
                        resolve_expr(step_expr, locals, globals, functions, arrays, errors);
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
                for_locals.insert(var.to_lowercase(), (loop_var_type, true));
                for s in body {
                    walk_stmt(
                        s,
                        &mut for_locals,
                        globals,
                        functions,
                        arrays,
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
                    let cond_type = resolve_expr(cond, locals, globals, functions, arrays, errors);
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
                        arrays,
                        errors,
                        inside_function,
                        current_ret_type,
                    );
                }
            }
            Statement::FunctionDecl { params, body, .. } => {
                let mut func_locals: HashMap<String, (Type, bool)> = HashMap::new();
                for p in params {
                    if let Some(t) = Type::from_name(&p.typ.name) {
                        func_locals.insert(p.name.to_lowercase(), (t, true));
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
                        arrays,
                        errors,
                        true,
                        ret_type.as_ref(),
                    );
                }
            }
            Statement::Assign { name, expr } => {
                let key = name.to_lowercase();
                let declared = locals.get(&key).map(|(t, _)| t).cloned();
                if declared.is_none() {
                    err(
                        errors,
                        SemanticErrorCode::E1040,
                        format!("Assignment to undeclared variable {}", name),
                    );
                } else {
                    let is_mut = locals.get(&key).map(|(_, m)| *m).unwrap_or(false);
                    if !is_mut {
                        err(
                            errors,
                            SemanticErrorCode::E1042,
                            format!("Assignment to immutable variable {}", name),
                        );
                    }
                    let expr_type = resolve_expr(expr, locals, globals, functions, arrays, errors);
                    if let (Some(d), Some(a)) = (&declared, &expr_type) {
                        if !types_compatible(d, a) {
                            err(
                                errors,
                                SemanticErrorCode::E1041,
                                format!(
                                    "Type mismatch: variable {} is {} but assigned value is {}",
                                    name,
                                    d.to_rust_str(),
                                    a.to_rust_str(),
                                ),
                            );
                        }
                    }
                }
            }
            Statement::AssignOp { name, op, expr } => {
                let key = name.to_lowercase();
                if let Some((var_type, is_mut)) = locals.get(&key).cloned() {
                    if !is_mut {
                        err(
                            errors,
                            SemanticErrorCode::E1044,
                            format!("Compound assignment to immutable variable {}", name),
                        );
                    }
                    let binary_op = match op {
                        CompoundAssignOp::AddEq => BinaryOp::Add,
                        CompoundAssignOp::SubEq => BinaryOp::Sub,
                        CompoundAssignOp::MulEq => BinaryOp::Mul,
                        CompoundAssignOp::DivEq => BinaryOp::Div,
                        CompoundAssignOp::IntDivEq => BinaryOp::IntDiv,
                        CompoundAssignOp::ModEq => BinaryOp::Mod,
                    };
                    let op_sym = match op {
                        CompoundAssignOp::AddEq => "+",
                        CompoundAssignOp::SubEq => "-",
                        CompoundAssignOp::MulEq => "*",
                        CompoundAssignOp::DivEq => "/",
                        CompoundAssignOp::IntDivEq => "\\",
                        CompoundAssignOp::ModEq => "MOD",
                    };
                    let rhs_type = resolve_expr(expr, locals, globals, functions, arrays, errors);
                    if let Some(rhs) = &rhs_type {
                        if !binary_op_valid(&binary_op, &var_type, rhs) {
                            err(
                                errors,
                                SemanticErrorCode::E1045,
                                format!(
                                    "Compound assignment type mismatch: {} {}= {}",
                                    var_type.to_rust_str(),
                                    op_sym,
                                    rhs.to_rust_str()
                                ),
                            );
                        }
                    }
                } else {
                    err(
                        errors,
                        SemanticErrorCode::E1043,
                        format!("Compound assignment to undeclared variable {}", name),
                    );
                    resolve_expr(expr, locals, globals, functions, arrays, errors);
                }
            }
            Statement::ArrayAssign {
                name,
                indices,
                value,
            } => {
                let key = name.to_lowercase();
                // Check array exists
                let array_info = arrays.get(&key).cloned().or_else(|| {
                    err(
                        errors,
                        SemanticErrorCode::E1060,
                        format!("Unknown array {}", name),
                    );
                    None
                });
                if let Some((base_type, expected_dims)) = array_info {
                    // Validate dimension count
                    if indices.len() != expected_dims {
                        err(
                            errors,
                            SemanticErrorCode::E1062,
                            format!(
                                "Array {} expects {} dimensions, got {}",
                                name,
                                expected_dims,
                                indices.len()
                            ),
                        );
                    }
                    // Validate each index is integer type
                    for (i, idx) in indices.iter().enumerate() {
                        let idx_type =
                            resolve_expr(idx, locals, globals, functions, arrays, errors);
                        if let Some(t) = &idx_type {
                            if !t.is_integer() {
                                err(
                                    errors,
                                    SemanticErrorCode::E1061,
                                    format!(
                                        "Array index {} of {} must be integer, got {}",
                                        i + 1,
                                        name,
                                        t.to_rust_str()
                                    ),
                                );
                            }
                        }
                    }
                    // Validate value type matches array base type
                    let value_type =
                        resolve_expr(value, locals, globals, functions, arrays, errors);
                    if let Some(vt) = &value_type {
                        if !types_compatible(&base_type, vt) {
                            err(
                                errors,
                                SemanticErrorCode::E1020,
                                format!(
                                    "Type mismatch: array {} expects {}, got {}",
                                    name,
                                    base_type.to_rust_str(),
                                    vt.to_rust_str()
                                ),
                            );
                        }
                    }
                }
            }
            Statement::Dim { declarations } => {
                for decl in declarations {
                    let base_type =
                        Type::from_name(&decl.array_type.base_type.name).unwrap_or(Type::I32);
                    let dim_count = decl.array_type.dimensions.len();
                    let key = decl.name.to_lowercase();
                    if let std::collections::hash_map::Entry::Vacant(e) = locals.entry(key.clone())
                    {
                        e.insert((base_type.clone(), false));
                    } else {
                        err(
                            errors,
                            SemanticErrorCode::E1002,
                            format!("Duplicate array variable {}", decl.name),
                        );
                    }
                    // Also track array info for access validation
                    arrays.insert(key, (base_type, dim_count));
                }
            }
            Statement::OnError { .. } => {
                // ON ERROR GOTO — no semantic validation in v0.1
            }
            Statement::Input { target, .. } => {
                let key = target.to_lowercase();
                match locals.get(&key) {
                    None => err(
                        errors,
                        SemanticErrorCode::E1050,
                        format!("INPUT target '{}' is not declared", target),
                    ),
                    Some((ty, is_mut)) => {
                        if !is_mut {
                            err(
                                errors,
                                SemanticErrorCode::E1051,
                                format!("INPUT target '{}' is not mutable", target),
                            );
                        }
                        let supported = matches!(
                            ty,
                            Type::String
                                | Type::I32
                                | Type::I64
                                | Type::F64
                                | Type::Bool
                                | Type::U8
                        );
                        if !supported {
                            err(
                                errors,
                                SemanticErrorCode::E1052,
                                format!(
                                    "INPUT does not support type {} for variable '{}'",
                                    ty.to_rust_str(),
                                    target
                                ),
                            );
                        }
                    }
                }
            }
            Statement::Resume { .. } => {
                // RESUME — no semantic validation in v0.1
            }
            // File I/O statements
            Statement::Open {
                filename,
                mode,
                handle,
                record_len,
            } => {
                resolve_expr(filename, locals, globals, functions, arrays, errors);
                resolve_expr(handle, locals, globals, functions, arrays, errors);
                if let Some(len) = record_len {
                    resolve_expr(len, locals, globals, functions, arrays, errors);
                }
                // Validate mode-specific requirements
                if *mode == FileMode::Random && record_len.is_none() {
                    err(
                        errors,
                        SemanticErrorCode::E1060,
                        "RANDOM mode requires LEN clause".to_string(),
                    );
                }
            }
            Statement::Close { handles } => {
                for handle in handles {
                    resolve_expr(handle, locals, globals, functions, arrays, errors);
                }
            }
            Statement::InputHash { handle, targets } => {
                resolve_expr(handle, locals, globals, functions, arrays, errors);
                for target in targets {
                    let key = target.to_lowercase();
                    if !locals.contains_key(&key) {
                        err(
                            errors,
                            SemanticErrorCode::E1061,
                            format!("INPUT# target '{}' is not declared", target),
                        );
                    }
                }
            }
            Statement::PrintHash { handle, items } => {
                resolve_expr(handle, locals, globals, functions, arrays, errors);
                for item in items {
                    if let PrintItem::Expr(expr) = item {
                        resolve_expr(expr, locals, globals, functions, arrays, errors);
                    }
                }
            }
            Statement::LineInputHash { handle, target } => {
                resolve_expr(handle, locals, globals, functions, arrays, errors);
                let key = target.to_lowercase();
                if !locals.contains_key(&key) {
                    err(
                        errors,
                        SemanticErrorCode::E1062,
                        format!("LINE INPUT# target '{}' is not declared", target),
                    );
                }
            }
            Statement::SubDecl { .. } => {
                // SUB declaration not yet supported in semantic analysis
            }
            Statement::SelectCase {
                expr,
                cases,
                else_case,
            } => {
                // Analyze the select expression
                let select_type = resolve_expr(expr, locals, globals, functions, arrays, errors);

                // Analyze each case clause
                for case in cases {
                    for value in &case.values {
                        match value {
                            CaseValue::Single(val_expr) => {
                                let val_type = resolve_expr(
                                    val_expr, locals, globals, functions, arrays, errors,
                                );
                                if let (Some(st), Some(vt)) = (&select_type, &val_type) {
                                    if !types_compatible(st, vt) {
                                        err(
                                            errors,
                                            SemanticErrorCode::E1020,
                                            format!("case value type {:?} not compatible with select expression type {:?}", vt, st),
                                        );
                                    }
                                }
                            }
                            CaseValue::Range(low, high) => {
                                let low_type =
                                    resolve_expr(low, locals, globals, functions, arrays, errors);
                                let high_type =
                                    resolve_expr(high, locals, globals, functions, arrays, errors);
                                if let (Some(st), Some(lt)) = (&select_type, &low_type) {
                                    if !types_compatible(st, lt) {
                                        err(
                                            errors,
                                            SemanticErrorCode::E1020,
                                            format!("range low bound type {:?} not compatible with select expression type {:?}", lt, st),
                                        );
                                    }
                                }
                                if let (Some(st), Some(ht)) = (&select_type, &high_type) {
                                    if !types_compatible(st, ht) {
                                        err(
                                            errors,
                                            SemanticErrorCode::E1020,
                                            format!("range high bound type {:?} not compatible with select expression type {:?}", ht, st),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    // Analyze case body
                    for stmt in &case.body {
                        walk_stmt(
                            stmt,
                            locals,
                            globals,
                            functions,
                            arrays,
                            errors,
                            inside_function,
                            current_ret_type,
                        );
                    }
                }

                // Analyze else case body
                if let Some(else_stmts) = else_case {
                    for stmt in else_stmts {
                        walk_stmt(
                            stmt,
                            locals,
                            globals,
                            functions,
                            arrays,
                            errors,
                            inside_function,
                            current_ret_type,
                        );
                    }
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

    // Walk top-level statements — share a persistent locals map so declarations
    // persist across statements (e.g., LET + assignment on separate lines).
    let mut top_locals: HashMap<String, (Type, bool)> = HashMap::new();
    for stmt in &prog.statements {
        walk_stmt(
            stmt,
            &mut top_locals,
            &globals,
            &functions,
            &mut arrays,
            &mut errors,
            false,
            None,
        );
    }

    if errors.is_empty() {
        Ok(arrays)
    } else {
        Err(errors)
    }
}
