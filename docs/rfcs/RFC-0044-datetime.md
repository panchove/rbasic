# RFC-0044: Date/Time System

Status: Draft
Version: 0.2
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

# 1. Summary

Implement a dual Date/Time system for RBASIC:
- **Phase 1 (Classic)**: VB6-compatible `Date` type (stored as `Double`), standalone functions
- **Phase 2 (Modern)**: `DateTime` type with explicit timezone support, methods, and `TimeSpan` type

VB6 is the source of truth for Phase 1. Phase 2 draws from Go's `time.Time` and Rust's `chrono` crate.

---

# 2. Design Principles

## Phase 1 (Classic - VB6)
- **Double Storage**: Dates stored as `Double` (days since 30/dec/1899, fraction = time)
- **No GC**: Date is a primitive type (F64), no heap allocation
- **Functions, not methods**: VB6-style standalone functions
- **Literal syntax**: `#mm/dd/yyyy#` and `#hh:mm:ss#`

## Phase 2 (Modern)
- **DateTime struct**: Contains UTC timestamp + offset
- **Timezone-aware**: Every DateTime carries its timezone offset
- **Method-based**: OOP-style API with methods
- **Immutable**: DateTime is a value type, methods return new instances
- **UTC-first**: Internal storage is always UTC, local is derived

---

# 3. Phase 1: Classic Date Type

## 3.1 Internal Representation (F64)

VB6 stores dates as `Double`:
- **Integer part**: Days since December 30, 1899 (serial date)
- **Fractional part**: Time of day (0.0 = midnight, 0.5 = noon)

| Value | Meaning |
|-------|---------|
| `0.0` | December 30, 1899, 00:00:00 |
| `1.0` | December 31, 1899, 00:00:00 |
| `46561.0` | June 20, 2026, 00:00:00 |
| `46561.5` | June 20, 2026, 12:00:00 |
| `46561.604166...` | June 20, 2026, 14:30:00 |

## 3.2 Date Literal

```ebnf
date_literal ::= "#" DATE_COMPONENTS "#"
date_components ::= MM "/" DD "/" YYYY (" " HH ":" MM [ ":" SS ])?
```

Examples:
```basic
#12/25/2026#
#06/20/2026 14:30:00#
#12/31/1899 00:00:00#
```

## 3.3 Type Declaration

```basic
DIM d AS Date      ' Equivalent to DIM d AS F64
DIM d AS F64       ' Also works
d = Now
t = DateSerial(2026, 6, 20)
```

## 3.4 Phase 1 API Reference

### Current Date/Time Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `Now` | `() -> Date` | Current date and time |
| `Date` | `() -> Date` | Current date (time = 0) |
| `Time` | `() -> Date` | Current time (date = 0) |
| `Timer` | `() -> F64` | Seconds since midnight |

### Date Construction Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `DateSerial` | `(year: I32, month: I32, day: I32) -> Date` | Create date from components |
| `TimeSerial` | `(hour: I32, minute: I32, second: I32) -> Date` | Create time from components |
| `DateValue` | `(str: String) -> Date` | Parse date string |
| `TimeValue` | `(str: String) -> Date` | Parse time string |

### Component Extraction Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `Year` | `(d: Date) -> I32` | Extract year (1700-9999) |
| `Month` | `(d: Date) -> I32` | Extract month (1-12) |
| `Day` | `(d: Date) -> I32` | Extract day (1-31) |
| `Hour` | `(d: Date) -> I32` | Extract hour (0-23) |
| `Minute` | `(d: Date) -> I32` | Extract minute (0-59) |
| `Second` | `(d: Date) -> I32` | Extract second (0-59) |
| `Weekday` | `(d: Date) -> I32` | Day of week (1=Sunday, 7=Saturday) |

### Date Arithmetic Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `DateAdd` | `(interval: String, number: I32, date: Date) -> Date` | Add interval |
| `DateDiff` | `(interval: String, date1: Date, date2: Date) -> I64` | Difference between dates |
| `DatePart` | `(interval: String, date: Date) -> I32` | Extract part of date |

**Interval codes:**

| Code | Interval |
|------|----------|
| `"yyyy"` | Year |
| `"q"` | Quarter |
| `"m"` | Month |
| `"y"` | Day of year |
| `"d"` | Day |
| `"w"` | Weekday |
| `"ww"` | Week |
| `"h"` | Hour |
| `"n"` | Minute |
| `"s"` | Second |

### Formatting Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `Format` | `(date: Date, fmt: String) -> String` | Format date as string |

**Format codes:**

| Code | Description | Example |
|------|-------------|---------|
| `"yyyy"` | 4-digit year | `"2026"` |
| `"yy"` | 2-digit year | `"26"` |
| `"mm"` | Month (01-12) | `"06"` |
| `"m"` | Month (no leading zero) | `"6"` |
| `"dd"` | Day (01-31) | `"20"` |
| `"d"` | Day (no leading zero) | `"20"` |
| `"hh"` | Hour (00-23) | `"14"` |
| `"nn"` | Minute (00-59) | `"30"` |
| `"ss"` | Second (00-59) | `"00"` |
| `"tt"` | AM/PM | `"PM"` |
| `"w"` | Day of week | `"7"` |
| `"ww"` | Week of year | `"25"` |

### Conversion Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `CDate` | `(value: any) -> Date` | Convert to Date |
| `IsDate` | `(value: any) -> Bool` | Check if convertible to Date |
| `Date$` | `() -> String` | Current date as "mm-dd-yyyy" |
| `Time$` | `() -> String` | Current time as "hh:mm:ss" |

---

# 4. Phase 2: Modern DateTime Type

## 4.1 Internal Representation

`DateTime` is a struct with three fields:

```basic
TYPE DateTime
    utc_secs: I64       ' Seconds since Unix epoch (1970-01-01 00:00:00 UTC)
    utc_nanos: I32      ' Nanoseconds (0-999999999)
    offset_mins: I16    ' UTC offset in minutes (-720 to +720)
END TYPE
```

**Internal storage is always UTC.** Local time is derived by adding `offset_mins`.

## 4.2 TimeSpan Type

```basic
TYPE TimeSpan
    secs: I64           ' Total seconds
    nanos: I32          ' Nanoseconds
END TYPE
```

## 4.3 Timezone Constants

```basic
CONST UTC AS I16 = 0
CONST LOCAL AS I16 = -32768  ' Sentinel: use system timezone
```

---

# 5. Phase 2: DateTime API

## 5.1 Constructors

| Method | Signature | Description |
|--------|-----------|-------------|
| `DateTime.Now` | `() -> DateTime` | Local time (system timezone) |
| `DateTime.UtcNow` | `() -> DateTime` | UTC time |
| `DateTime.FromComponents` | `(y, m, d, h, n, s, offH, offM) -> DateTime` | Create with offset |
| `DateTime.FromUnixSeconds` | `(ts: I64) -> DateTime` | From Unix timestamp (UTC) |
| `DateTime.FromUnixMillis` | `(ts: I64) -> DateTime` | From milliseconds (UTC) |
| `DateTime.FromDate` | `(d: Date) -> DateTime` | Convert classic Date (local) |
| `DateTime.ParseISO8601` | `(s: String) -> Result<DateTime, ParseError>` | Parse ISO 8601 string |
| `DateTime.Parse` | `(s, fmt) -> Result<DateTime, ParseError>` | Parse with format |

## 5.2 Conversion Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `ToUTC` | `() -> DateTime` | Convert to UTC (offset = 0) |
| `ToLocal` | `() -> DateTime` | Convert to system timezone |
| `ToOffset` | `(hours: I32, minutes: I32) -> DateTime` | Convert to offset |
| `ToDate` | `() -> Date` | Convert to classic F64 (local) |
| `ToUnixSeconds` | `() -> I64` | Unix timestamp |
| `ToUnixMillis` | `() -> I64` | Unix timestamp (ms) |
| `ToISO8601` | `() -> String` | ISO 8601 format |

## 5.3 Component Extraction Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Year` | `() -> I32` | Year (local time) |
| `Month` | `() -> I32` | Month (1-12, local) |
| `Day` | `() -> I32` | Day (1-31, local) |
| `Hour` | `() -> I32` | Hour (0-23, local) |
| `Minute` | `() -> I32` | Minute (0-59, local) |
| `Second` | `() -> I32` | Second (0-59, local) |
| `Weekday` | `() -> I32` | Day of week (1=Sunday, 7=Saturday) |
| `Offset` | `() -> TimeSpan` | UTC offset |
| `OffsetHours` | `() -> I32` | Offset hours component |
| `OffsetMinutes` | `() -> I32` | Offset minutes component |
| `IsUTC` | `() -> Bool` | Is offset zero? |
| `IsDST` | `() -> Bool` | Is in daylight saving time? |

## 5.4 Arithmetic Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Add` | `(ts: TimeSpan) -> DateTime` | Add time span |
| `AddDays` | `(n: I64) -> DateTime` | Add days |
| `AddHours` | `(n: I64) -> DateTime` | Add hours |
| `AddMinutes` | `(n: I64) -> DateTime` | Add minutes |
| `AddSeconds` | `(n: I64) -> DateTime` | Add seconds |
| `AddMonths` | `(n: I32) -> DateTime` | Add months |
| `AddYears` | `(n: I32) -> DateTime` | Add years |
| `Subtract` | `(other: DateTime) -> TimeSpan` | Difference |
| `Duration` | `() -> TimeSpan` | Absolute duration (if negative) |

## 5.5 Comparison Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Before` | `(other: DateTime) -> Bool` | Is before? |
| `After` | `(other: DateTime) -> Bool` | Is after? |
| `Equals` | `(other: DateTime) -> Bool` | Same instant? |
| `CompareTo` | `(other: DateTime) -> I32` | -1, 0, or 1 |

**Comparison is always by UTC instant**, regardless of offset.

## 5.6 Formatting Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `Format` | `(fmt: String) -> String` | Custom format |
| `FormatLocal` | `(fmt: String) -> String` | Format in local time |
| `FormatUTC` | `(fmt: String) -> String` | Format in UTC |

**Format codes:**

| Code | Description | Example |
|------|-------------|---------|
| `"yyyy"` | 4-digit year | `"2026"` |
| `"mm"` | Month (01-12) | `"06"` |
| `"dd"` | Day (01-31) | `"20"` |
| `"hh"` | Hour (00-23) | `"14"` |
| `"nn"` | Minute (00-59) | `"30"` |
| `"ss"` | Second (00-59) | `"00"` |
| `"zzz"` | UTC offset | `"-03:00"` |
| `"Z"` | UTC indicator | `"Z"` |

## 5.7 TimeSpan Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `TotalDays` | `() -> F64` | Days as float |
| `TotalHours` | `() -> F64` | Hours as float |
| `TotalMinutes` | `() -> F64` | Minutes as float |
| `TotalSeconds` | `() -> I64` | Seconds as integer |
| `Days` | `() -> I32` | Whole days |
| `Hours` | `() -> I32` | Remaining hours |
| `Minutes` | `() -> I32` | Remaining minutes |
| `Seconds` | `() -> I32` | Remaining seconds |
| `IsNegative` | `() -> Bool` | Is negative? |
| `IsZero` | `() -> Bool` | Is zero? |
| `Negate` | `() -> TimeSpan` | Negate |
| `Add` | `(other: TimeSpan) -> TimeSpan` | Add |
| `Subtract` | `(other: TimeSpan) -> TimeSpan` | Subtract |

---

# 6. Examples

## 6.1 Classic Date (Phase 1)

```basic
PRINT Now              ' 46561.604166...
PRINT Date$            ' "06-20-2026"
PRINT Time$            ' "14:30:00"
PRINT Year(Now)        ' 2026

DIM d AS Date = DateSerial(2026, 12, 25)
PRINT Format(d, "mmmm d, yyyy")  ' "December 25, 2026"

DIM d2 AS Date = DateAdd("d", 30, d)
DIM diff AS I64 = DateDiff("d", Date, d2)
```

## 6.2 Modern DateTime (Phase 2)

```basic
' Current time in different zones
DIM utc AS DateTime = DateTime.UtcNow
DIM local AS DateTime = DateTime.Now
DIM tokyo AS DateTime = utc.ToOffset(9, 0)

PRINT utc.Format("yyyy-mm-ddThh:nn:ssZ")    ' "2026-06-20T14:30:00Z"
PRINT local.Format("yyyy-mm-dd hh:nn:ss zzz") ' "2026-06-20 14:30:00 -03:00"
PRINT tokyo.Format("yyyy-mm-dd hh:nn:ss zzz") ' "2026-06-20 23:30:00 +09:00"
```

## 6.3 Timezone Conversions

```basic
DIM meeting_utc AS DateTime = DateTime.FromComponents(2026, 6, 20, 18, 0, 0, 0, 0)
DIM meeting_nyc AS DateTime = meeting_utc.ToOffset(-5, 0)
DIM meeting_tokyo AS DateTime = meeting_utc.ToOffset(9, 0)

PRINT meeting_nyc.Format("hh:nn tt")    ' "01:00 PM"
PRINT meeting_tokyo.Format("hh:nn tt")  ' "03:00 AM"
```

## 6.4 Duration and Comparison

```basic
DIM start AS DateTime = DateTime.UtcNow
' ... some work ...
DIM end AS DateTime = DateTime.UtcNow

DIM elapsed AS TimeSpan = end.Subtract(start)
PRINT "Elapsed: "; elapsed.TotalSeconds(); " seconds"

IF start.Before(end) THEN
    PRINT "Start is before end"
END IF

DIM diff AS TimeSpan = DateTime.UtcNow.Subtract(
    DateTime.FromUnixSeconds(0)
)
PRINT "Days since epoch: "; diff.Days()
```

## 6.5 Unix Timestamps

```basic
DIM ts AS I64 = DateTime.UtcNow.ToUnixSeconds()
PRINT "Unix timestamp: "; ts

DIM restored AS DateTime = DateTime.FromUnixSeconds(ts)
PRINT "Restored: "; restored.ToISO8601()
```

## 6.6 ISO 8601 Parsing

```basic
DIM result = DateTime.ParseISO8601("2026-06-20T14:30:00Z")
IF result.IsOk() THEN
    DIM dt AS DateTime = result.Unwrap()
    PRINT "Parsed: "; dt.Format("yyyy-mm-dd hh:nn:ss zzz")
END IF
```

## 6.7 Interop with Classic Date

```basic
' Classic to Modern
DIM classic AS Date = Now
DIM modern AS DateTime = DateTime.FromDate(classic)

' Modern to Classic
DIM back AS Date = modern.ToDate()
PRINT Format(back, "yyyy-mm-dd hh:nn:ss")
```

---

# 7. Code Generation (Rust)

## 7.1 Runtime Types

```rust
#[derive(Clone, Copy)]
pub struct DateTime {
    utc_secs: i64,
    utc_nanos: i32,
    offset_mins: i16,
}

#[derive(Clone, Copy)]
pub struct TimeSpan {
    secs: i64,
    nanos: i32,
}

pub type Date = f64;
```

## 7.2 Key Implementations

```rust
impl DateTime {
    pub fn now() -> Self {
        let now = chrono::Local::now();
        let offset = now.offset().local_minus_utc();
        let utc = now.with_timezone(&chrono::Utc);
        Self {
            utc_secs: utc.timestamp(),
            utc_nanos: utc.timestamp_subsec_nanos() as i32,
            offset_mins: (offset / 60) as i16,
        }
    }

    pub fn utc_now() -> Self {
        let now = chrono::Utc::now();
        Self {
            utc_secs: now.timestamp(),
            utc_nanos: now.timestamp_subsec_nanos() as i32,
            offset_mins: 0,
        }
    }

    pub fn to_offset(&self, hours: i32, minutes: i32) -> Self {
        Self {
            utc_secs: self.utc_secs,
            utc_nanos: self.utc_nanos,
            offset_mins: (hours * 60 + minutes) as i16,
        }
    }

    pub fn to_iso8601(&self) -> String {
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(
            self.utc_secs, self.utc_nanos as u32
        ).unwrap();
        dt.to_rfc3339()
    }
}
```

## 7.3 Dependency

```toml
[dependencies]
chrono = "0.4"
```

---

# 8. Acceptance Criteria

## Phase 1 (Classic)
```
✓ Date type defined as F64 alias
✓ Date literal #mm/dd/yyyy# parsed correctly
✓ Date literal #mm/dd/yyyy hh:mm:ss# parsed correctly
✓ Now, Date, Time, Timer functions
✓ DateSerial, TimeSerial constructors
✓ DateValue, TimeValue string parsers
✓ Year, Month, Day, Hour, Minute, Second extraction
✓ Weekday extraction (1=Sunday, 7=Saturday)
✓ DateAdd with all interval codes
✓ DateDiff with all interval codes
✓ DatePart with all interval codes
✓ Format with all format codes
✓ CDate conversion
✓ IsDate validation
✓ Date$, Time$ string functions
✓ Leap year calculation correct
✓ Tests for all functions
```

## Phase 2 (Modern)
```
✓ DateTime struct with utc_secs, utc_nanos, offset_mins
✓ TimeSpan struct
✓ DateTime.Now, DateTime.UtcNow constructors
✓ DateTime.FromComponents with offset
✓ DateTime.FromUnixSeconds/Millis
✓ DateTime.FromDate (classic interop)
✓ DateTime.ParseISO8601
✓ ToUTC, ToLocal, ToOffset conversions
✓ ToDate (classic interop)
✓ ToUnixSeconds/Millis
✓ ToISO8601
✓ Year, Month, Day, Hour, Minute, Second, Weekday
✓ Offset, OffsetHours, OffsetMinutes, IsUTC, IsDST
✓ Add, AddDays, AddHours, AddMinutes, AddSeconds, AddMonths, AddYears
✓ Subtract, Duration
✓ Before, After, Equals, CompareTo (UTC comparison)
✓ Format, FormatLocal, FormatUTC
✓ TimeSpan: TotalDays/Hours/Minutes/Seconds
✓ TimeSpan: Days, Hours, Minutes, Seconds components
✓ TimeSpan: IsNegative, IsZero, Negate, Add, Subtract
✓ Interop: DateTime.FromDate and DateTime.ToDate
✓ Tests for all operations
```

---

# 9. References

- VB6 Language Reference: Date Functions
- Go `time.Time` — timezone-aware, method-based
- Rust `chrono` crate — DateTime with timezone
- ISO 8601 — date/time format standard
- Unix timestamp — seconds since 1970-01-01 UTC
