use glib_build_tools::compile_resources;
use std::process::Command;

fn main() {
    // Recompile when Blueprint files change
    println!("cargo:rerun-if-changed=ui/");
    println!("cargo:rerun-if-changed=../../data/");

    // Compile Blueprint files to UI files
    let blueprint_files = ["ui/window.blp"];
    for blueprint_file in &blueprint_files {
        if std::path::Path::new(blueprint_file).exists() {
            let ui_file = blueprint_file.replace(".blp", ".ui");
            println!("Compiling {} to {}", blueprint_file, ui_file);

            let output = Command::new("blueprint-compiler")
                .arg("compile")
                .arg("--output")
                .arg(&ui_file)
                .arg(blueprint_file)
                .output()
                .expect("Failed to execute blueprint-compiler");

            if !output.status.success() {
                panic!(
                    "Blueprint compilation failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    // Compile GResources if the file exists
    let resources_path = "../../data/resources.gresource.xml";
    if std::path::Path::new(resources_path).exists() {
        compile_resources(&["../../data"], resources_path, "resources.gresource");
    }
}
