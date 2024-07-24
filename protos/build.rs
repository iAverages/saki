use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", env::var("OUT_DIR")?);
    let og_out_dir = PathBuf::from(env::var("OUT_DIR")?);   
    let out_dir = "./src/gen";

     tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(og_out_dir.join("helloworld.descriptor.bin"))
        .compile(&["./proto/helloworld.proto"], &["proto"])?;

     Ok(())
}
