use std::env;
use std::path::PathBuf;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <zip-file>", args[0]);
        return 1;
    }

    let file = std::fs::File::open(&args[1]).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath: PathBuf = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        dbg!(&outpath);
        if file.is_file() {
            println!(
                "File: {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)
                        .expect(format!("Failed to create directory {:#?}", p).as_str());
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        } else {
            println!("File: {} extracted to \"{}\"", i, outpath.display());
            std::fs::create_dir_all(&outpath).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}
