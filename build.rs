use std::{env, path::Path, io, process::Command};

fn main() -> io::Result<()> {
    let sisku_react_path = Path::new("./sisku-react");
    env::set_current_dir(&sisku_react_path)?;
    Command::new("npm")
        .args(&["run", "build"])
        .status()?;
    Ok(())
}