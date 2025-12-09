use std::process::Command;

#[test]
fn test_help_short_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_dtren"))
        .arg("-h")
        .output()
        .expect("Failed to execute dtren");

    assert_eq!(output.status.code(), Some(1));
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("dtren v"));
    assert!(stderr.contains("Usage: dtren <file>"));
    assert!(stderr.contains("Rename a file by appending its modified timestamp to the name."));
}

#[test]
fn test_help_long_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_dtren"))
        .arg("--help")
        .output()
        .expect("Failed to execute dtren");

    assert_eq!(output.status.code(), Some(1));
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("dtren v"));
    assert!(stderr.contains("Usage: dtren <file>"));
    assert!(stderr.contains("Rename a file by appending its modified timestamp to the name."));
}
