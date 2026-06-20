use std::fmt::Write;

use crate::parser::ast::{
    BinaryOp, CompoundAssignOp, DoLoopVariant, Expression, Literal, Program, Statement, UnaryOp,
};
use crate::semantic::types::Type;

pub fn generate_rust(program: &Program) -> String {
    let mut out = String::from("fn main() {\n");
    for stmt in &program.statements {
        gen_stmt(stmt, &mut out, 1);
    }
    out.push_str("}\n");
    out
}

fn gen_stmt(stmt: &Statement, out: &mut String, indent: usize) {
    let pad = "    ".repeat(indent);
    match stmt {
        Statement::VarDecl {
            name,
            is_mut,
            typ,
            init,
        } => {
            out.push_str(&pad);
            if *is_mut {
                out.push_str("let mut ");
            } else {
                out.push_str("let ");
            }
            out.push_str(name);
            if let Some(t) = typ {
                out.push_str(": ");
                out.push_str(&type_to_rust(&t.name));
            }
            out.push_str(" = ");
            gen_expr(init, out);
            out.push_str(";\n");
        }
        Statement::Print { expr } => {
            out.push_str(&pad);
            out.push_str("println!(\"{}\", ");
            gen_expr(expr, out);
            out.push_str(");\n");
        }
        Statement::Return { expr } => {
            out.push_str(&pad);
            out.push_str("return");
            if let Some(e) = expr {
                out.push(' ');
                gen_expr(e, out);
            }
            out.push_str(";\n");
        }
        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => {
            out.push_str(&pad);
            out.push_str("if ");
            gen_expr(condition, out);
            out.push_str(" {\n");
            for s in then_branch {
                gen_stmt(s, out, indent + 1);
            }
            if let Some(else_branch) = else_branch {
                out.push_str(&pad);
                out.push_str("} else {\n");
                for s in else_branch {
                    gen_stmt(s, out, indent + 1);
                }
            }
            out.push_str(&pad);
            out.push_str("}\n");
        }
        Statement::While { condition, body } => {
            out.push_str(&pad);
            out.push_str("while ");
            gen_expr(condition, out);
            out.push_str(" {\n");
            for s in body {
                gen_stmt(s, out, indent + 1);
            }
            out.push_str(&pad);
            out.push_str("}\n");
        }
        Statement::For {
            var,
            start,
            end,
            step,
            body,
        } => {
            out.push_str(&pad);
            out.push_str("{\n");
            let inner = "    ".repeat(indent + 1);
            out.push_str(&inner);
            out.push_str("let mut ");
            out.push_str(var);
            out.push_str(" = ");
            gen_expr(start, out);
            out.push_str(";\n");
            if let Some(step_expr) = step {
                out.push_str(&inner);
                out.push_str("let step = ");
                gen_expr(step_expr, out);
                out.push_str(";\n");
                out.push_str(&inner);
                out.push_str("if step >= 0 {\n");
                out.push_str(&"    ".repeat(indent + 2));
                out.push_str("while ");
                out.push_str(var);
                out.push_str(" <= ");
                gen_expr(end, out);
                out.push_str(" {\n");
                for s in body {
                    gen_stmt(s, out, indent + 3);
                }
                out.push_str(&"    ".repeat(indent + 2));
                out.push_str("    ");
                out.push_str(var);
                out.push_str(" += step;\n");
                out.push_str(&"    ".repeat(indent + 2));
                out.push_str("}\n");
                out.push_str(&inner);
                out.push_str("} else {\n");
                out.push_str(&"    ".repeat(indent + 2));
                out.push_str("while ");
                out.push_str(var);
                out.push_str(" >= ");
                gen_expr(end, out);
                out.push_str(" {\n");
                for s in body {
                    gen_stmt(s, out, indent + 3);
                }
                out.push_str(&"    ".repeat(indent + 2));
                out.push_str("    ");
                out.push_str(var);
                out.push_str(" += step;\n");
                out.push_str(&"    ".repeat(indent + 2));
                out.push_str("}\n");
                out.push_str(&inner);
                out.push_str("}\n");
            } else {
                out.push_str(&inner);
                out.push_str("while ");
                out.push_str(var);
                out.push_str(" <= ");
                gen_expr(end, out);
                out.push_str(" {\n");
                for s in body {
                    gen_stmt(s, out, indent + 2);
                }
                out.push_str(&inner);
                out.push_str("    ");
                out.push_str(var);
                out.push_str(" += 1;\n");
                out.push_str(&inner);
                out.push_str("}\n");
            }
            out.push_str(&pad);
            out.push_str("}\n");
        }
        Statement::DoLoop {
            variant,
            condition,
            body,
        } => match (variant, condition) {
            (DoLoopVariant::WhilePre, Some(cond)) => {
                out.push_str(&pad);
                out.push_str("while ");
                gen_expr(cond, out);
                out.push_str(" {\n");
                for s in body {
                    gen_stmt(s, out, indent + 1);
                }
                out.push_str(&pad);
                out.push_str("}\n");
            }
            (DoLoopVariant::UntilPre, Some(cond)) => {
                out.push_str(&pad);
                out.push_str("while !(");
                gen_expr(cond, out);
                out.push_str(") {\n");
                for s in body {
                    gen_stmt(s, out, indent + 1);
                }
                out.push_str(&pad);
                out.push_str("}\n");
            }
            (DoLoopVariant::WhilePost, Some(cond)) => {
                out.push_str(&pad);
                out.push_str("loop {\n");
                for s in body {
                    gen_stmt(s, out, indent + 1);
                }
                out.push_str(&"    ".repeat(indent));
                out.push_str("    if !(");
                gen_expr(cond, out);
                out.push_str(") { break; }\n");
                out.push_str(&pad);
                out.push_str("}\n");
            }
            (DoLoopVariant::UntilPost, Some(cond)) => {
                out.push_str(&pad);
                out.push_str("loop {\n");
                for s in body {
                    gen_stmt(s, out, indent + 1);
                }
                out.push_str(&"    ".repeat(indent));
                out.push_str("    if ");
                gen_expr(cond, out);
                out.push_str(" { break; }\n");
                out.push_str(&pad);
                out.push_str("}\n");
            }
            _ => {
                out.push_str(&pad);
                out.push_str("// unreachable: DO loop without condition\n");
            }
        },
        Statement::ExpressionStmt { expr } => {
            out.push_str(&pad);
            gen_expr(expr, out);
            out.push_str(";\n");
        }
        Statement::Assign { name, expr } => {
            out.push_str(&pad);
            out.push_str(name);
            out.push_str(" = ");
            gen_expr(expr, out);
            out.push_str(";\n");
        }
        Statement::FunctionDecl {
            name,
            params,
            ret_type,
            body,
        } => {
            out.push_str(&pad);
            out.push_str("fn ");
            out.push_str(name);
            out.push('(');
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&p.name);
                out.push_str(": ");
                out.push_str(&type_to_rust(&p.typ.name));
            }
            out.push(')');
            if let Some(rt) = ret_type {
                out.push_str(" -> ");
                out.push_str(&type_to_rust(&rt.name));
            }
            out.push_str(" {\n");
            for s in body {
                gen_stmt(s, out, indent + 1);
            }
            out.push_str(&pad);
            out.push_str("}\n");
        }
        // DIM arrays (RFC-0016)
        Statement::Dim { declarations } => {
            for decl in declarations {
                let base_type =
                    Type::from_name(&decl.array_type.base_type.name).unwrap_or(Type::I32);
                let dims = &decl.array_type.dimensions;
                let n = dims.len();
                out.push_str(&pad);
                out.push_str("let ");
                out.push_str(&decl.name);
                out.push_str(": ");
                // Vec<Vec<...<T>...>>
                for _ in 0..n {
                    out.push_str("Vec<");
                }
                out.push_str(&type_to_rust(base_type.to_rust_str()));
                for _ in 0..n {
                    out.push('>');
                }
                out.push_str(" = ");
                if n == 0 {
                    out.push_str("Vec::new()");
                } else {
                    let default = default_for_type(&base_type);
                    gen_dim_init(dims, &default, out, 0);
                }
                out.push_str(";\n");
            }
        }
        Statement::AssignOp { name, op, expr } => {
            // Desugar: x += y  →  x = x + y
            out.push_str(&pad);
            out.push_str(name);
            out.push_str(" = ");
            out.push_str(name);
            let op_str = match op {
                CompoundAssignOp::AddEq => " + ",
                CompoundAssignOp::SubEq => " - ",
                CompoundAssignOp::MulEq => " * ",
                CompoundAssignOp::DivEq => " / ",
                CompoundAssignOp::IntDivEq => " / ",
                CompoundAssignOp::ModEq => " % ",
            };
            out.push_str(op_str);
            gen_expr(expr, out);
            out.push_str(";\n");
        }
        Statement::ArrayAssign {
            name,
            indices,
            value,
        } => {
            out.push_str(&pad);
            out.push_str(name);
            for idx in indices {
                out.push('[');
                gen_expr(idx, out);
                out.push_str(" as usize]");
            }
            out.push_str(" = ");
            gen_expr(value, out);
            out.push_str(";\n");
        }
        Statement::OnError { .. } => {}
        Statement::Resume { .. } => {}
    }
}

fn gen_expr(expr: &Expression, out: &mut String) {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Int(v) => out.push_str(&v.to_string()),
            Literal::Float(v) => {
                let s = v.to_string();
                if !s.contains('.') {
                    write!(out, "{}.0", s).unwrap();
                } else {
                    out.push_str(&s);
                }
            }
            Literal::String(s) => {
                out.push('"');
                out.push_str(&escape_string(s));
                out.push_str("\".to_string()");
            }
            Literal::Bool(v) => out.push_str(if *v { "true" } else { "false" }),
        },
        Expression::Identifier(name) => out.push_str(name),
        Expression::Unary { op, expr } => {
            let op_str = match op {
                UnaryOp::Neg => "-",
                UnaryOp::Not => "!",
            };
            out.push_str(op_str);
            out.push('(');
            gen_expr(expr, out);
            out.push(')');
        }
        Expression::Binary { left, op, right } => {
            if *op == BinaryOp::Pow {
                out.push_str("((");
                gen_expr(left, out);
                out.push_str(" as f64).powf(");
                gen_expr(right, out);
                out.push_str(" as f64))");
            } else {
                let op_str = match op {
                    BinaryOp::Add => " + ",
                    BinaryOp::Sub => " - ",
                    BinaryOp::Mul => " * ",
                    BinaryOp::Div => " / ",
                    BinaryOp::Pow => unreachable!(),
                    BinaryOp::IntDiv => " / ",
                    BinaryOp::Mod => " % ",
                    BinaryOp::Eq => " == ",
                    BinaryOp::NotEq => " != ",
                    BinaryOp::Lt => " < ",
                    BinaryOp::Lte => " <= ",
                    BinaryOp::Gt => " > ",
                    BinaryOp::Gte => " >= ",
                    BinaryOp::And => " && ",
                    BinaryOp::Or => " || ",
                    BinaryOp::Xor => " ^ ",
                    BinaryOp::Shl => " << ",
                    BinaryOp::Shr => " >> ",
                };
                gen_expr(left, out);
                out.push_str(op_str);
                gen_expr(right, out);
            }
        }
        Expression::Grouping(inner) => {
            out.push('(');
            gen_expr(inner, out);
            out.push(')');
        }
        Expression::Cast { expr, target_type } => {
            gen_expr(expr, out);
            let target = Type::from_name(target_type)
                .map(|t| t.to_rust_str().to_string())
                .unwrap_or_else(|| target_type.to_lowercase());
            out.push_str(" as ");
            out.push_str(&target);
        }
        Expression::Call { callee, args } => {
            if is_builtin(callee) {
                gen_builtin_call(callee, args, out);
            } else {
                out.push_str(callee);
                out.push('(');
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    gen_expr(a, out);
                }
                out.push(')');
            }
        }
        Expression::ArrayAccess { name, indices } => {
            out.push_str(name);
            for idx in indices {
                out.push('[');
                gen_expr(idx, out);
                out.push_str(" as usize]");
            }
        }
    }
}

fn type_to_rust(name: &str) -> String {
    match name.to_uppercase().as_str() {
        "BOOL" => "bool".to_string(),
        "I8" => "i8".to_string(),
        "I16" => "i16".to_string(),
        "I32" => "i32".to_string(),
        "I64" => "i64".to_string(),
        "U8" => "u8".to_string(),
        "U16" => "u16".to_string(),
        "U32" => "u32".to_string(),
        "U64" => "u64".to_string(),
        "F32" => "f32".to_string(),
        "F64" => "f64".to_string(),
        "STRING" => "String".to_string(),
        _ => name.to_string(),
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn default_for_type(t: &Type) -> String {
    match t {
        Type::Bool => "false".to_string(),
        Type::I8 => "0i8".to_string(),
        Type::I16 => "0i16".to_string(),
        Type::I32 => "0i32".to_string(),
        Type::I64 => "0i64".to_string(),
        Type::U8 => "0u8".to_string(),
        Type::U16 => "0u16".to_string(),
        Type::U32 => "0u32".to_string(),
        Type::U64 => "0u64".to_string(),
        Type::F32 => "0.0f32".to_string(),
        Type::F64 => "0.0f64".to_string(),
        Type::String => "String::new()".to_string(),
    }
}

fn gen_dim_init(dims: &[Expression], default: &str, out: &mut String, idx: usize) {
    if idx >= dims.len() {
        out.push_str(default);
        return;
    }
    let size = if let Expression::Literal(Literal::Int(v)) = &dims[idx] {
        *v + 1
    } else {
        0
    };
    out.push_str("vec![");
    gen_dim_init(dims, default, out, idx + 1);
    write!(out, "; {}]", size).unwrap();
}

fn is_builtin(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "len"
            | "mid$"
            | "mid"
            | "left$"
            | "left"
            | "right$"
            | "right"
            | "chr$"
            | "chr"
            | "asc"
            | "instr"
            | "val"
            | "str$"
            | "str"
            | "ucase$"
            | "ucase"
            | "lcase$"
            | "lcase"
            | "trim$"
            | "trim"
            | "ltrim$"
            | "ltrim"
            | "rtrim$"
            | "rtrim"
            | "space$"
            | "space"
            | "string$"
            | "string"
    )
}

fn gen_builtin_call(callee: &str, args: &[Expression], out: &mut String) {
    match callee.to_lowercase().as_str() {
        "len" => {
            gen_expr(&args[0], out);
            out.push_str(".len() as i32");
        }
        "mid$" | "mid" => {
            gen_expr(&args[0], out);
            out.push_str(".chars().skip((");
            gen_expr(&args[1], out);
            out.push_str(" - 1) as usize).take(");
            gen_expr(&args[2], out);
            out.push_str(" as usize).collect::<String>()");
        }
        "left$" | "left" => {
            gen_expr(&args[0], out);
            out.push_str(".chars().take(");
            gen_expr(&args[1], out);
            out.push_str(" as usize).collect::<String>()");
        }
        "right$" | "right" => {
            gen_expr(&args[0], out);
            out.push_str(".chars().rev().take(");
            gen_expr(&args[1], out);
            out.push_str(" as usize).collect::<String>().chars().rev().collect::<String>()");
        }
        "chr$" | "chr" => {
            out.push_str("char::from_u32(");
            gen_expr(&args[0], out);
            out.push_str(" as u32).map(|c| c.to_string()).unwrap_or_default()");
        }
        "asc" => {
            gen_expr(&args[0], out);
            out.push_str(".chars().next().map(|c| c as i32).unwrap_or(0)");
        }
        "instr" => {
            if args.len() == 2 {
                gen_expr(&args[0], out);
                out.push_str(".find(&");
                gen_expr(&args[1], out);
                out.push_str(").map(|i| i as i32 + 1).unwrap_or(0)");
            } else {
                out.push('(');
                gen_expr(&args[1], out);
                out.push('[');
                gen_expr(&args[0], out);
                out.push_str(" as usize..]).find(&");
                gen_expr(&args[2], out);
                out.push_str(").map(|i| (i + ");
                gen_expr(&args[0], out);
                out.push_str(" as usize) as i32 + 1).unwrap_or(0)");
            }
        }
        "val" => {
            out.push('(');
            gen_expr(&args[0], out);
            out.push_str(".trim().parse::<f64>().unwrap_or(0.0))");
        }
        "str$" | "str" => {
            gen_expr(&args[0], out);
            out.push_str(".to_string()");
        }
        "ucase$" | "ucase" => {
            gen_expr(&args[0], out);
            out.push_str(".to_uppercase()");
        }
        "lcase$" | "lcase" => {
            gen_expr(&args[0], out);
            out.push_str(".to_lowercase()");
        }
        "trim$" | "trim" => {
            gen_expr(&args[0], out);
            out.push_str(".trim().to_string()");
        }
        "ltrim$" | "ltrim" => {
            gen_expr(&args[0], out);
            out.push_str(".trim_start().to_string()");
        }
        "rtrim$" | "rtrim" => {
            gen_expr(&args[0], out);
            out.push_str(".trim_end().to_string()");
        }
        "space$" | "space" => {
            out.push_str("\" \".repeat(");
            gen_expr(&args[0], out);
            out.push_str(" as usize)");
        }
        "string$" | "string" => {
            gen_expr(&args[1], out);
            out.push_str(".repeat(");
            gen_expr(&args[0], out);
            out.push_str(" as usize)");
        }
        _ => {
            // Fallback: generic call (should not happen since is_builtin check passes)
            out.push_str(callee);
            out.push('(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                gen_expr(a, out);
            }
            out.push(')');
        }
    }
}
