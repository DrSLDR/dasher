use std::{env, path::PathBuf};

use dasher::*;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let mut iter = env::args_os();
    // Drop the self-reference off the front
    let self_ref = iter.next().unwrap();
    let self_name = self_ref.to_string_lossy();
    // Prepare a vector for any paths
    let mut paths: Vec<PathBuf> = Vec::new();

    for arg in iter {
        paths.push(PathBuf::try_from(arg)?);
    }

    if paths.len() == 0 {
        println!("Usage: {} <path> [<path> [<path> ...]]", self_name);
    } else {
        for path in paths.iter() {
            let hash = match hash_directory(path.clone()) {
                Ok(v) => Ok(v),
                Err(e) => Err(anyhow!(e)),
            }?;
            let digest = hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            println!("{}\t{}", digest, path.to_string_lossy());
        }
    }

    Ok(())
}
