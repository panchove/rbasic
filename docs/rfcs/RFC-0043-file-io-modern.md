# RFC-0043: Modern File I/O

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

# 1. Summary

Define a modern, type-safe file I/O API for RBASIC using first-class `File` objects with methods, `Result<T, E>` error handling, and ARC memory management. This is the Phase 2 counterpart to RFC-0042 (Classic QuickBASIC File I/O).

---

# 2. Design Principles

- **Object-oriented**: `File` is a first-class type with methods
- **Type-safe**: Explicit return types, no magic handle numbers
- **Error handling**: `Result<T, IOError>` for all fallible operations
- **RAII**: Files close automatically via ARC when going out of scope
- **Iterator-based**: Line-by-line reading via `FOR EACH`
- **Binary support**: Typed read/write for numeric types

---

# 3. Types

## 3.1 FileMode Enum

```basic
ENUM FileMode
    Input
    Output
    Append
    Random
    Binary
END ENUM
```

## 3.2 SeekOrigin Enum

```basic
ENUM SeekOrigin
    Start
    Current
    End
END ENUM
```

## 3.3 IOError Type

```basic
TYPE IOError
    code: I32
    message: String
END TYPE
```

## 3.4 File Type

```basic
TYPE File
    path: String
    mode: FileMode
    handle: Ref<InternalHandle>
END TYPE
```

The `InternalHandle` is opaque and managed by the runtime. The `Ref<InternalHandle>` ensures RAII via ARC.

---

# 4. Syntax

## 4.1 File Module

All file operations live in the `File` module:

```basic
IMPORT File
```

## 4.2 Constructors (Static Methods)

```ebnf
file_open    ::= "File" "." "Open" "(" expression "," expression ")"
file_create  ::= "File" "." "Create" "(" expression ")"
file_temp    ::= "File" "." "Temp" "(" ")"
```

Examples:
```basic
DIM f AS File = File.Open("data.txt", FileMode.Input)
DIM g AS File = File.Create("output.txt")
DIM h AS File = File.Temp()
```

## 4.3 Instance Methods

```ebnf
file_method  ::= expression "." IDENTIFIER "(" arguments? ")"
```

Examples:
```basic
f.WriteLine("Hello")
f.WriteI32(42)
DIM line AS String = f.ReadLine()
DIM pos AS I64 = f.Tell()
f.Seek(0, SeekOrigin.Start)
f.Close()
```

## 4.4 Iterator

```basic
FOR EACH line AS String IN File.Lines("data.txt")
    PRINT line
NEXT line
```

---

# 5. API Reference

## 5.1 Constructors

| Method | Signature | Description |
|--------|-----------|-------------|
| `File.Open` | `(path: String, mode: FileMode) -> Result<File, IOError>` | Open existing file |
| `File.Create` | `(path: String) -> Result<File, IOError>` | Create/truncate file |
| `File.Temp` | `() -> Result<File, IOError>` | Create temporary file |
| `File.Lines` | `(path: String) -> Iterator<String>` | Iterate lines |

## 5.2 Read Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `ReadLine` | `() -> Result<String, IOError>` | Read one line |
| `ReadAll` | `() -> Result<String, IOError>` | Read entire file |
| `ReadBytes` | `(n: I32) -> Result<Array<Byte>, IOError>` | Read N bytes |
| `ReadI32` | `() -> Result<I32, IOError>` | Read 4-byte LE integer |
| `ReadF64` | `() -> Result<F64, IOError>` | Read 8-byte LE float |
| `ReadAllBytes` | `() -> Result<Array<Byte>, IOError>` | Read all bytes |

## 5.3 Write Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Write` | `(data: String) -> Result<(), IOError>` | Write string |
| `WriteLine` | `(data: String) -> Result<(), IOError>` | Write string + newline |
| `WriteBytes` | `(data: Array<Byte>) -> Result<(), IOError>` | Write byte array |
| `WriteI32` | `(val: I32) -> Result<(), IOError>` | Write 4-byte LE integer |
| `WriteF64` | `(val: F64) -> Result<(), IOError>` | Write 8-byte LE float |
| `Flush` | `() -> Result<(), IOError>` | Flush buffer |

## 5.4 Navigation Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Seek` | `(pos: I64, origin: SeekOrigin) -> Result<I64, IOError>` | Move position |
| `Tell` | `() -> I64` | Current position |
| `IsEOF` | `() -> Bool` | At end of file? |

## 5.5 Metadata Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Length` | `() -> I64` | File size in bytes |
| `Name` | `() -> String` | File path |
| `IsOpen` | `() -> Bool` | Is file open? |
| `Mode` | `() -> FileMode` | Open mode |

## 5.6 Close Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Close` | `() -> Result<(), IOError>` | Explicit close |
| `Drop` | `()` | ARC destructor (implicit) |

---

# 6. Semantics

## 6.1 RAII and ARC

- `File` holds a `Ref<InternalHandle>` which is reference-counted
- When the last `File` reference goes out of scope, the handle is automatically closed
- Explicit `Close()` is optional but recommended for early release
- Double-close is safe (no-op)

## 6.2 Error Handling

All fallible operations return `Result<T, IOError>`. Callers must handle errors:

```basic
DIM result = File.Open("data.txt", FileMode.Input)
IF result.IsErr() THEN
    PRINT "Failed: "; result.UnwrapErr().message
    RETURN
END IF
DIM f AS File = result.Unwrap()
```

Or using `?` operator (if implemented):

```basic
DIM f AS File = File.Open("data.txt", FileMode.Input)?
```

## 6.3 Binary Mode

In `Binary` mode, `Read`/`Write` operate on raw bytes:

```basic
DIM f AS File = File.Open("image.bin", FileMode.Binary)
DIM header AS Array<Byte, 16>
f.ReadBytes(16) -> header
DIM width AS I32 = f.ReadI32()
```

## 6.4 Random Access

```basic
DIM f AS File = File.Open("data.dat", FileMode.Random)
f.Seek(128, SeekOrigin.Start)
DIM record AS Array<Byte, 64>
f.ReadBytes(64) -> record
```

---

# 7. Code Generation (Rust)

## 7.1 Runtime Module

```rust
pub struct FileHandle {
    path: String,
    mode: FileMode,
    reader: Option<BufReader<File>>,
    writer: Option<BufWriter<File>>,
}

impl FileHandle {
    pub fn open(path: &str, mode: FileMode) -> Result<Self, IOError> { ... }
    pub fn read_line(&mut self) -> Result<String, IOError> { ... }
    pub fn write_line(&mut self, data: &str) -> Result<(), IOError> { ... }
    // ... etc
}
```

## 7.2 Generated Code Examples

```basic
DIM f AS File = File.Open("data.txt", FileMode.Input)
DIM line AS String = f.ReadLine()
f.Close()
```
→
```rust
let f = rbasic::runtime::file::FileHandle::open("data.txt", FileMode::Input)?;
let line = f.read_line()?;
f.close()?;
```

---

# 8. Migration from Classic

| Classic (RFC-0042) | Modern (RFC-0043) |
|---------------------|-------------------|
| `OPEN "f" FOR INPUT AS #1` | `DIM f AS File = File.Open("f", FileMode.Input)` |
| `INPUT #1, var` | `var = f.ReadLine()?` |
| `PRINT #1, expr` | `f.WriteLine(expr)?` |
| `CLOSE #1` | `f.Close()?` |
| `EOF(1)` | `f.IsEOF()` |
| `LOF(1)` | `f.Length()` |
| `SEEK(1)` | `f.Tell()` / `f.Seek(...)` |
| `FREEFILE` | Not needed (automatic) |
| `MKI$/CVI` | `WriteI32` / `ReadI32` |

---

# 9. Acceptance Criteria

```
✓ File type defined with Ref<InternalHandle>
✓ FileMode and SeekOrigin enums defined
✓ IOError type defined
✓ File.Open, File.Create, File.Temp constructors
✓ ReadLine, ReadAll, ReadBytes, ReadI32, ReadF64 methods
✓ Write, WriteLine, WriteBytes, WriteI32, WriteF64 methods
✓ Seek, Tell, IsEOF navigation methods
✓ Length, Name, IsOpen, Mode metadata methods
✓ Close and ARC-based Drop
✓ File.Lines iterator
✓ All methods return Result<T, IOError>
✓ Double-close is safe (no-op)
✓ RAII: file closes when last reference drops
✓ Tests for all operations
✓ Tests for error handling
✓ Tests for RAII/ARC behavior
```

---

# 10. References

- Rust `std::fs::File` — RAII, methods, Result<T, E>
- Go `os.File` — methods, error handling
- Python `io.FileIO` — iterator protocol
- QuickBASIC classic I/O (RFC-0042) — backward compatibility
