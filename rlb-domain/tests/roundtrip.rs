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
    use rlb_format::EntrySlot;

    #[test]
    fn parses_and_resolves_table_of_contents_names() {
        let bytes = std::fs::read("../examples/ScriptList_Ar05Zn02.rlb").unwrap();
        let file = RLBFile::parse(&bytes).expect("parse should succeed on a well-formed file");
        assert_eq!(file.toc.len(), 9);

        let entry = file
            .toc
            .find_by_name("FsbFileListData")
            .expect("table named `FsbFileListData` should be found via the label pool index");

        match entry {
            EntrySlot::Named { address, .. } => assert_eq!(*address, 0xA08),
            EntrySlot::Unknown { .. } => panic!("expected a named entry"),
        }

        assert!(
            file.toc.find_by_name("does-not-exist").is_none(),
            "an unknown name should not resolve to anything"
        );
    }
}
