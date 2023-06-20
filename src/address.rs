use crate::address_components::*;
use crate::business::*;
use crate::import::*;
use crate::compare::*;
use indicatif::ProgressBar;
// use indicatif::ProgressIterator;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashSet;
use tracing::{error, info};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Address {
    address_number: i64,
    address_number_suffix: Option<String>,
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    street_name: String,
    street_name_post_type: StreetNamePostType,
    subaddress_type: Option<SubaddressType>,
    subaddress_identifier: Option<String>,
    floor: Option<i64>,
    building: Option<String>,
    zip_code: i64,
    postal_community: String,
    state_name: String,
    status: AddressStatus,
    object_id: i64,
    address_latitude: f64,
    address_longitude: f64,
}

impl Address {
    pub fn coincident(&self, other: &Address) -> AddressMatch {
        let mut coincident = false;
        let mut mismatches = Vec::new();
        if self.address_number == other.address_number
            && self.address_number_suffix == other.address_number_suffix
            && self.street_name_pre_directional == other.street_name_pre_directional
            && self.street_name == other.street_name
            && self.street_name_post_type == other.street_name_post_type
            && self.subaddress_identifier == other.subaddress_identifier
            && self.zip_code == other.zip_code
            && self.postal_community == other.postal_community
            && self.state_name == other.state_name
        {
            coincident = true;
            if self.subaddress_type != other.subaddress_type {
                mismatches.push(Mismatch::subaddress_type(
                    self.subaddress_type,
                    other.subaddress_type,
                ));
            }
            if self.floor != other.floor {
                mismatches.push(Mismatch::floor(self.floor, other.floor));
            }
            if self.building != other.building {
                mismatches.push(Mismatch::building(
                    self.building.clone(),
                    other.building.clone(),
                ));
            }
            if self.status != other.status {
                mismatches.push(Mismatch::status(self.status, other.status));
            }
        }
        AddressMatch::new(coincident, mismatches)
    }

    pub fn label(&self) -> String {
        let complete_address_number = match &self.address_number_suffix {
            Some(suffix) => format!("{} {}", self.address_number, suffix),
            None => self.address_number.to_string(),
        };
        let complete_street_name = match self.street_name_pre_directional {
            Some(pre_directional) => format!(
                "{:?} {} {:?}",
                pre_directional, self.street_name, self.street_name_post_type
            ),
            None => format!("{} {:?}", self.street_name, self.street_name_post_type),
        };
        let complete_subaddress = match &self.subaddress_identifier {
            Some(identifier) => match self.subaddress_type {
                Some(subaddress_type) => Some(format!("{:?} {}", subaddress_type, identifier)),
                None => Some(format!("#{}", identifier)),
            },
            None => self
                .subaddress_type
                .map(|subaddress_type| format!("{:?}", subaddress_type)),
        };
        match complete_subaddress {
            Some(subaddress) => format!(
                "{} {} {}",
                complete_address_number, complete_street_name, subaddress
            ),
            None => format!("{} {}", complete_address_number, complete_street_name),
        }
    }

    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn post_type(&self) -> StreetNamePostType {
        self.street_name_post_type
    }

    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    pub fn floor(&self) -> Option<i64> {
        self.floor
    }

    pub fn zip_code(&self) -> i64 {
        self.zip_code
    }

    pub fn status(&self) -> AddressStatus {
        self.status
    }

    pub fn state_name(&self) -> String {
        self.state_name.to_owned()
    }

    pub fn postal_community(&self) -> String {
        self.postal_community.to_owned()
    }

    pub fn object_id(&self) -> i64 {
        self.object_id
    }

    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

impl From<CityAddress> for Address {
    fn from(item: CityAddress) -> Self {
        Address {
            address_number: item.address_number(),
            address_number_suffix: item.address_number_suffix(),
            street_name_pre_directional: item.street_name_pre_directional(),
            street_name: item.street_name(),
            street_name_post_type: item.street_name_post_type(),
            subaddress_type: item.subaddress_type(),
            subaddress_identifier: item.subaddress_identifier(),
            floor: item.floor(),
            building: item.building(),
            zip_code: item.zip_code(),
            postal_community: item.postal_community(),
            state_name: item.state_name(),
            status: item.status(),
            object_id: item.object_id(),
            address_latitude: item.address_latitude(),
            address_longitude: item.address_longitude(),
        }
    }
}

impl From<&CityAddress> for Address {
    fn from(item: &CityAddress) -> Self {
        Address {
            address_number: item.address_number(),
            address_number_suffix: item.address_number_suffix(),
            street_name_pre_directional: item.street_name_pre_directional(),
            street_name: item.street_name(),
            street_name_post_type: item.street_name_post_type(),
            subaddress_type: item.subaddress_type(),
            subaddress_identifier: item.subaddress_identifier(),
            floor: item.floor(),
            building: item.building(),
            zip_code: item.zip_code(),
            postal_community: item.postal_community(),
            state_name: item.state_name(),
            status: item.status(),
            object_id: item.object_id(),
            address_latitude: item.address_latitude(),
            address_longitude: item.address_longitude(),
        }
    }
}

impl TryFrom<CountyAddress> for Address {
    type Error = ();

    fn try_from(item: CountyAddress) -> Result<Self, Self::Error> {
        match item.street_name_post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: item.address_number_suffix(),
                street_name_pre_directional: item.street_name_pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: item.subaddress_type(),
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<&CountyAddress> for Address {
    type Error = ();

    fn try_from(item: &CountyAddress) -> Result<Self, Self::Error> {
        match item.street_name_post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: item.address_number_suffix(),
                street_name_pre_directional: item.street_name_pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: item.subaddress_type(),
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<GrantsPass2022Address> for Address {
    type Error = ();

    fn try_from(item: GrantsPass2022Address) -> Result<Self, Self::Error> {
        match item.post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: None,
                street_name_pre_directional: item.pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: None,
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<&GrantsPass2022Address> for Address {
    type Error = ();

    fn try_from(item: &GrantsPass2022Address) -> Result<Self, Self::Error> {
        match item.post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: None,
                street_name_pre_directional: item.pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: None,
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

#[derive(Default, Serialize, Clone)]
pub struct Addresses {
    pub records: Vec<Address>,
}

impl Addresses {
    pub fn filter(&self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "duplicate" => {
                let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Checking for duplicate addresses.'}",
        )
        .unwrap();
                let mut seen = HashSet::new();
                let bar = ProgressBar::new(self.records.len() as u64);
                bar.set_style(style);
                for address in &self.records {
                    let label = address.label();
                    if !seen.contains(&label) {
                        seen.insert(label.clone());
                        let mut same = self.filter_field("label", &label);
                        if same.records.len() > 1 {
                            records.append(&mut same.records);
                        }
                    }
                    bar.inc(1);
                }
            }
            _ => error!("Invalid filter provided."),
        }
        Addresses { records }
    }

    fn filter_field(&self, filter: &str, field: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "label" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.label())
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        Addresses { records }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.records.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl From<CityAddresses> for Addresses {
    fn from(item: CityAddresses) -> Self {
        let mut records = Vec::new();
        for address in item.records {
            if let Ok(record) = Address::try_from(address) {
                records.push(record);
            }
        }
        Addresses { records }
    }
}

impl From<CountyAddresses> for Addresses {
    fn from(item: CountyAddresses) -> Self {
        let mut records = Vec::new();
        for address in item.records {
            if let Ok(record) = Address::try_from(address) {
                records.push(record);
            }
        }
        Addresses { records }
    }
}

impl From<GrantsPass2022Addresses> for Addresses {
    fn from(item: GrantsPass2022Addresses) -> Self {
        let mut records = Vec::new();
        for address in item.records {
            if let Ok(record) = Address::try_from(address) {
                records.push(record);
            }
        }
        Addresses { records }
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct PartialAddress {
    address_number: Option<i64>,
    address_number_suffix: Option<String>,
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    street_name: Option<String>,
    street_name_post_type: Option<StreetNamePostType>,
    subaddress_type: Option<SubaddressType>,
    subaddress_identifier: Option<String>,
    floor: Option<i64>,
    building: Option<String>,
    zip_code: Option<i64>,
    postal_community: Option<String>,
    state_name: Option<String>,
    status: Option<AddressStatus>,
}

impl PartialAddress {
    pub fn new() -> Self {
        PartialAddress::default()
    }

    pub fn address_number(&mut self, value: i64) {
        self.address_number = Some(value);
    }

    pub fn pre_directional(&mut self, value: &StreetNamePreDirectional) {
        self.street_name_pre_directional = Some(value.to_owned());
    }

    pub fn street_name(&mut self, value: &str) {
        self.street_name = Some(value.to_owned());
    }

    pub fn post_type(&mut self, value: &StreetNamePostType) {
        self.street_name_post_type = Some(value.to_owned());
    }

    pub fn subaddress_type(&mut self, value: &SubaddressType) {
        self.subaddress_type = Some(value.to_owned());
    }

    pub fn subaddress_identifier(&mut self, value: &str) {
        self.subaddress_identifier = Some(value.to_owned());
    }
}
