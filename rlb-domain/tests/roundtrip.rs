#[cfg(test)]
mod tests {
    #![cfg_attr(
        test,
        allow(
            clippy::cast_possible_truncation,
            clippy::expect_used,
            clippy::map_err_ignore,
        )
    )]

    use rlb_domain::RLBFile;
    use std::path::{Path, PathBuf};

    #[test]
    fn parses_and_resolves_table_of_contents_names() {
        let bytes = std::fs::read("../examples/ScriptList_Ar05Zn02.rlb").unwrap();
        let file = RLBFile::parse(&bytes).expect("parse should succeed on a well-formed file");

        // tests
        println!("{:#?}", file);
    }

    fn example_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
        let mut files: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "rlb"))
            .collect();

        files.sort();
        files
    }

    #[test]
    fn write_script_lists() {
        for path in example_files("../examples/script_lists") {
            let original = std::fs::read(&path).unwrap();

            let parsed =
                RLBFile::parse(&original).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let written = parsed
                .write()
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let reparsed =
                RLBFile::parse(&written).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            assert_eq!(
                written,
                reparsed.write().unwrap(),
                "round-trip failed for {}",
                path.display(),
            );
            println!("success: {}", path.display());
        }
    }
}
