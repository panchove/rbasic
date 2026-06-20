# RFC-0042: File I/O (QuickBASIC Classic)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

# 1. Summary

Implement QuickBASIC-style file I/O for sequential, random, and binary file access. QuickBASIC is the source of truth for this feature.

---

# 2. Syntax

## 2.1 OPEN Statement

```ebnf
open_stmt ::= "OPEN" expression "FOR" mode ("AS" "#" expression ("LEN" "=" expression)?)?
mode      ::= "INPUT" | "OUTPUT" | "APPEND" | "RANDOM" | "BINARY"
```

Examples:
```basic
OPEN "data.txt" FOR INPUT AS #1
OPEN "output.txt" FOR OUTPUT AS #2
OPEN "data.dat" FOR RANDOM AS #3 LEN = 128
OPEN "image.bin" FOR BINARY AS #4
```

## 2.2 CLOSE Statement

```ebnf
close_stmt ::= "CLOSE" ("#" expression ("," "#" expression)*)?
```

Examples:
```basic
CLOSE #1
CLOSE #1, #2, #3
CLOSE
```

## 2.3 INPUT# Statement

```ebnf
input_hash_stmt ::= "INPUT" "#" expression "," identifier ("," identifier)*
```

Reads values from a file handle into variables.

Examples:
```basic
INPUT #1, name$
INPUT #1, id, score
```

## 2.4 PRINT# Statement

```ebnf
print_hash_stmt ::= "PRINT" "#" expression ("," | ";") expression (("," | ";") expression)*
```

Writes expressions to a file handle. `,` adds space, `;` adds nothing, newline at end unless trailing separator.

Examples:
```basic
PRINT #1, "Hello"
PRINT #1, id; " "; name$
PRINT #1, a, b, c
```

## 2.5 LINE INPUT# Statement

```ebnf
line_input_hash_stmt ::= "LINE" "INPUT" "#" expression "," identifier
```

Reads an entire line from a file into a string variable.

Examples:
```basic
LINE INPUT #1, line$
```

## 2.6 File Functions

```ebnf
func_call ::= "EOF" "(" expression ")"
           | "LOF" "(" expression ")"
           | "FREEFILE"
           | "SEEK" "(" expression ")"
           | "FILEATTR" "(" expression "," expression ")"
           | "MKI$" "(" expression ")"
           | "MKS$" "(" expression ")"
           | "MKD$" "(" expression ")"
           | "CVI" "(" expression ")"
           | "CVS" "(" expression ")"
           | "CVD" "(" expression ")"
```

---

# 3. Semantics

## 3.1 File Handles

- File handles are integer values (1-255 in classic QuickBASIC, unlimited in RBASIC).
- `FREEFILE` returns the next available file handle.
- Handles are automatically assigned if `AS #n` is omitted.

## 3.2 File Modes

| Mode | Description | File must exist | Position |
|------|-------------|-----------------|----------|
| INPUT | Read only | Yes | Beginning |
| OUTPUT | Write only (truncates) | No (created if missing) | Beginning |
| APPEND | Write only (adds to end) | No (created if missing) | End |
| RANDOM | Read/write (record-based) | No (created if missing) | Beginning |
| BINARY | Read/write (byte-level) | No (created if missing) | Beginning |

## 3.3 EOF function

Returns `-1` (true) if the file pointer is at end-of-file, `0` (false) otherwise.

## 3.4 LOF function

Returns the length of the file in bytes.

## 3.5 FREEFILE function

Returns the next available file handle number.

## 3.6 SEEK function

Returns the current position in the file (1-based byte offset).

## 3.7 FILEATTR function

Returns file attribute information. Mode 1 returns file mode, mode 2 returns file size.

## 3.8 Record Functions

- `MKI$(n)` — Convert I32 to 2-byte string (little-endian)
- `MKS$(n)` — Convert F32 to 4-byte string (little-endian)
- `MKD$(n)` — Convert F64 to 8-byte string (little-endian)
- `CVI(s)` — Convert 2-byte string to I32
- `CVS(s)` — Convert 4-byte string to F32
- `CVD(s)` — Convert 8-byte string to F64

---

# 4. Code Generation (Rust)

## 4.1 Runtime Module

The file I/O operations map to Rust's `std::fs::File` and `std::io` traits:

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write, Seek, SeekFrom};

struct FileHandle {
    reader: Option<BufReader<File>>,
    writer: Option<BufWriter<File>>,
}
```

## 4.2 Generated Code Examples

```basic
OPEN "data.txt" FOR INPUT AS #1
```
→
```rust
let file_handle_1 = FileHandle::open("data.txt", FileMode::Input)?;
```

```basic
PRINT #1, "Hello"
```
→
```rust
file_handle_1.write_line("Hello")?;
```

```basic
INPUT #1, name$
```
→
```rust
name = file_handle_1.read_line()?;
```

---

# 5. Error Codes

| Code  | Description                          |
|-------|--------------------------------------|
| E1050 | File not found                       |
| E1051 | File already open                    |
| E1052 | Invalid file handle                  |
| E1053 | File not open                        |
| E1054 | Input past end of file               |
| E1055 | Permission denied                    |
| E1056 | Disk full                            |
| E1057 | Bad file name or number              |
| E1058 | File already exists                  |

---

# 6. Acceptance Criteria

```
✓ OPEN with all 5 modes parsed and codegen'd correctly
✓ CLOSE with single, multiple, and no handles
✓ INPUT# reads values from file
✓ PRINT# writes values to file
✓ LINE INPUT# reads entire line
✓ EOF() returns correct end-of-file status
✓ LOF() returns file length
✓ FREEFILE returns next available handle
✓ SEEK() returns current position
✓ MKI$/MKS$/MKD$ and CVI/CVS/CVD convert numeric types
✓ Random access with LEN clause
✓ Binary access
✓ Error handling for file operations
✓ Tests for all operations
```

---

# 7. References

- QuickBASIC 4.5 Language Reference: OPEN, CLOSE, INPUT#, PRINT#, LINE INPUT#, EOF, LOF, FREEFILE, SEEK, FILEATTR, MKI$, MKS$, MKD$, CVI, CVS, CVD
