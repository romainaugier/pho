use std::path::Path;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=res");

    let out_dir = std::env::var("OUT_DIR")?;
    let target_dir = Path::new(&out_dir)
        .parent().unwrap() // debug|release
        .parent().unwrap() // deps
        .parent()          // target/debug|release
        .ok_or("Cannot find target directory")?;

    let source_dir = Path::new("res");
    let dest_dir = target_dir.join("res");

    if dest_dir.exists() {
        fs::remove_dir_all(&dest_dir)
            .map_err(|e| format!("Failed to remove old res dir: {}", e))?;
    }

    copy_dir_all(source_dir, &dest_dir)?;

    println!("cargo:info=Copied res/ directory to {}", dest_dir.display());

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
