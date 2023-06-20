use crate::address::*;
use crate::address_components::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;


pub enum Mismatch {
    SubaddressType(String),
    Floor(String),
    Building(String),
    Status(String),
}

impl Mismatch {
    pub fn subaddress_type(from: Option<SubaddressType>, to: Option<SubaddressType>) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::SubaddressType(message)
    }

    pub fn floor(from: Option<i64>, to: Option<i64>) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::Floor(message)
    }

    pub fn building(from: Option<String>, to: Option<String>) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::Building(message)
    }

    pub fn status(from: AddressStatus, to: AddressStatus) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::Status(message)
    }
}

struct Mismatches {
    fields: Vec<Mismatch>,
}

impl Mismatches {
    pub fn new(fields: Vec<Mismatch>) -> Self {
        Mismatches { fields }
    }
}

pub struct AddressMatch {
    coincident: bool,
    mismatches: Option<Mismatches>,
}

impl AddressMatch {
    pub fn new(coincident: bool, fields: Vec<Mismatch>) -> Self {
        let mismatches = match fields.len() {
            0 => None,
            _ => Some(Mismatches::new(fields)),
        };
        AddressMatch {
            coincident,
            mismatches,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MatchStatus {
    Matching,
    Divergent,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRecord {
    pub match_status: MatchStatus,
    pub address_label: String,
    pub self_id: i64,
    pub other_id: Option<i64>,
    pub subaddress_type: Option<String>,
    pub floor: Option<String>,
    pub building: Option<String>,
    pub status: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Clone)]
pub struct MatchRecords {
    pub records: Vec<MatchRecord>,
}

impl MatchRecords {
    pub fn new(self_address: &Address, other_addresses: &[Address]) -> Self {
        let self_id = self_address.object_id();
        let address_label = self_address.label();
        let latitude = self_address.address_latitude();
        let longitude = self_address.address_longitude();

        let mut match_record = Vec::new();

        for address in other_addresses {
            let address_match = self_address.coincident(address);
            if address_match.coincident {
                let other_id = Some(address.object_id());
                let mut subaddress_type = None;
                let mut floor = None;
                let mut building = None;
                let mut status = None;
                match address_match.mismatches {
                    None => match_record.push(MatchRecord {
                        match_status: MatchStatus::Matching,
                        address_label: address_label.clone(),
                        self_id,
                        other_id,
                        subaddress_type,
                        floor,
                        building,
                        status,
                        longitude,
                        latitude,
                    }),
                    Some(mismatches) => {
                        for mismatch in mismatches.fields {
                            match mismatch {
                                Mismatch::SubaddressType(message) => {
                                    subaddress_type = Some(message)
                                }
                                Mismatch::Floor(message) => floor = Some(message),
                                Mismatch::Building(message) => building = Some(message),
                                Mismatch::Status(message) => status = Some(message),
                            }
                        }
                        match_record.push(MatchRecord {
                            match_status: MatchStatus::Divergent,
                            address_label: address_label.clone(),
                            self_id,
                            other_id,
                            subaddress_type,
                            floor,
                            building,
                            status,
                            longitude,
                            latitude,
                        })
                    }
                }
            }
        }
        if match_record.is_empty() {
            match_record.push(MatchRecord {
                match_status: MatchStatus::Missing,
                address_label,
                self_id,
                other_id: None,
                subaddress_type: None,
                floor: None,
                building: None,
                status: None,
                longitude,
                latitude,
            })
        }
        MatchRecords {
            records: match_record,
        }
    }

    pub fn compare(self_addresses: &[Address], other_addresses: &[Address]) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = self_addresses
            .par_iter()
            .map(|address| MatchRecords::new(address, other_addresses))
            .progress_with_style(style)
            .collect::<Vec<MatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item.records);
        }
        MatchRecords { records }
    }

    pub fn filter(self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "missing" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Missing)
                    .collect(),
            ),
            "divergent" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Divergent)
                    .collect(),
            ),
            "subaddress" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent
                            && record.subaddress_type.is_some()
                    })
                    .collect(),
            ),
            "floor" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent && record.floor.is_some()
                    })
                    .collect(),
            ),
            "building" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent && record.building.is_some()
                    })
                    .collect(),
            ),
            "status" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent && record.status.is_some()
                    })
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        MatchRecords { records }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.records.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut records = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: MatchRecord = result?;
            records.push(record);
        }

        Ok(MatchRecords { records })
    }
}


