use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
        }
}