use shaderc::ShaderKind;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/shaders");

    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));

    std::fs::create_dir_all("assets/shaders")?;

    for entry in std::fs::read_dir("src/shaders")? {
        let dir = entry?;
        if dir.file_type()?.is_file() {
            let path = dir.path();

            let kind = path
                .extension()
                .and_then(|ext| match ext.to_string_lossy().as_ref() {
                    "vert" => Some(ShaderKind::Vertex),
                    "frag" => Some(ShaderKind::Fragment),
                    "geom" => Some(ShaderKind::Geometry),
                    _ => None,
                });

            let source = std::fs::read_to_string(&path)?;

            if let Some(kind) = kind {
                let binary_result = compiler
                    .compile_into_spirv(
                        &source,
                        kind,
                        &path.to_str().unwrap(),
                        "main",
                        Some(&options),
                    )
                    .unwrap();

                let binary_u8 = binary_result.as_binary_u8();

                let name = String::from("assets/shaders/")
                    + &path.file_name().unwrap().to_str().unwrap()
                    + ".spv";
                std::fs::write(name, binary_u8);
            }
        }
    }

    Ok(())
}
