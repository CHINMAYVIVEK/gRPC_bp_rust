use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // List of proto files to compile with their respective module paths
    let proto_files = vec![
        ("src/proto/healthcheck.proto", "health"), // (proto file, module name)
                                                   // Add additional proto files here as needed, e.g.:
                                                   // ("src/proto/another_service.proto", "another_service"),
    ];

    // Compile each proto file
    for (proto, module) in &proto_files {
        tonic_build::compile_protos(proto).expect("Failed to compile proto file");

        // Define the output path for the generated files based on the module name
        let out_dir = env::var("OUT_DIR").unwrap();
        let module_out_path = PathBuf::from(format!("src/app/{}", module));

        // Create the destination directory if it doesn't exist
        fs::create_dir_all(&module_out_path).unwrap();

        // Move generated files to the respective module directory
        for entry in fs::read_dir(&out_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map(|ext| ext == "rs").unwrap_or(false) {
                let dest = module_out_path.join(path.file_name().unwrap());
                fs::copy(&path, &dest).unwrap();
            }
        }
    }
}
