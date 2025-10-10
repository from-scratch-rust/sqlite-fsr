use crate::models::record::Record;

pub mod interiortablepage;
pub use interiortablepage::InteriorTablePage;

pub mod leaftablepage;
pub use leaftablepage::LeafTablePage;

pub trait Table {
    fn to_table_rows(&mut self) -> Vec<Record>;
}


pub enum TablePage<'a> {
    Leaf(LeafTablePage),
    Interior(InteriorTablePage<'a>),
}

impl Table for TablePage<'_> {
    fn to_table_rows(&mut self) -> Vec<Record> {
        match self {
            TablePage::Leaf(p) => p.to_table_rows(),
            TablePage::Interior(p) => p.to_table_rows(),
        }
    }
}