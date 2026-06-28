use std::collections::HashSet;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RelocationTable {
    sites: Vec<u32>,
    lookup: HashSet<u32>,
}

impl RelocationTable {
    pub fn from_raw(sites: &[u32]) -> Self {
        let lookup = sites.iter().copied().collect();
        Self {
            sites: Vec::from(sites),
            lookup,
        }
    }

    pub fn len(&self) -> usize {
        self.sites.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sites.is_empty()
    }

    pub fn sites(&self) -> impl Iterator<Item = u32> + '_ {
        self.sites.iter().copied()
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.sites
    }

    pub fn is_relocated(&self, offset: u32) -> bool {
        self.lookup.contains(&offset)
    }

    // TODO mutation API
}

#[cfg(test)]
mod tests {
    use super::RelocationTable;

    #[test]
    fn is_relocated_reflects_exactly_the_site_list() {
        let table = RelocationTable::from_raw(&[0x10, 0x40]);

        assert!(table.is_relocated(0x10));
        assert!(table.is_relocated(0x40));
        assert!(
            !table.is_relocated(0x20),
            "an offset not in the table must not be reported as a pointer site"
        );
    }

    #[test]
    fn sites_preserves_on_disk_order_for_round_tripping() {
        let table = RelocationTable::from_raw(&[0x40, 0x10, 0x30]);
        let sites: Vec<u32> = table.sites().collect();
        assert_eq!(sites, vec![0x40, 0x10, 0x30]);
    }
}
