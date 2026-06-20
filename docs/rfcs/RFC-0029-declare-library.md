# RFC-0029: DECLARE LIBRARY (C Interop)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add C foreign function interface (FFI) support to RBASIC via `DECLARE LIBRARY` and `DECLARE DYNAMIC LIBRARY`. This allows RBASIC programs to call native C functions from shared libraries without writing Rust glue code. Static declarations bind at compile time; dynamic declarations bind at runtime via `dlopen`/`LoadLibrary`.

---

## 2. Syntax (EBNF)

```ebnf
declare_lib     ::= "DECLARE" "LIBRARY" string_literal NEWLINE
                    declare_entry+
                    "END" "DECLARE"

declare_dyn_lib ::= "DECLARE" "DYNAMIC" "LIBRARY" string_literal NEWLINE
                    declare_entry+
                    "END" "DECLARE"

declare_entry   ::= "FUNCTION" IDENTIFIER "(" declare_params ")" ["AS" type_ref]
                  | "SUB" IDENTIFIER "(" declare_params ")"

declare_params  ::= [declare_param ("," declare_param)*]

declare_param   ::= IDENTIFIER "AS" type_ref
```

- `DECLARE`, `LIBRARY`, `DYNAMIC`, `FUNCTION`, `SUB`, `END`, `AS` are case-insensitive reserved keywords.
- The string literal after `LIBRARY` names the shared library (e.g., `"libc"`, `"user32"`).
- `DECLARE LIBRARY` binds at compile time (static FFI via `extern`).
- `DECLARE DYNAMIC LIBRARY` binds at runtime (dynamic FFI via `dlopen`).
- Parameter types map to C types (see mapping table below).
- Return types map to C types.

Examples:

```basic
DECLARE LIBRARY "libc"
    FUNCTION printf(fmt AS STRING) AS INTEGER
    FUNCTION malloc(size AS INTEGER) AS INTEGER
    SUB free(ptr AS INTEGER)
END DECLARE

DECLARE DYNAMIC LIBRARY "libcurl"
    FUNCTION curl_easy_init() AS INTEGER
    FUNCTION curl_easy_perform(handle AS INTEGER) AS INTEGER
    SUB curl_easy_cleanup(handle AS INTEGER)
END DECLARE

DIM result AS INTEGER
result = printf("Hello, %s!\n", "world")
```

---

## 3. Semantics

1. `DECLARE LIBRARY "name"` registers a static library binding. The compiler emits `#[link(name = "name")]` in the generated Rust code.
2. `DECLARE DYNAMIC LIBRARY "name"` registers a dynamic library binding. The runtime loads the library via `dlopen` (POSIX) or `LoadLibrary` (Windows).
3. Each `FUNCTION` or `SUB` declaration within the block declares an external function signature.
4. RBASIC types map to C types at the FFI boundary (see mapping table).
5. String parameters are passed as `*const c_char` (static) or via runtime conversion (dynamic).
6. Calling a declared function with wrong argument count emits `E1300`.
7. Calling a declared function with incompatible argument types emits `E1301`.
8. Dynamic library load failure at runtime emits `E1302`.
9. Dynamic function call failure emits `E1303`.

### Type Mapping

| RBASIC Type | C Type          | Rust Type         |
|-------------|-----------------|-------------------|
| INTEGER     | int             | i32               |
| LONG        | long            | i64               |
| SINGLE      | float           | f32               |
| DOUBLE      | double          | f64               |
| STRING      | const char*     | *const c_char     |
| BOOLEAN     | int (0/1)       | bool              |
| any integer | void*           | *mut c_void       |

---

## 4. AST (node definitions)

### DeclareLibrary

```text
DeclareLibrary {
    name:     String,
    dynamic:  bool,
    entries:  Vec<DeclareEntry>,
}

DeclareEntry {
    name:      String,
    params:    Vec<DeclareParam>,
    ret_type:  Option<TypeRef>,
    is_sub:    bool,
}

DeclareParam {
    name: String,
    typ:  TypeRef,
}
```

### CallExternal (Expression)

```text
CallExternal {
    lib_name: String,
    func:     String,
    args:     Vec<Expression>,
}
```

---

## 5. Parsing

When `DECLARE` keyword is encountered:

1. Consume `DECLARE`.
2. Check for optional `DYNAMIC` keyword.
3. Consume `LIBRARY`.
4. Parse the library name (string literal).
5. Parse entry declarations until `END DECLARE`.
6. Consume `END DECLARE`.
7. Produce `Statement::DeclareLibrary { name, dynamic, entries }`.

```rust
fn parse_declare_library() -> Result<DeclareLibrary> {
    consume(Declare);
    let dynamic = if peek() == Dynamic {
        advance();
        true
    } else {
        false
    };
    consume(Library);
    let name = expect_string_literal()?;
    let mut entries = Vec::new();
    loop {
        if peek() == End && peek_ahead() == Declare {
            advance(); // END
            advance(); // DECLARE
            break;
        }
        entries.push(parse_declare_entry()?);
    }
    Ok(DeclareLibrary { name, dynamic, entries })
}
```

---

## 6. Semantic Analysis

1. **Library registration** — `DECLARE LIBRARY "name"` registers the library. Duplicate library names in the same scope emit `E1304`.
2. **Function signature validation** — parameter types must be FFI-compatible. Unsupported types emit `E1305`.
3. **Call validation** — calling a declared function validates argument count (E1300) and types (E1301).
4. **Static vs dynamic** — static libraries are resolved at link time; dynamic libraries are resolved at runtime. The compiler tracks which mode each library uses.
5. **String handling** — string arguments require conversion at the FFI boundary. The compiler inserts appropriate conversion code.

---

## 7. Code Generation

### Static Library (DECLARE LIBRARY)

```basic
DECLARE LIBRARY "libc"
    FUNCTION printf(fmt AS STRING) AS INTEGER
END DECLARE
```

Compiles to:

```rust
extern "C" {
    #[link(name = "libc")]
    fn printf(fmt: *const std::ffi::c_char, ...) -> i32;
}
```

### Dynamic Library (DECLARE DYNAMIC LIBRARY)

```basic
DECLARE DYNAMIC LIBRARY "libcurl"
    FUNCTION curl_easy_init() AS INTEGER
END DECLARE
```

Compiles to runtime loading:

```rust
// Runtime library loading
fn load_library(name: &str) -> Result<LibraryHandle> {
    // dlopen on POSIX, LoadLibrary on Windows
}

fn load_symbol(handle: LibraryHandle, name: &str) -> Result<*const c_void> {
    // dlsym on POSIX, GetProcAddress on Windows
}
```

### Function Call

```basic
DIM result AS INTEGER
result = printf("Hello\n")
```

Compiles to:

```rust
let result: i32 = unsafe {
    let fmt = CString::new("Hello\n").unwrap();
    printf(fmt.as_ptr())
};
```

---

## 8. Error Codes

| Code  | Description                                             |
|-------|---------------------------------------------------------|
| E1300 | Wrong argument count in external function call           |
| E1301 | Type mismatch in external function call                 |
| E1302 | Dynamic library load failure at runtime                 |
| E1303 | Dynamic function symbol lookup failure                  |
| E1304 | Duplicate DECLARE LIBRARY name in same scope            |
| E1305 | Unsupported type in DECLARE LIBRARY signature           |

---

## 9. Acceptance Criteria

```text
✓ DECLARE LIBRARY parsed as DeclareLibrary with dynamic=false
✓ DECLARE DYNAMIC LIBRARY parsed as DeclareLibrary with dynamic=true
✓ FUNCTION entries parsed with name, params, return type
✓ SUB entries parsed with name, params, no return type
✓ Static library compiles to extern "C" with #[link]
✓ Dynamic library compiles to runtime dlopen/dlsym code
✓ String arguments converted at FFI boundary
✓ Wrong argument count produces E1300
✓ Type mismatch produces E1301
✓ Dynamic load failure produces E1302
✓ Full test suite passes
```
