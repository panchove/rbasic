# RBASIC

## Intent Document

### Version 0.1

### 1. Introduction

RBASIC is a project to design and develop a modern programming language inspired by the simplicity of BASIC, incorporating contemporary principles of safety, performance, maintainability, and native compilation.

**QuickBASIC is the source of truth** for the language features that RBASIC aims to implement. All syntax, semantics, and control flow constructs are defined by QuickBASIC as the canonical reference. RBASIC extends this foundation with modern safety features, but the core language behavior must remain faithful to QuickBASIC.

The project aims to revive the readability and ease of learning that characterized classic BASIC, eliminating limitations that historically prevented adoption in modern software systems.

RBASIC is not intended to be a direct recreation of classic BASIC, but a modern reinterpretation focused on building safe, maintainable, and efficient software.

---

### 2. Vision

Create a programming language that combines:

- The readability of BASIC.
- The safety of Rust.
- The operational simplicity of Go.
- The hardware proximity of C.
- The productivity of modern languages.

RBASIC should evolve into a self‑contained language capable of compiling its own compiler.

---

### 3. Strategic Objectives

#### Main Objective

Develop a complete ecosystem composed of:

- RBASIC language.
- RBASIC compiler.
- Development tools.
- Package system.
- Standard library.
- Official documentation.

#### Technical Objectives

- Native compilation.
- Static typing.
- Memory safety (stack + ARC, no garbage collector).
- Explicit error handling.
- Cross‑platform portability.
- Interoperability with C.
- Future WebAssembly support.
- Future self‑hosting capability.

---

### 4. Core Principles

#### 4.1 Default Security

All RBASIC programs must be secure by default. Programmers must explicitly opt into unsafe operations via controlled mechanisms.

#### 4.2 Readability First

Code must be understandable before being clever. Language constructs favor clarity over complexity.

#### 4.3 Evolutionary Simplicity

The core language must remain small and stable. Advanced features build on a minimal, consistent foundation.

#### 4.4 Zero Cost When Possible

Language abstractions should not incur unnecessary runtime penalties.

---

### 5. Lexical Specification

- **Encoding:** UTF‑8.
- **File extension:** `.rbas`.
- **Case sensitivity:** Keywords are case‑insensitive; identifiers are case‑preserving.
- **Comments:** Single‑line comments start with `'`.
- **Identifiers:** `[A-Za-z_][A-Za-z0-9_]*[$]?` — the `$` suffix is allowed for classic BASIC string function names (e.g., `MID$`, `LEFT$`).

---

### 6. Language Features

Features are organized in two categories: those inherited from QuickBASIC and classic BASIC, and modern features introduced by RBASIC.

#### 6.1 QuickBASIC-Compatible Features

These features exist in QuickBASIC and are preserved as‑is in RBASIC. QuickBASIC is the canonical reference for syntax and semantics.

##### Variables

```basic
LET name = "RBASIC"
counter = 42
```

`LET` is optional. Variables are declared with `DIM`; without `DIM`, they are implicitly declared on first use.

##### Option Explicit

```basic
OPTION EXPLICIT
' All variables must be declared with DIM before use
DIM x AS INTEGER
x = 10
```

When `OPTION EXPLICIT` is active, using an undeclared variable produces a compile error.

##### Functions

```basic
FUNCTION add(a AS INTEGER, b AS INTEGER) AS INTEGER
    add = a + b
END FUNCTION

' BYVAL — pass by value (copy)
FUNCTION DoubleIt(BYVAL x AS INTEGER) AS INTEGER
    DoubleIt = x * 2
END FUNCTION

' BYREF — pass by reference (original variable affected)
FUNCTION Increment(BYREF x AS INTEGER) AS INTEGER
    x = x + 1
    Increment = x
END FUNCTION

' OPTIONAL — parameter with default value
FUNCTION Power(base AS INTEGER, OPTIONAL exp AS INTEGER = 2) AS INTEGER
    ' ...
END FUNCTION
```

##### Subroutines

```basic
SUB Greet(name AS STRING)
    PRINT "Hello, " + name
END SUB

SUB Swap(BYREF a AS INTEGER, BYREF b AS INTEGER)
    DIM temp AS INTEGER
    temp = a
    a = b
    b = temp
END SUB

CALL Greet("World")
```

- `BYVAL` — parameter is passed by value (a copy is made).
- `BYREF` — parameter is passed by reference (the original variable is modified).
- `OPTIONAL` — parameter may be omitted; a default value must be provided.

##### Module Visibility — PUBLIC / PRIVATE

```basic
' PUBLIC — visible from other modules (default)
PUBLIC SUB Exported()
    PRINT "Hello from module"
END SUB

' PRIVATE — only visible within this module
PRIVATE SUB Internal()
    PRINT "Internal use only"
END SUB
```

In QuickBASIC, `PUBLIC`/`PRIVATE` control whether a `SUB` or `FUNCTION` is accessible from other modules in a multi‑module program.

##### User-Defined Types — TYPE

```basic
TYPE Point
    x AS SINGLE
    y AS SINGLE
END TYPE

DIM p AS Point
p.x = 10.5
p.y = 20.3

' Nested types
TYPE Rect
    origin AS Point
    width AS SINGLE
    height AS SINGLE
END TYPE
```

- `TYPE...END TYPE` defines a composite type (like a C struct).
- Fields are accessed with dot notation (`p.x`).
- Types can be nested.

In QuickBASIC, functions return values by assigning to the function name. There is no `RETURN` statement inside functions or subroutines — `RETURN` is only used to exit a `GOSUB`.

##### Print

```basic
PRINT "Hello, World!"
PRINT x + y
```

##### Input

```basic
INPUT "Enter value: ", x
```

##### Control Flow — IF…THEN…ELSE

```basic
IF condition THEN
    ' body
ELSEIF other THEN
    ' alternative
ELSE
    ' default
END IF
```

##### Control Flow — SELECT CASE

```basic
SELECT CASE variable
    CASE 1
        ' do something
    CASE 2 TO 5
        ' another case
    CASE ELSE
        ' default
END SELECT
```

##### Control Flow — WHILE / WEND

```basic
WHILE condition
    ' body
WEND
```

##### Control Flow — FOR…NEXT…STEP

```basic
FOR i = 1 TO 10
    PRINT i
NEXT i

FOR i = 0 TO 20 STEP 2
    PRINT i
NEXT i
```

##### Control Flow — DO/LOOP

Four variants: `DO WHILE...LOOP`, `DO UNTIL...LOOP`, `DO...LOOP WHILE`, `DO...LOOP UNTIL`.

```basic
DO WHILE condition
    ' body
LOOP

DO UNTIL finished
    ' body
LOOP

DO
    ' body
LOOP WHILE condition

DO
    ' body
LOOP UNTIL done
```

##### Control Flow — EXIT

```basic
FOR i = 1 TO 100
    IF i > 50 THEN EXIT FOR
NEXT i

WHILE running
    IF time > limit THEN EXIT WHILE
WEND

DO
    IF error THEN EXIT DO
LOOP
```

##### Control Flow — GOTO / GOSUB

```basic
GOTO label1

label1:
    PRINT "Jumped here"

GOSUB SubRoutine
PRINT "Back from subroutine"

SubRoutine:
    PRINT "Inside subroutine"
RETURN
```

##### DO EVENTS

```basic
' Cooperative multitasking — yield control to the OS
DO
    ' process data
    DO EVENTS
LOOP

' Keep application responsive while processing
WHILE running
    ' handle input
    DO EVENTS
WEND
```

- `DO EVENTS` yields control to the operating system to process pending events.
- Used for cooperative multitasking in Windows/DOS environments.

##### Control Flow — ON…GOTO / ON…GOSUB

```basic
ON choice GOTO Option1, Option2, Option3

Option1:
    PRINT "Option 1 selected"
END
```

##### Arrays — DIM

```basic
' Default: 0-based (indices 0 to 10, 11 elements)
DIM arr(10)

' Explicit bounds
DIM a(0 TO 10)     ' indices 0 to 10
DIM b(1 TO 10)     ' indices 1 to 10

' Multi-dimensional
DIM matrix(5, 5)        ' 6x6 matrix (0‑5, 0‑5)
DIM grid(1 TO 3, 1 TO 4) ' 3x4 grid (1‑3, 1‑4)

' Multiple declarations
DIM a(100), b(200)

' OPTION BASE 1 — changes default lower bound to 1
OPTION BASE 1
DIM c(10)          ' now indices 1 to 10 (10 elements)
```

- Default lower bound is `0` (like C, Rust, Go).
- `OPTION BASE 1` switches default to `1` (QuickBASIC legacy mode).
- `TO` keyword specifies explicit bounds.

##### Error Handling — ON ERROR / RESUME

```basic
ON ERROR GOTO ErrorHandler
' ... code ...
RESUME

ErrorHandler:
    PRINT "Error occurred"
RESUME

ON ERROR GOTO RecoveryHandler
' ... code ...

RecoveryHandler:
    PRINT "Handling error, resuming at specific label"
RESUME NextLine

NextLine:
    PRINT "Continued after error"
```

- `ON ERROR GOTO label` — sets the error handler to a label.
- `RESUME` — transfers control back to the line that caused the error.
- `RESUME label` — transfers control to the specified label after error handling.

##### Standard Library Functions

All standard functions from QuickBASIC are included in RBASIC.

###### String Functions

| Function | Args | Return | Description |
|----------|------|--------|-------------|
| `LEN` | s | `I32` | Length of string |
| `MID$` | s, start, len | `STRING` | Extract substring |
| `LEFT$` | s, n | `STRING` | Leftmost n characters |
| `RIGHT$` | s, n | `STRING` | Rightmost n characters |
| `CHR$` | n | `STRING` | Character from ASCII/Unicode code |
| `ASC` | s | `I32` | ASCII code of first character |
| `INSTR` | [start,] s, search | `I32` | Find substring position (1‑based) |
| `VAL` | s | `F64` | Convert string to number |
| `STR$` | n | `STRING` | Convert number to string |
| `UCASE$` | s | `STRING` | Convert to uppercase |
| `LCASE$` | s | `STRING` | Convert to lowercase |
| `TRIM$` | s | `STRING` | Remove leading/trailing whitespace |
| `LTRIM$` | s | `STRING` | Remove leading whitespace |
| `RTRIM$` | s | `STRING` | Remove trailing whitespace |
| `SPACE$` | n | `STRING` | String of n spaces |
| `STRING$` | n, ch | `STRING` | String of n repeated characters |

###### Math Functions

| Function | Args | Return | Description |
|----------|------|--------|-------------|
| `ABS` | x | same | Absolute value |
| `SQR` | x | `F64` | Square root |
| `SIN` | x | `F64` | Sine (radians) |
| `COS` | x | `F64` | Cosine (radians) |
| `TAN` | x | `F64` | Tangent (radians) |
| `ATN` | x | `F64` | Arctangent (radians) |
| `LOG` | x | `F64` | Natural logarithm |
| `EXP` | x | `F64` | e raised to power x |
| `INT` | x | same | Integer part (floor) |
| `FIX` | x | same | Integer part (truncate) |
| `SGN` | x | same | Sign: ‑1, 0, or 1 |
| `RND` | [n] | `F64` | Random number (0‑1) |
| `ROUND` | x, n | same | Round to n decimal places |

###### Type Conversion Functions

| Function | Args | Return | Description |
|----------|------|--------|-------------|
| `CINT` | x | `I32` | Convert to INTEGER |
| `CLNG` | x | `I64` | Convert to LONG |
| `CSNG` | x | `F32` | Convert to SINGLE |
| `CDBL` | x | `F64` | Convert to DOUBLE |
| `CSTR` | x | `STRING` | Convert to STRING |
| `CBOOL` | x | `BOOL` | Convert to BOOLEAN |
| `CVI` | s | `I32` | Convert raw bytes to INTEGER |
| `CVS` | s | `F32` | Convert raw bytes to SINGLE |
| `CVD` | s | `F64` | Convert raw bytes to DOUBLE |

###### File I/O Functions

| Statement | Description |
|-----------|-------------|
| `OPEN` | Open a file for reading/writing |
| `CLOSE` | Close a file |
| `INPUT#` | Read from file |
| `PRINT#` | Write to file |
| `LINE INPUT#` | Read a full line from file |
| `EOF` | Test for end of file |
| `LOF` | Length of file |
| `FREEFILE` | Get next available file number |
| `SEEK` | Set file position |
| `MKI$`, `MKS$`, `MKD$` | Convert number to raw bytes |
| `FILEATTR` | Get file attributes |

###### Memory Functions

| Function | Args | Return | Description |
|----------|------|--------|-------------|
| `PEEK` | addr | `I8` | Read byte from memory address |
| `POKE` | addr, val | — | Write byte to memory address |

###### System Functions

| Function | Args | Return | Description |
|----------|------|--------|-------------|
| `TIMER` | — | `F64` | Seconds since midnight |
| `DATE$` | — | `STRING` | Current date (MM‑DD‑YYYY) |
| `TIME$` | — | `STRING` | Current time (HH:MM:SS) |
| `ENVIRON$` | — | `STRING` | Environment variables |
| `COMMAND$` | — | `STRING` | Command‑line arguments |
| `SHELL` | cmd | — | Execute shell command |
| `SYSTEM` | — | — | Exit program |
| `END` | — | — | Terminate program |

##### Operators (Classic)

- Arithmetic: `+`, `-`, `*`, `/`, `^` (power), `\` (integer division), `MOD`
- Bitwise: `SHL`, `SHR`
- Logical: `AND`, `OR`, `XOR`, `NOT`
- Relational: `=`, `<>`, `<`, `>`, `<=`, `>=`

##### String Identifiers

Identifiers may end with `$` for classic BASIC string function names (e.g., `MID$`, `LEFT$`).

##### String Handling (Classic)

```basic
' Variable-length string
DIM name AS STRING
name = "RBASIC"

' Fixed-length string
DIM code AS STRING * 10
code = "ABC"

' Concatenation
DIM full AS STRING
full = "Hello" + " " + "World"
full = "Hello" & " " & "World"    ' & also works

' Comparison
IF name = "RBASIC" THEN PRINT "Match"
IF name <> "OTHER" THEN PRINT "Different"

' Empty string
DIM empty AS STRING
empty = ""
```

- `+` and `&` concatenate strings.
- Comparison operators work on strings (lexicographic order).
- Fixed‑length strings are padded with spaces.

---

#### 6.2 Modern Features

These features go beyond QuickBASIC, bringing safety, expressiveness, and modern programming concepts to RBASIC. They extend the QuickBASIC foundation without breaking compatibility.

##### Mutable Variables — LET MUT

QuickBASIC variables are immutable by default. RBASIC adds `LET MUT` to explicitly opt into mutability:

```basic
LET MUT counter = 0
counter = counter + 1
```

##### Option Implicit (Modern Inverse)

In QuickBASIC, `OPTION EXPLICIT` enforces variable declaration. RBASIC inverts this: **all variables must be declared by default**. Use `OPTION IMPLICIT` to opt into the classic QuickBASIC behavior:

```basic
OPTION IMPLICIT
' Variables are implicitly declared on first use (QuickBASIC legacy mode)
x = 10
```

Without `OPTION IMPLICIT`, using an undeclared variable produces a compile error.

##### Typed Function Declarations

Explicit parameter and return type annotations using `:` syntax. Functions can use `RETURN` to exit early and return a value (not available in QuickBASIC):

```basic
FUNCTION add(a: i32, b: i32) RETURNS i32
    RETURN a + b
END FUNCTION

FUNCTION classify(x: i32) RETURNS string
    IF x < 0 THEN RETURN "negative"
    IF x = 0 THEN RETURN "zero"
    RETURN "positive"
END FUNCTION
```

Parameter passing modes use the same keywords but with modern `:` syntax:

```basic
' BYVAL — pass by value (copy)
FUNCTION DoubleIt(BYVAL x: i32) RETURNS i32
    RETURN x * 2
END FUNCTION

' BYREF — pass by reference (original variable affected)
FUNCTION Increment(BYREF x: i32) RETURNS i32
    x = x + 1
    RETURN x
END FUNCTION

' OPTIONAL — parameter with default value
FUNCTION Power(base: i32, OPTIONAL exp: i32 = 2) RETURNS i32
    ' ...
END FUNCTION

' SUB with BYREF
SUB Swap(BYREF a: i32, BYREF b: i32)
    DIM temp: i32 = a
    a = b
    b = temp
END SUB
```

##### Expanded Primitive Types

Full set of sized integer and floating‑point types beyond QuickBASIC's limited set:

- `BOOL`
- `I8`, `I16`, `I32`, `I64`
- `U8`, `U16`, `U32`, `U64`
- `F32`, `F64`
- `STRING`

##### Type Aliases

Classic BASIC names mapped to canonical RBASIC types:

- `BOOLEAN` → `BOOL`, `BYTE` → `U8`, `WORD` → `U16`, `DWORD` → `U32`, `QWORD` → `U64`
- `INTEGER` → `I32`, `LONG` → `I64`, `LONGLONG` → `I64`
- `SINGLE` → `F32`, `DOUBLE` → `F64`

##### Memory Safety

RBASIC uses a hybrid memory model: stack for primitives, heap with ARC for reference types. No garbage collector, no borrow checker.

###### Value Types (stack, copy semantics)

| Type | Size | Rust equivalent |
|------|------|-----------------|
| `BOOL` | 1 byte | `bool` |
| `I8` / `U8` | 1 byte | `i8` / `u8` |
| `I16` / `U16` | 2 bytes | `i16` / `u16` |
| `I32` / `U32` | 4 bytes | `i32` / `u32` |
| `I64` / `U64` | 8 bytes | `i64` / `u64` |
| `F32` | 4 bytes | `f32` |
| `F64` | 8 bytes | `f64` |

Assignment copies the value:

```basic
DIM a: i32 = 10
DIM b: i32 = a    ' b is a copy; changing b does not affect a
```

###### Reference Types (heap, ARC + Weak)

| Type | Rust equivalent | Notes |
|------|-----------------|-------|
| `STRING` | `Rc<String>` | ARC; clone increments refcount |
| Arrays | `Rc<Vec<Value>>` | ARC; clone increments refcount |
| `Ref<T>` | `Rc<T>` | Strong reference; keeps object alive |
| `Weak<T>` | `Weak<T>` | Weak reference; does not keep object alive |

Assignment shares the heap allocation:

```basic
DIM s: string = "hello"
DIM t: string = s    ' s and t share heap; refcount = 2
```

Weak references break cycles:

```basic
DIM parent: Ref<Node> = Node()
DIM child: Ref<Node> = Node()
child.parent = Weak(parent)    ' does not increment refcount

' Check if object is still alive
IF child.parent.IsSome() THEN
    DIM p = child.parent.Unwrap()
END IF
```

###### Lifecycle

- Entering scope allocates.
- Leaving scope runs `Drop`, which decrements the reference count.
- When refcount reaches 0, heap memory is freed.
- `Weak` references do not prevent deallocation.
- Deterministic cleanup, no stop‑the‑world pauses.

##### Compound Assignment Operators

```basic
x += 5
y -= 2
z *= 3
w /= 4
v ^= 2
u \= 3
```

##### Explicit Type Cast — AS

```basic
LET x: f64 = 3.14
LET y: i32 = x AS i32
```

##### String Handling (Modern)

```basic
' Modern declaration with : syntax
DIM name: string = "RBASIC"

' String interpolation
DIM age: i32 = 25
DIM msg: string = f"Hello, {name}! Age: {age}"

' Raw strings (no escape processing)
DIM path: string = raw"C:\Users\test\file.txt"

' Multi-line strings
DIM poem: string = "
Roses are red,
Violets are blue,
RBASIC is safe,
And fun to use.
"

' Type-safe concatenation (only string + string allowed)
DIM a: string = "Hello"
DIM b: string = " World"
DIM c: string = a + b

' Comparison
IF name = "RBASIC" THEN PRINT "Match"
IF name <> "OTHER" THEN PRINT "Different"
```

- `f"..."` — string interpolation with embedded expressions.
- `raw"..."` — raw string literal (no escape sequences processed).
- Multi‑line strings use double‑quote delimiters across lines.
- String concatenation with `+` is type‑safe (no implicit number-to-string conversion).

##### Arrays (Modern)

```basic
' Modern declaration with explicit type
DIM arr: array<i32, 11>     ' 11 elements, type i32
DIM matrix: array<f64, 6, 6> ' 6x6 matrix

' Array with initialization (inferred type)
DIM primes = {2, 3, 5, 7, 11, 13}

' Array with explicit type
DIM primes: array<i32> = {2, 3, 5, 7, 11, 13}

' Fixed-size array (stack-allocated)
DIM buffer: [u8; 256]

' Slicing
DIM sub: array<i32> = arr[2 TO 5]
```

- `array<T, N>` — generic array syntax with element type and size.
- `[T; N]` — fixed‑size array (stack‑allocated, like Rust).
- Array literals use `{}` for initialization.
- Type can be inferred from the initializer when unambiguous.
- Slicing with `TO` extracts sub‑arrays.

##### Control Flow — CONTINUE

Not present in QuickBASIC. RBASIC adds `CONTINUE` for all loop types:

```basic
FOR i = 1 TO 10
    IF i MOD 2 = 0 THEN CONTINUE FOR
    PRINT i
NEXT i

WHILE more
    IF skip THEN CONTINUE WHILE
    process()
WEND
```

##### Control Flow — FOR EACH

Not present in QuickBASIC. Iterates over collections:

```basic
FOR EACH item IN collection
    PRINT item
END FOR
```

##### Safe References

```basic
Ref<T>     ' Strong reference (ARC, keeps object alive)
Weak<T>    ' Weak reference (does not keep object alive, breaks cycles)
```

- `Ref<T>` — strong reference, increments refcount on copy.
- `Weak<T>` — weak reference, does not increment refcount. Use `IsSome()` to check if the object is still alive, `Unwrap()` to access it.

##### C Interop — DECLARE LIBRARY

RBASIC can call native C functions using `DECLARE LIBRARY`. This extends QuickBASIC's `DECLARE LIBRARY` (used for VBX/DLLs) to full C interop:

```basic
' Static linking — resolved at compile time
DECLARE LIBRARY "libc"
    FUNCTION printf(fmt: pointer, ...) RETURNS i32
    FUNCTION malloc(size: u64) RETURNS pointer
    FUNCTION free(ptr: pointer)
END DECLARE

' Dynamic loading — resolved at runtime via dlopen/LoadLibrary
DECLARE DYNAMIC LIBRARY "libm"
    FUNCTION sqrt(x: f64) RETURNS f64
    FUNCTION pow(base: f64, exp: f64) RETURNS f64
END DECLARE

' Usage
printf("Result: %f\n", sqrt(2.0))
```

- `DECLARE LIBRARY "name"` — links against the library at compile time.
- `DECLARE DYNAMIC LIBRARY "name"` — loads the library at runtime.
- Parameters use RBASIC types mapped to C types: `i32` → `int`, `f64` → `double`, `pointer` → `void*`, `string` → `const char*`.

##### Async / Concurrency

Not present in QuickBASIC. RBASIC adds async primitives inspired by Rust and Go:

```basic
' ASYNC — launch a task and get a future
ASYNC result = FetchData(url)
DIM value = AWAIT result

' Multiple concurrent tasks
ASYNC t1 = FetchData(url1)
ASYNC t2 = FetchData(url2)
DIM v1 = AWAIT t1
DIM v2 = AWAIT t2

' GO — fire-and-forget (like Go goroutines)
GO FetchData(url)
GO ProcessLog(entry)
```

- `ASYNC task = expr` — launches a concurrent task, returns a handle.
- `AWAIT task` — waits for the task to complete and returns its value.
- `GO expr` — spawns a fire‑and‑forget task (like Go's `go` keyword).

##### Module Visibility — PRIVATE / PUBLIC (Extended)

QuickBASIC limits `PUBLIC`/`PRIVATE` to `SUB`/`FUNCTION`. RBASIC extends this to `STRUCT` fields and `MODULE` exports:

```basic
' STRUCT — field-level visibility
TYPE Point
    PUBLIC x: f64
    PUBLIC y: f64
    PRIVATE internal_id: i32
END TYPE

' MODULE — control what is exported
MODULE MathLib
    PUBLIC FUNCTION Add(a: i32, b: i32) RETURNS i32
        RETURN a + b
    END FUNCTION

    PRIVATE FUNCTION Validate(x: i32) RETURNS bool
        RETURN x >= 0
    END FUNCTION
END MODULE
```

- `PUBLIC` — accessible from outside the struct/module.
- `PRIVATE` — only accessible within the struct/module.

##### Modules

Not present in QuickBASIC. RBASIC adds a module system for organizing code into separate files and namespaces:

```basic
' File: math.rbas
MODULE MathLib

    PUBLIC FUNCTION Add(a: i32, b: i32) RETURNS i32
        RETURN a + b
    END FUNCTION

    PUBLIC FUNCTION Subtract(a: i32, b: i32) RETURNS i32
        RETURN a - b
    END FUNCTION

    PRIVATE FUNCTION Validate(x: i32) RETURNS bool
        RETURN x >= 0
    END FUNCTION

END MODULE
```

```basic
' File: main.rbas
IMPORT MathLib

DIM result: i32 = MathLib.Add(10, 5)
PRINT result
```

- Each `.bas` file is a module (file-as-module).
- `MODULE name...END MODULE` defines the module boundary.
- `IMPORT module` brings a module into scope.
- `PUBLIC` items are accessible from importing modules.
- `PRIVATE` items are only accessible within the module.

##### User-Defined Types — TYPE (Modern)

QuickBASIC uses `AS` for field types. RBASIC uses `:` syntax:

```basic
TYPE Point
    x: f64
    y: f64
END TYPE

DIM p: Point
p.x = 10.5
p.y = 20.3

' TYPE with default values
TYPE Config
    width: i32 = 800
    height: i32 = 600
    title: string = "RBASIC"
END TYPE
```

##### Enums — ENUM

Not present in QuickBASIC. RBASIC adds `ENUM` for named integer constants:

```basic
ENUM Direction
    UP
    DOWN
    LEFT
    RIGHT
END ENUM

DIM dir: Direction = Direction.UP

SELECT CASE dir
    CASE Direction.UP
        PRINT "Moving up"
    CASE Direction.DOWN
        PRINT "Moving down"
END SELECT

' Explicit values
ENUM Status
    OK = 0
    WARNING = 1
    ERROR_CODE = 2
    CRITICAL = 3
END ENUM
```

- Members are implicitly typed as `I32`.
- Default values start at 0 and auto‑increment.
- Explicit values can be assigned.

##### Optional Values (Future)

```basic
Optional<T>
```

##### Result and Error Types (Future)

```basic
Result<T, E>
```

---

### 7. Initial Exclusions

To keep the scope controlled, the first version will NOT include:

- Classic inheritance.
- Complex classes.
- Garbage collection.
- Advanced macros.
- Reflection.
- Metaprogramming.
- Advanced concurrency.
- Complex generics.
- Optional\<T\>, Result\<T,E\> (require generics).

These features will be evaluated later.

---

### 8. Compiler Architecture

#### Initial Phase

The first compiler will be written in Rust.

Architecture diagram:

```text
Lexer
  ↓
Parser
  ↓
AST
  ↓
Semantic Analysis
  ↓
Typed AST
  ↓
Code Generation
```

#### Initial Backend

RBASIC → Rust → rustc (LLVM)

This strategy allows:

- Leveraging LLVM optimization via rustc.
- Keeping the main compiler in a single language (Rust).
- Easier bootstrapping to rewrite codegen in RBASIC.
- Portability to all platforms supported by Rust.

#### Compilation Modes

RBASIC supports compilation for different target architectures:

- **32‑bit** (`--target i686`) — generates 32‑bit executables. `INTEGER` is 32‑bit, pointers are 32‑bit.
- **64‑bit** (`--target x86_64`) — default. Generates 64‑bit executables. `INTEGER` is 32‑bit, pointers are 64‑bit.

Type sizes remain consistent across architectures:

| Type | 32‑bit | 64‑bit |
|------|--------|--------|
| `I8` / `U8` | 1 byte | 1 byte |
| `I16` / `U16` | 2 bytes | 2 bytes |
| `I32` / `U32` | 4 bytes | 4 bytes |
| `I64` / `U64` | 8 bytes | 8 bytes |
| `F32` | 4 bytes | 4 bytes |
| `F64` | 8 bytes | 8 bytes |
| Pointers | 4 bytes | 8 bytes |
| `INTEGER` | 32‑bit | 32‑bit |

---

### 9. Implementation Order

RFCs and features are implemented in three phases: first the QuickBASIC‑compatible core, then modern extensions, then future features.

#### Phase 1 — QuickBASIC Core

The foundation. Implements the classic language as defined by QuickBASIC.

| # | Feature | RFC |
|---|---------|-----|
| 1 | Vision and Objectives | RFC-0001 |
| 2 | Lexical Specification | RFC-0002 |
| 3 | MVP Definition | RFC-0003 |
| 4 | Grammar Specification | RFC-0004 |
| 5 | AST Specification | RFC-0005 |
| 6 | Semantic Analysis | RFC-0006 |
| 7 | Type Compatibility | RFC-0007 |
| 8 | Type Checking | RFC-0008 |
| 9 | Variables (`LET`, standalone assignment) | RFC-0015 |
| 10 | `PRINT` | — |
| 11 | `INPUT` | RFC-0019 |
| 12 | `IF…THEN…ELSE` | — |
| 13 | `WHILE…WEND` | — |
| 14 | `FOR…NEXT…STEP` | RFC-0009 |
| 15 | `DO/LOOP` (4 variants) | RFC-0010 |
| 16 | `SELECT CASE` | — |
| 17 | `GOTO` / `GOSUB` | — |
| 18 | `ON…GOTO` / `ON…GOSUB` | — |
| 19 | `FUNCTION` (assign to name, no `RETURN`) | — |
| 20 | `SUB` / `CALL` | — |
| 21 | `BYVAL` / `BYREF` / `OPTIONAL` | — |
| 22 | `DIM` (arrays) | RFC-0012 |
| 23 | `TYPE…END TYPE` | — |
| 24 | `PUBLIC` / `PRIVATE` | — |
| 25 | `ON ERROR` / `RESUME` | RFC-0013 |
| 26 | `OPTION EXPLICIT` | — |
| 27 | String handling (classic) | — |
| 28 | Standard library (string, math, conversion, file I/O, memory, system) | RFC-0017 |
| 29 | Type Aliases | RFC-0011 |
| 30 | Memory management (stack + ARC) | RFC-0014 |

#### Phase 2 — Modern Extensions

Extends QuickBASIC with safety, expressiveness, and modern features.

| # | Feature | RFC |
|---|---------|-----|
| 31 | `LET MUT` (explicit mutability) | — |
| 32 | Typed declarations with `:` and `RETURNS` | — |
| 33 | `RETURN` in functions | — |
| 34 | Expanded primitive types (I8–I64, U8–U64, F32, F64) | — |
| 35 | Compound assignment (`+=`, `-=`, etc.) | RFC-0018 |
| 36 | `AS` type cast | — |
| 37 | `OPTION IMPLICIT` | — |
| 38 | `CONTINUE FOR/WHILE/DO` | — |
| 39 | `FOR EACH` | — |
| 40 | String interpolation (`f"..."`) / raw strings | — |
| 41 | Modern arrays (`array<T, N>`, inference, `[T; N]`) | — |
| 42 | `ENUM` | — |
| 43 | `MODULE` | — |
| 44 | `DECLARE LIBRARY` (C interop) | — |
| 45 | `PRIVATE`/`PUBLIC` extended (STRUCT, MODULE) | — |
| 46 | `Ref<T>` / `Weak<T>` (ARC + Weak memory model) | RFC-0014 |

#### Phase 3 — Future Features

Requires generics.

| # | Feature | RFC |
|---|---------|-----|
| 47 | `Optional<T>` | — |
| 48 | `Result<T, E>` | — |

---

### 10. Evolution Strategy

#### Stage 1

- Minimum viable RBASIC (Phase 1 complete).
- QuickBASIC-compatible core fully functional.

#### Stage 2

- Phase 2 features begin (modern extensions).
- Basic standard library.

#### Stage 3

- Phase 2 complete.
- Compiler capable of processing real projects.

#### Stage 4

- Progressive rewrite of the compiler in RBASIC.
- Requires Phase 1 + Phase 2 (file I/O, strings, arrays, TYPE, MODULE).

#### Stage 5

- Self‑compilation (self‑hosting).
- RBASIC compiler compiles itself.

#### Stage 6

- Native backend based on LLVM or Cranelift.

#### Stage 7

- Phase 3 features (generics, Optional, Result).

---

### 11. Long‑Term Goal

Make RBASIC a modern, safe, and sustainable language usable for:

- Console applications.
- Automation tools.
- Embedded systems.
- Backend services.
- Cross‑platform applications.
- Compilers and development tools.
- Office suite automation via **RBA (RBasic for Applications)**, a modern replacement for VBA in LibreOffice, FreeOffice, and OnlyOffice.

#### RBA — RBasic for Applications

RBA will be an embedded variant of RBASIC designed to serve as a scripting engine in open‑source office suites:

- **LibreOffice** — integration via UNO API.
- **FreeOffice** — integration via native API.
- **OnlyOffice** — integration via plugin/scripting mechanisms.

RBA will share the same core language as RBASIC but include a standard library oriented toward manipulating documents, spreadsheets, presentations, and office automation tasks. The goal is to provide a modern, secure, cross‑platform alternative to VBA, eliminating dependence on Windows and Microsoft Office.

---

### 12. RBScript

**RBScript** is a scripting language inspired by VBScript, designed to provide automation and scripting capabilities within the RBASIC ecosystem. It will share the same core language as RBASIC and RBA, targeting environments where a lightweight, embeddable scripting engine is needed.

---

### 13. Final Declaration

RBASIC is born with the intention of proving that a language can be simultaneously simple to read, safe to execute, and powerful enough to build modern systems. Simplicity will remain a permanent feature, not a temporary limitation.
