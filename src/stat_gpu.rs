use std::process::Command;

pub fn name() -> String {
    let output = Command::new("lspci")
        .output()
        .expect("Failed to run pciutils");

    let out = String::from_utf8_lossy(&output.stdout);

    let mut name: String = "unknown".to_string();
    for l in out.lines() {
        if l.contains("VGA compatible controller") || l.contains("3D controller") {
            if let Some(n) = l.split(":").nth(2) {
                name = n.to_string().trim().to_string();
            }
        }
    }

    name
}
