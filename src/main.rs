use std::fs;
use std::process;

use rbasic::{analyze, generate_rust, lex, Parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: rbasic <command> <file.rbas> [output.rs]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  check  <file>           Lex, parse and semantic analysis only");
        eprintln!("  build  <file> [output]   Generate Rust code (stdout if no output)");
        eprintln!("  run    <file>            Build and execute immediately");
        process::exit(1);
    }

    let command = &args[1];
    let input_path = &args[2];

    let source = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", input_path, e);
            process::exit(1);
        }
    };

    match command.as_str() {
        "check" => cmd_check(&source, input_path),
        "build" => {
            let output = compile_to_rust(&source, input_path);
            if args.len() > 3 {
                if let Err(e) = fs::write(&args[3], &output) {
                    eprintln!("Error writing {}: {}", args[3], e);
                    process::exit(1);
                }
            } else {
                print!("{}", output);
            }
        }
        "run" => cmd_run(&source, input_path),
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}

fn compile_to_rust(source: &str, path: &str) -> String {
    let tokens = lex(source);
    let mut parser = Parser::new(tokens);
    let prog = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "{}:{}:{}: Parse error: {}",
                path, e.span.start, e.span.end, e.message
            );
            process::exit(1);
        }
    };
    if let Err(errors) = analyze(&prog) {
        for err in &errors {
            let span = err
                .span
                .map(|(s, e)| format!(":{}:{}", s, e))
                .unwrap_or_default();
            eprintln!("{}{}: {:?} — {}", path, span, err.code, err.message);
        }
        process::exit(1);
    }
    generate_rust(&prog)
}

fn cmd_check(source: &str, path: &str) {
    let _ = compile_to_rust(source, path);
    println!("Check passed: {} is valid RBASIC", path);
}

fn cmd_run(source: &str, path: &str) {
    let rust_code = compile_to_rust(source, path);

    let tmp_dir = std::env::temp_dir();
    let rs_path = tmp_dir.join("rbasic_output.rs");
    let bin_path = tmp_dir.join(if cfg!(windows) {
        "rbasic_output.exe"
    } else {
        "rbasic_output"
    });

    if let Err(e) = fs::write(&rs_path, &rust_code) {
        eprintln!("Error writing temporary file: {}", e);
        process::exit(1);
    }

    let status = process::Command::new("rustc")
        .arg(&rs_path)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to invoke rustc: {}", e);
            eprintln!("Make sure rustc is installed and in your PATH");
            process::exit(1);
        });

    if !status.success() {
        let _ = fs::remove_file(&rs_path);
        let _ = fs::remove_file(&bin_path);
        process::exit(status.code().unwrap_or(1));
    }

    let status = process::Command::new(&bin_path)
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Failed to run compiled binary");
            process::exit(1);
        });

    // Cleanup temp files
    let _ = fs::remove_file(&rs_path);
    let _ = fs::remove_file(&bin_path);

    process::exit(status.code().unwrap_or(0));
}
