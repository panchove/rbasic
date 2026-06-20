use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Input,
    Output,
    Append,
    Random,
    Binary,
}

struct FileHandle {
    file: File,
    mode: FileMode,
}

lazy_static::lazy_static! {
    static ref FILE_HANDLES: Mutex<HashMap<i64, FileHandle>> = Mutex::new(HashMap::new());
}

pub fn open(filename: &str, mode: FileMode, _record_len: Option<i64>) -> Result<i64, String> {
    let free = freefile();
    let file = match mode {
        FileMode::Input => File::open(filename).map_err(|e| format!("E1050: {}", e))?,
        FileMode::Output | FileMode::Append => std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(mode == FileMode::Output)
            .append(mode == FileMode::Append)
            .open(filename)
            .map_err(|e| format!("E1055: {}", e))?,
        FileMode::Random | FileMode::Binary => std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(filename)
            .map_err(|e| format!("E1055: {}", e))?,
    };

    let handle = FileHandle { file, mode };

    let mut handles = FILE_HANDLES
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    handles.insert(free, handle);
    Ok(free)
}

pub fn close(handle: i64) {
    let mut handles = FILE_HANDLES.lock().unwrap();
    handles.remove(&handle);
}

pub fn close_all() {
    let mut handles = FILE_HANDLES.lock().unwrap();
    handles.clear();
}

pub fn freefile() -> i64 {
    let handles = FILE_HANDLES.lock().unwrap();
    let mut n = 1;
    while handles.contains_key(&n) {
        n += 1;
    }
    n
}

pub fn input_hash(handle: i64) -> Result<String, String> {
    let mut handles = FILE_HANDLES
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    let file_handle = handles
        .get_mut(&handle)
        .ok_or("E1052: Invalid file handle")?;
    let mut reader = BufReader::new(
        file_handle
            .file
            .try_clone()
            .map_err(|e| format!("E1050: {}", e))?,
    );
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .map_err(|e| format!("E1054: {}", e))?;
    if line.is_empty() {
        return Err("E1054: Input past end of file".to_string());
    }
    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    Ok(line)
}

pub fn print_hash(
    handle: i64,
    format_str: &str,
    args: &[&dyn std::fmt::Display],
) -> Result<(), String> {
    let mut handles = FILE_HANDLES
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    let file_handle = handles
        .get_mut(&handle)
        .ok_or("E1052: Invalid file handle")?;
    let mut writer = BufWriter::new(
        file_handle
            .file
            .try_clone()
            .map_err(|e| format!("E1050: {}", e))?,
    );

    // Simple format replacement
    let mut result = format_str.to_string();
    for arg in args {
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos + 2, &arg.to_string());
        }
    }
    writeln!(writer, "{}", result).map_err(|e| format!("E1055: {}", e))?;
    Ok(())
}

pub fn line_input_hash(handle: i64) -> Result<String, String> {
    input_hash(handle)
}

pub fn eof(handle: i64) -> bool {
    let handles = FILE_HANDLES.lock().unwrap();
    if let Some(file_handle) = handles.get(&handle) {
        let mut clone = file_handle.file.try_clone().unwrap();
        let mut buf = [0u8; 1];
        match clone.read(&mut buf) {
            Ok(0) => true,
            Ok(_) => false,
            Err(_) => true,
        }
    } else {
        true
    }
}

pub fn lof(handle: i64) -> i64 {
    let handles = FILE_HANDLES.lock().unwrap();
    if let Some(file_handle) = handles.get(&handle) {
        let mut clone = file_handle.file.try_clone().unwrap();
        let pos = clone.stream_position().unwrap_or(0);
        clone.seek(SeekFrom::End(0)).unwrap_or(0);
        let len = clone.stream_position().unwrap_or(0);
        clone.seek(SeekFrom::Start(pos)).unwrap_or(0);
        len as i64
    } else {
        0
    }
}

pub fn seek(handle: i64) -> i64 {
    let handles = FILE_HANDLES.lock().unwrap();
    if let Some(file_handle) = handles.get(&handle) {
        let mut clone = file_handle.file.try_clone().unwrap();
        clone.stream_position().unwrap_or(0) as i64
    } else {
        0
    }
}

pub fn fileattr(handle: i64, mode: i64) -> i64 {
    let handles = FILE_HANDLES.lock().unwrap();
    if let Some(file_handle) = handles.get(&handle) {
        match mode {
            1 => match file_handle.mode {
                FileMode::Input => 1,
                FileMode::Output => 2,
                FileMode::Append => 4,
                FileMode::Random => 8,
                FileMode::Binary => 32,
            },
            2 => lof(handle),
            _ => 0,
        }
    } else {
        0
    }
}

pub fn mki(value: i64) -> Vec<u8> {
    value.to_le_bytes()[..2].to_vec()
}

pub fn mks(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

pub fn mkd(value: f64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

pub fn cvi(bytes: &[u8]) -> i64 {
    let mut buf = [0u8; 8];
    buf[..2].copy_from_slice(&bytes[..2.min(bytes.len())]);
    i64::from_le_bytes(buf)
}

pub fn cvs(bytes: &[u8]) -> f32 {
    let mut buf = [0u8; 4];
    buf[..4.min(bytes.len())].copy_from_slice(&bytes[..4.min(bytes.len())]);
    f32::from_le_bytes(buf)
}

pub fn cvd(bytes: &[u8]) -> f64 {
    let mut buf = [0u8; 8];
    buf[..8.min(bytes.len())].copy_from_slice(&bytes[..8.min(bytes.len())]);
    f64::from_le_bytes(buf)
}
