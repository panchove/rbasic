#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
}

impl Type {
    pub fn from_name(name: &str) -> Option<Type> {
        match name.to_uppercase().as_str() {
            // Canonical names
            "BOOL" => Some(Type::Bool),
            "I8" => Some(Type::I8),
            "I16" => Some(Type::I16),
            "I32" => Some(Type::I32),
            "I64" => Some(Type::I64),
            "U8" => Some(Type::U8),
            "U16" => Some(Type::U16),
            "U32" => Some(Type::U32),
            "U64" => Some(Type::U64),
            "F32" => Some(Type::F32),
            "F64" => Some(Type::F64),
            "STRING" => Some(Type::String),
            // Classic BASIC aliases (RFC-0011)
            "BOOLEAN" => Some(Type::Bool),
            "BYTE" => Some(Type::U8),
            "WORD" => Some(Type::U16),
            "DWORD" => Some(Type::U32),
            "QWORD" => Some(Type::U64),
            "INTEGER" => Some(Type::I32),
            "LONG" | "LONGLONG" => Some(Type::I64),
            "SINGLE" => Some(Type::F32),
            "DOUBLE" => Some(Type::F64),
            _ => None,
        }
    }

    pub fn to_rust_str(&self) -> &'static str {
        match self {
            Type::Bool => "bool",
            Type::I8 => "i8",
            Type::I16 => "i16",
            Type::I32 => "i32",
            Type::I64 => "i64",
            Type::U8 => "u8",
            Type::U16 => "u16",
            Type::U32 => "u32",
            Type::U64 => "u64",
            Type::F32 => "f32",
            Type::F64 => "f64",
            Type::String => "String",
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
        )
    }

    pub fn is_signed(&self) -> bool {
        matches!(self, Type::I8 | Type::I16 | Type::I32 | Type::I64)
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(self, Type::U8 | Type::U16 | Type::U32 | Type::U64)
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::F32
                | Type::F64
        )
    }

    /// Returns the wider of two same-family integer types.
    /// Panics if types are from different families (signed vs unsigned).
    pub fn widen_int(a: &Type, b: &Type) -> Type {
        let rank = |t: &Type| -> u8 {
            match t {
                Type::I8 | Type::U8 => 0,
                Type::I16 | Type::U16 => 1,
                Type::I32 | Type::U32 => 2,
                Type::I64 | Type::U64 => 3,
                _ => 255,
            }
        };
        if rank(a) >= rank(b) {
            a.clone()
        } else {
            b.clone()
        }
    }
}
