# RBASIC Diagnostic Registry

This document is the authoritative registry for all compiler diagnostics (error codes and warnings). Every diagnostic produced by the RBASIC compiler must be registered here with a unique code, description, and cross-reference to the defining RFC.

## Code Range Allocation

All diagnostics use the format `E` followed by a four‑digit number. The range `E1000–E1089` is allocated for v0.x. Codes outside this range are reserved for future versions.

| Range         | Category              | RFC          | Status |
|---------------|-----------------------|--------------|--------|
| E1000–E1019   | Resolution Errors     | RFC-0006     | Active |
| E1020–E1029   | Type Compatibility    | RFC-0007     | Active |
| E1030–E1039   | Type Checking         | RFC-0008     | Active |
| E1040–E1049   | Assignment System     | RFC-0015     | Active |
| E1050–E1059   | Loop Validation       | _Unallocated_ | Free  |
| E1060–E1069   | Arrays                | _Unallocated_ | Free  |
| E1070–E1079   | Runtime               | _Unallocated_ | Free  |
| E1080–E1089   | Code Generation       | _Unallocated_ | Free  |

## Registered Diagnostics

### E1000–E1019: Resolution Errors

| Code  | Description                                         | RFC       |
|-------|-----------------------------------------------------|-----------|
| E1001 | Unknown variable (not declared in any visible scope)| RFC-0006  |
| E1002 | Duplicate variable (already declared in scope)      | RFC-0006  |
| E1003 | Unknown function (call to undeclared function)      | RFC-0006  |
| E1004 | Duplicate function (redeclared at global scope)     | RFC-0006  |
| —     | (unused)                                            |           |
| E1010 | Unknown type (type annotation not recognized)       | RFC-0006  |
| E1011 | Duplicate parameter (same name in parameter list)   | RFC-0006  |

**Free slots (9):** E1005–E1009, E1012–E1015

### E1020–E1029: Type Compatibility

| Code  | Description                                         | RFC       |
|-------|-----------------------------------------------------|-----------|
| E1020 | Type mismatch (expression does not match expected)  | RFC-0007  |
| E1021 | Invalid binary operation (incompatible operands)    | RFC-0007  |
| E1022 | Invalid unary operation (incompatible operand)      | RFC-0007  |

**Free slots (7):** E1023–E1029

### E1030–E1039: Type Checking

| Code  | Description                                         | RFC       |
|-------|-----------------------------------------------------|-----------|
| E1030 | Argument count mismatch                             | RFC-0008  |
| E1031 | Return type mismatch                                | RFC-0008  |
| E1032 | Invalid condition type (expected BOOL)              | RFC-0008  |
| E1033 | Return outside function body                        | RFC-0008  |
| E1034 | Non-numeric step value (FOR…STEP)                   | RFC-0008  |

**Free slots (5):** E1035–E1039

### E1040–E1049: Assignment System

| Code  | Description                                         | RFC       |
|-------|-----------------------------------------------------|-----------|
| E1040 | Assignment to undeclared variable                   | RFC-0015  |
| E1041 | Type mismatch in assignment                         | RFC-0015  |
| E1042 | Assignment to immutable variable                    | RFC-0015  |

**Free slots (7):** E1043–E1049. Reserved for compound assignment operators (RFC-0018).

### E1050–E1059: Loop Validation (Unallocated)

**Free slots (10).** Reserved for future loop‑specific diagnostics (e.g., loop variable shadowing, invalid bounds).

### E1060–E1069: Arrays (Unallocated)

**Free slots (10).** Reserved for array‑specific diagnostics (RFC-0012/RFC-0016):

| Reserved | Description                             | RFC |
|----------|-----------------------------------------|-----|
| E1060    | Index out of bounds                     | —   |
| E1061    | Invalid index type (must be integer)    | —   |
| E1062    | Dimension mismatch                      | —   |

### E1070–E1079: Runtime (Unallocated)

**Free slots (10).** Reserved for runtime error codes emitted by the RBA Runtime.

### E1080–E1089: Code Generation (Unallocated)

**Free slots (10).** Reserved for codegen‑failure diagnostics (e.g., unsupported feature, target emission error).

## Allocation Rules

1. **No gaps**: When assigning a new code within an active block, use the lowest‑available free slot.
2. **No cross‑block migration**: A diagnostic's purpose determines its block. Do not move a diagnostic from one block to another without updating this registry.
3. **Every code must link to an RFC**: Each code must appear in exactly one RFC's diagnostic table. The RFC is the source of truth for the diagnostic's meaning and trigger conditions.
4. **Unused codes may be reassigned**: An unallocated slot within a block may be assigned to any new diagnostic that fits the block's category. If a block is full, the next available block must be used and the allocation table above must be updated.
5. **Allocation changes require a registry update**: Any addition, removal, or reassignment of codes must be reflected here, in the relevant RFC, and in the CHANGELOG.

## Appendix: RFC Cross‑Reference

| RFC    | Defines diagnostics   |
|--------|-----------------------|
| 0006   | E1001–E1004, E1010–E1011 |
| 0007   | E1020–E1022           |
| 0008   | E1030–E1034           |
| 0015   | E1040–E1042           |
| (0018) | E1043–E1049 (planned) |
