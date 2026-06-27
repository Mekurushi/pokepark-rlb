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

    use rlb_format::{RawFile, TableRecord};

    fn build_file(
        data: &[u8],
        relocs: &[u32],
        named: &[(u32, u32)],
        unknown: &[(u32, u32)],
        labels: &[u8],
    ) -> Vec<u8> {
        let header_size = 0x20u32;
        let data_size = data.len() as u32;
        let reloc_size = relocs.len() as u32 * 4;
        let entries_size = (named.len() + unknown.len()) as u32 * 8;
        let labels_size = labels.len() as u32;
        let file_size = header_size + data_size + reloc_size + entries_size + labels_size;

        let mut bytes = Vec::with_capacity(file_size as usize);
        bytes.extend_from_slice(&file_size.to_be_bytes());
        bytes.extend_from_slice(&data_size.to_be_bytes());
        bytes.extend_from_slice(&(relocs.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&(named.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&(unknown.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&[0u8; 12]);

        bytes.extend_from_slice(data);
        for r in relocs {
            bytes.extend_from_slice(&r.to_be_bytes());
        }
        for &(address, name_offset) in named {
            bytes.extend_from_slice(&address.to_be_bytes());
            bytes.extend_from_slice(&name_offset.to_be_bytes());
        }
        for &(address, raw_offset) in unknown {
            bytes.extend_from_slice(&address.to_be_bytes());
            bytes.extend_from_slice(&raw_offset.to_be_bytes());
        }
        bytes.extend_from_slice(labels);

        assert_eq!(bytes.len() as u32, file_size);
        bytes
    }

    fn single_entry_file_bytes() -> Vec<u8> {
        build_file(&[0xAA, 0xBB, 0xCC, 0xDD], &[0], &[(0x10, 0)], &[], b"foo\0")
    }

    fn multi_entry_file_bytes() -> Vec<u8> {
        build_file(
            &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            &[4, 0],
            &[(0x100, 0), (0x200, 6)],
            &[(0x300, 0xDEAD_BEEF)],
            b"alpha\0beta\0",
        )
    }

    #[test]
    fn parse_then_write_round_trips_byte_for_byte() {
        for original in [single_entry_file_bytes(), multi_entry_file_bytes()] {
            let raw = RawFile::parse(&original).expect("parse should succeed");
            let rewritten = raw.serialize_custom().expect("write should succeed");

            assert_eq!(original, rewritten);
        }
    }

    #[test]
    fn parses_named_entries_before_unknown_entries_in_order() {
        let bytes = multi_entry_file_bytes();
        let raw = RawFile::parse(&bytes).expect("parse should succeed");

        assert_eq!(raw.records.len(), 2);
        assert_eq!(raw.other_records.len(), 1);

        assert_eq!(
            raw.records[0],
            TableRecord {
                address: 0x100,
                label_offset: 0,
            }
        );

        assert_eq!(
            raw.records[1],
            TableRecord {
                address: 0x200,
                label_offset: 6
            }
        );

        assert_eq!(
            raw.other_records[0],
            TableRecord {
                address: 0x300,
                label_offset: 0xDEAD_BEEF
            }
        );

        let reloc_sites: Vec<u32> = raw.relocation_table.into_iter().collect();
        assert_eq!(reloc_sites, vec![4, 0]);
    }

    #[test]
    fn rejects_file_with_mismatched_declared_size() {
        let mut bytes = single_entry_file_bytes();
        bytes.truncate(bytes.len() - 1);

        assert!(RawFile::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_internally_inconsistent_section_counts_without_panicking() {
        let mut bytes = single_entry_file_bytes();
        bytes[8..12].copy_from_slice(&999u32.to_be_bytes());

        assert!(RawFile::parse(&bytes).is_err());
    }

    #[test]
    fn sanity_test_with_real_file() {
        // uncommitted file
        let bytes =
            std::fs::read("../examples/ScriptList_Ar05Zn02.rlb").expect("read file should succeed");
        let raw = RawFile::parse(&bytes).expect("parse should succeed");
        let rewritten = raw.serialize_custom().expect("write should succeed");
        assert_eq!(bytes, rewritten);
    }
}
