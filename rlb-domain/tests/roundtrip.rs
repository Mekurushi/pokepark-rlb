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

    #[test]
    fn parses_and_resolves_table_of_contents_names() {
        let bytes = std::fs::read("../examples/ScriptList_Ar05Zn02.rlb").unwrap();
        let file = RLBFile::parse(&bytes).expect("parse should succeed on a well-formed file");

        // tests
        println!("{:#?}", file);
    }
}
