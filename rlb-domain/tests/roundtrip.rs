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

    #[test]
    fn write_wandering() {
        for path in example_files("../examples/wandering") {
            let original = std::fs::read(&path).unwrap();

            let parsed =
                RLBFile::parse(&original).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let written = parsed
                .write()
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let reparsed =
                RLBFile::parse(&written).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            assert_eq!(
                original,
                written,
                "round-trip failed for {}",
                path.display(),
            );
            assert_eq!(
                written,
                reparsed.write().unwrap(),
                "round-trip failed for {}",
                path.display(),
            );
            println!("success: {}", path.display());
        }
    }
    #[test]
    fn tables_round_trip() {
        for path in example_files("../examples/wandering")
            .iter()
            .chain(&example_files("../examples/script_lists"))
        {
            let original = std::fs::read(&path).unwrap();

            let parsed =
                RLBFile::parse(&original).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let written = parsed
                .clone()
                .write()
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let reparsed =
                RLBFile::parse(&written).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

            let original_tables: Vec<_> = parsed.tables().collect();
            let reparsed_tables: Vec<_> = reparsed.tables().collect();

            assert_eq!(
                original_tables.len(),
                reparsed_tables.len(),
                "{}",
                path.display()
            );

            for (before_table, after_table) in original_tables.iter().zip(&reparsed_tables) {
                assert_eq!(before_table.id, after_table.id);
                assert_eq!(before_table.label, after_table.label);
                assert_eq!(before_table.entry_count, after_table.entry_count);
                assert_eq!(before_table.fields.len(), after_table.fields.len());

                for row in 0..before_table.entry_count {
                    for field in before_table.fields {
                        let before = parsed.get_field(before_table.id, row, &field.name).unwrap();
                        let after = reparsed
                            .get_field(after_table.id, row, &field.name)
                            .unwrap();

                        assert_eq!(
                            before,
                            after,
                            "{}: table {:?}, row {}, field {} changed after round-trip",
                            path.display(),
                            before_table.label,
                            row,
                            field.name,
                        );
                    }
                }
            }
        }
    }
}
