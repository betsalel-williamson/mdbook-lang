use std::vec::Vec;
use std::fs::{self, File};
use std::io::Write;

pub use crate::common::{prepare, exe_command, remove_dir_from_error};

pub fn build_pkl(code_block: String, sandbox_args_vec: Vec<String>) -> String {
    let (dir, source_file, _output_file) = prepare(
        std::env::temp_dir().to_str().unwrap().to_string(),
        "input.pkl".to_string(),
        "output.txt".to_string() // Output file is not directly used for execution, but prepare requires it
    );

    let mut source = File::create_new(source_file.clone()).unwrap();

    // Write the source code into file
    let _r = source.write_all(code_block.as_bytes());
    let _r = source.flush();

    // Execute the Pkl code using 'pkl eval'
    let mut command_args = vec![
        "eval".to_string(),
        source_file.as_path().to_str().unwrap().to_string()
    ];

    // Prepend sandbox arguments if provided
    let cmd = if sandbox_args_vec.is_empty() {
        "pkl".to_string()
    } else {
        let mut args = sandbox_args_vec.clone();
        let actual_cmd = args.remove(0);
        command_args.splice(0..0, args); // Insert sandbox args at the beginning
        actual_cmd
    };

    let result = exe_command(cmd, command_args);

    // Remove temporary directory from error messages for cleaner output
    let result = remove_dir_from_error(&result, "input.pkl".to_string());

    // Clean up the temporary directory
    let _r = fs::remove_dir_all(dir.clone().as_path());

    result
}

#[test]
fn build_pkl_test(){
    let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block=r#"// This is an example of a Pkl code block.
key = "value""#;
    let result = build_pkl(code_block.to_string(), sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("key = \"value\"\n"));
}