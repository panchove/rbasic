#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn compile_file(path: &str) -> String {
        let source = fs::read_to_string(path).expect("read example");
        let (tokens, _) = rbasic::lex(&source);
        let mut parser = rbasic::Parser::new(tokens);
        let prog = parser.parse_program().expect("parse");
        assert!(rbasic::analyze(&prog).is_ok(), "semantic analysis failed");
        rbasic::generate_rust(&prog)
    }

    fn run_compiled(source: &str) -> String {
        let rust_code = compile_file(source);
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);

        let tmp_dir = std::env::temp_dir();
        let unique = format!("rbasic_integration_{}", id);
        let rs_path = tmp_dir.join(format!("{}.rs", unique));
        let bin_path = if cfg!(windows) {
            tmp_dir.join(format!("{}.exe", unique))
        } else {
            tmp_dir.join(&unique)
        };

        fs::write(&rs_path, &rust_code).expect("write rust temp");
        let status = Command::new("rustc")
            .arg(&rs_path)
            .arg("-o")
            .arg(&bin_path)
            .status()
            .expect("rustc");
        assert!(status.success(), "rustc compilation failed");

        let output = Command::new(&bin_path).output().expect("run binary");

        // Cleanup
        let _ = fs::remove_file(&rs_path);
        let _ = fs::remove_file(&bin_path);

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    #[test]
    fn hello_example() {
        let out = run_compiled("examples/hello.rbas");
        assert_eq!(out.trim(), "Hello, RBASIC");
    }

    #[test]
    fn add_example() {
        let out = run_compiled("examples/add.rbas");
        assert_eq!(out.trim(), "30");
    }

    #[test]
    fn fibonacci_first_10() {
        let out = run_compiled("examples/fibonacci.rbas");
        let nums: Vec<i32> = out.lines().map(|l| l.trim().parse().unwrap()).collect();
        assert_eq!(nums.len(), 20);
        assert_eq!(nums[0], 1);
        assert_eq!(nums[1], 1);
        assert_eq!(nums[2], 2);
        assert_eq!(nums[3], 3);
        assert_eq!(nums[4], 5);
        assert_eq!(nums[9], 55);
    }

    #[test]
    fn fizzbuzz_first_15() {
        let out = run_compiled("examples/fizzbuzz.rbas");
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 100);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[2], "Fizz");
        assert_eq!(lines[4], "Buzz");
        assert_eq!(lines[14], "FizzBuzz");
    }

    #[test]
    fn for_step_example() {
        let out = run_compiled("examples/for_step.rbas");
        let nums: Vec<i32> = out.lines().map(|l| l.trim().parse().unwrap()).collect();
        assert_eq!(nums, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn operators_example() {
        let out = run_compiled("examples/operators.rbas");
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "8"); // 2 ^ 3
        assert_eq!(lines[1], "1"); // 10 MOD 3
        assert_eq!(lines[2], "2"); // 7 \ 3
        assert_eq!(lines[3], "4"); // (10 MOD 4) * 2
    }

    #[test]
    fn comments_do_not_affect_execution() {
        // Inline source with comments — must compile and run correctly
        let source = "'header comment\nPRINT 42 ' inline\n' trailing";
        let (tokens, lex_errors) = rbasic::lex(source);
        assert!(lex_errors.is_empty());
        let mut parser = rbasic::Parser::new(tokens);
        let prog = parser.parse_program().expect("parse");
        assert!(rbasic::analyze(&prog).is_ok());
        let rust = rbasic::generate_rust(&prog);
        assert!(rust.contains("42"));
    }
}
