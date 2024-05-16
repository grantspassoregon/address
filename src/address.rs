//! The `address` module defines the library data standard for a valid address, and provides
//! implementation blocks to convert data from import types to the valid address format.
use crate::prelude::{
    from_csv, load_bin, save, to_csv, AddressMatch, AddressStatus, FireInspections, LexisNexis,
    Mismatch, Point, Portable, StreetNamePostType, StreetNamePreDirectional, SubaddressType,
    Vectorized,
};
use aid::prelude::*;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use tracing::{error, info, trace};

/// The `Address` trait enables the data to function as well-formed address.  The methods of the
/// trait define values for constituent components of an address.  The address components follow
/// the FGDC classification.
pub trait Address {
    /// The `number` method returns the address number component.
    fn number(&self) -> i64;
    /// The `number_mut` method returns a mutable reference to the address number component.
    fn number_mut(&mut self) -> &mut i64;
    /// The `number_suffix` method returns the address number suffix component.
    fn number_suffix(&self) -> &Option<String>;
    /// The `number_suffix_mut` method returns a mutable reference to the address number suffix component.
    fn number_suffix_mut(&mut self) -> &mut Option<String>;
    /// The `directional` method returns the [`StreetNamePreDirectional`] component, if any.
    fn directional(&self) -> &Option<StreetNamePreDirectional>;
    /// The `directional` method returns a mutable reference to the [`StreetNamePreDirectional`] value.
    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional>;
    /// The `street_name_pre_modifier` method returns the street name pre modifier component.
    fn street_name_pre_modifier(&self) -> &Option<String>;
    /// The `street_name_pre_modifier_mut` method returns a mutable reference to the street name pre modifier component.
    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<String>;
    /// The `street_name_pre_type` method returns the street name pre type component.
    fn street_name_pre_type(&self) -> &Option<String>;
    /// The `street_name_pre_type_mut` method returns a mutable reference to the street name pre type component.
    fn street_name_pre_type_mut(&mut self) -> &mut Option<String>;
    /// The `street_name_separator` method returns the separator element component.
    fn street_name_separator(&self) -> &Option<String>;
    /// The `street_name_separator_mut` method returns a mutable reference to the separator element component.
    fn street_name_separator_mut(&mut self) -> &mut Option<String>;
    /// The `street_name` method returns the street name component.
    fn street_name(&self) -> &String;
    /// The `street_name_mut` method returns a mutable reference to the street name component.
    fn street_name_mut(&mut self) -> &mut String;
    /// The `street_type` method returns the street name post type component.
    fn street_type(&self) -> &Option<StreetNamePostType>;
    /// The `street_type_mut` method returns a mutable reference to the street name post type component.
    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType>;
    /// The `subaddress_id` method returns the subaddress identifier component, if any.
    fn subaddress_id(&self) -> &Option<String>;
    /// The `subaddress_id_mut` method returns a mutable reference to the vale of the subaddress identifier component.
    fn subaddress_id_mut(&mut self) -> &mut Option<String>;
    /// The `subaddress_type` method returns the subaddress type component, if any.
    fn subaddress_type(&self) -> &Option<SubaddressType>;
    /// The `subaddress_type_mut` method returns a mutable reference to the value of the subaddress type component.
    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType>;
    /// The `floor` method returns the floor identifier corresponding to the `Floor` field in the
    /// NENA standard, required for emergency response.
    fn floor(&self) -> &Option<i64>;
    /// The `floor_mut` method returns a mutable reference to the value of the floor identifier.
    fn floor_mut(&mut self) -> &mut Option<i64>;
    /// The `building` method returns the building identifier corresponing to the `Building` field
    /// in the NENA standard, required for emergency response.
    fn building(&self) -> &Option<String>;
    /// The `building_mut` method returns a mutable reference to the value of the building
    /// identifier.
    fn building_mut(&mut self) -> &mut Option<String>;
    /// The `zip` method returns the zip code component of the address.
    fn zip(&self) -> i64;
    /// The `zip_mut` method returns a mutable reference to the value of the zip code component.
    fn zip_mut(&mut self) -> &mut i64;
    /// The `postal_community` method returns the postal community component of the address, being
    /// the unincorporated or incorporated municipality name.
    fn postal_community(&self) -> &String;
    /// The `postal_community_mut` method returns a mutable reference to the value of the postal
    /// community component.
    fn postal_community_mut(&mut self) -> &mut String;
    /// The `state` method returns the state name component of the address.
    fn state(&self) -> &String;
    /// The `state_mut` method returns a mutable reference to the value of the state name
    /// component.
    fn state_mut(&mut self) -> &mut String;
    /// The `status` method returns the local status of the address, as determined by the
    /// relevant address authority.
    fn status(&self) -> &AddressStatus;
    /// The `status_mut` method returns a mutable reference to the value of the address status.
    fn status_mut(&mut self) -> &mut AddressStatus;

    /// An address is coincident when the `other` address refers to the same assignment or
    /// location.  If the addresses are coincident, but details (such as the floor number or
    /// address status) differ, then the differences are recorded as a vector of type [`Mismatch`].
    /// The results are converted to type [`AddressMatch`].
    fn coincident<T: Address>(&self, other: &T) -> AddressMatch {
        let mut coincident = false;
        let mut mismatches = Vec::new();
        if self.number() == other.number()
            && self.number_suffix() == other.number_suffix()
            && self.directional() == other.directional()
            && self.street_name_pre_modifier() == other.street_name_pre_modifier()
            && self.street_name_pre_type() == other.street_name_pre_type()
            && self.street_name_separator() == other.street_name_separator()
            && self.street_name() == other.street_name()
            && self.street_type() == other.street_type()
            && self.subaddress_id() == other.subaddress_id()
            && self.zip() == other.zip()
            && self.postal_community() == other.postal_community()
            && self.state() == other.state()
        {
            coincident = true;
            if self.subaddress_type() != other.subaddress_type() {
                mismatches.push(Mismatch::subaddress_type(
                    *self.subaddress_type(),
                    *other.subaddress_type(),
                ));
            }
            if self.floor() != other.floor() {
                mismatches.push(Mismatch::floor(*self.floor(), *other.floor()));
            }
            if self.building() != other.building() {
                mismatches.push(Mismatch::building(
                    self.building().clone(),
                    other.building().clone(),
                ));
            }
            if self.status() != other.status() {
                mismatches.push(Mismatch::status(*self.status(), *other.status()));
            }
        }
        AddressMatch::new(coincident, mismatches)
    }

    /// Returns a String representing the address label, consisting of the complete address number,
    /// complete street name and complete subaddress, used to produce map or mailing labels.
    fn label(&self) -> String {
        let complete_address_number = match &self.number_suffix() {
            Some(suffix) => format!("{} {}", self.number(), suffix),
            None => self.number().to_string(),
        };

        let complete_street_name = self.complete_street_name(true);

        let accessory = self.building().as_ref().map(|v| format!("BLDG {v}"));

        let complete_subaddress = match &self.subaddress_id() {
            Some(identifier) => match self.subaddress_type() {
                Some(subaddress_type) => Some(format!("{} {}", subaddress_type, identifier)),
                None => Some(format!("#{}", identifier)),
            },
            None => self
                .subaddress_type()
                .map(|subaddress_type| format!("{}", subaddress_type)),
        };

        match complete_subaddress {
            Some(subaddress) => format!(
                "{} {} {}",
                complete_address_number, complete_street_name, subaddress
            ),
            None => match accessory {
                Some(value) => format!(
                    "{} {} {}",
                    complete_address_number, complete_street_name, value
                ),
                None => format!("{} {}", complete_address_number, complete_street_name),
            },
        }
    }

    /// The `complete_street_name` method returns the complete street name of the address.
    fn complete_street_name(&self, abbreviate: bool) -> String {
        let mut name = String::new();
        if let Some(directional) = self.directional() {
            if abbreviate {
                if let Some(dir) = &self.directional_abbreviated() {
                    name.push_str(dir);
                }
            } else {
                name.push_str(&directional.to_string());
            }
            name.push(' ');
        }
        if let Some(modifier) = self.street_name_pre_modifier() {
            name.push_str(modifier);
            name.push(' ');
        }
        if let Some(pre_type) = self.street_name_pre_type() {
            name.push_str(pre_type);
            name.push(' ');
        }
        if let Some(separator) = self.street_name_separator() {
            name.push_str(separator);
            name.push(' ');
        }
        name.push_str(&self.street_name().to_string());
        if let Some(post_type) = self.street_type() {
            name.push(' ');
            name.push_str(&post_type.to_string());
        }
        name
    }

    /// The `pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    fn directional_abbreviated(&self) -> Option<String> {
        match self.directional() {
            Some(StreetNamePreDirectional::NORTH) => Some("N".to_string()),
            Some(StreetNamePreDirectional::EAST) => Some("E".to_string()),
            Some(StreetNamePreDirectional::SOUTH) => Some("S".to_string()),
            Some(StreetNamePreDirectional::WEST) => Some("W".to_string()),
            Some(StreetNamePreDirectional::NORTHEAST) => Some("NE".to_string()),
            Some(StreetNamePreDirectional::NORTHWEST) => Some("NW".to_string()),
            Some(StreetNamePreDirectional::SOUTHEAST) => Some("SE".to_string()),
            Some(StreetNamePreDirectional::SOUTHWEST) => Some("SW".to_string()),
            None => None,
        }
    }

    /// The `filter_field` method returns the subset of addresses where the field `filter` is equal
    /// to the value in `field`.
    fn filter_field<T: Address + Clone + Send + Sync>(
        values: &[T],
        filter: &str,
        field: &str,
    ) -> Vec<T> {
        let mut records = Vec::new();
        match filter {
            "label" => records.append(
                &mut values
                    // .iter()
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.label())
                    .collect(),
            ),
            "street_name" => records.append(
                &mut values
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.street_name())
                    .collect(),
            ),
            "pre_directional" => records.append(
                &mut values
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        if let Some(dir) = record.directional() {
                            field == format!("{}", dir)
                        } else {
                            false
                        }
                    })
                    .collect(),
            ),
            "post_type" => records.append(
                &mut values
                    .par_iter()
                    .cloned()
                    .filter(|record| field == format!("{:?}", record.street_type()))
                    .collect(),
            ),

            _ => info!("Invalid filter provided."),
        }
        records
    }
}

/// The `Addresses` trait enables methods that act on vectors of type [`Address`].
pub trait Addresses<T: Address + Clone + Send + Sync>
where
    Self: Vectorized<T> + Clone,
{
    /// The `filter` method returns the subset of addresses that match the filter.  Current values
    /// include "duplicate", which retains addresses that contain a duplicate in the set.
    fn filter(&self, filter: &str) -> Vec<T> {
        let mut records = Vec::new();
        let values = self.values();
        match filter {
            "duplicate" => {
                let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Checking for duplicate addresses.'}",
        )
        .unwrap();
                let mut seen = HashSet::new();
                let bar = ProgressBar::new(self.len() as u64);
                bar.set_style(style);
                for address in values {
                    let label = address.label();
                    if !seen.contains(&label) {
                        seen.insert(label.clone());
                        let mut same = self.clone();
                        same.filter_field("label", &label);
                        if same.len() > 1 {
                            records.append(&mut same.into_values());
                        }
                    }
                    bar.inc(1);
                }
            }
            _ => error!("Invalid filter provided."),
        }
        records
    }

    /// The `filter_field` method returns the subset of addresses where the field `filter` is equal
    /// to the value in `field`.
    fn filter_field(&mut self, filter: &str, field: &str) {
        let mut records = Vec::new();
        match filter {
            "label" => records.append(
                &mut self
                    .values_mut()
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.label())
                    .collect(),
            ),
            "street_name" => records.append(
                &mut self
                    .values_mut()
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.street_name())
                    .collect(),
            ),
            "pre_directional" => records.append(
                &mut self
                    .values_mut()
                    .par_iter()
                    .cloned()
                    .filter(|record| field == format!("{:?}", record.directional()))
                    .collect(),
            ),
            "post_type" => records.append(
                &mut self
                    .values_mut()
                    .par_iter()
                    .cloned()
                    .filter(|record| field == format!("{:?}", record.street_type()))
                    .collect(),
            ),

            _ => info!("Invalid filter provided."),
        }
        self.values_mut().clear();
        self.values_mut().extend(records);
    }

    /// Compares the complete street name of an address to the value in `street`, returning true if
    /// equal.
    fn contains_street(&self, street: &String) -> bool {
        let mut contains = false;
        for address in self.values() {
            let comp_street = address.complete_street_name(false);
            if &comp_street == street {
                contains = true;
            }
        }
        contains
    }

    /// The `orphan_streets` method returns the list of complete street names that are contained in
    /// self but are not present in `other`.
    fn orphan_streets<V: Address + Clone + Send + Sync, U: Addresses<V>>(
        &self,
        other: &U,
    ) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut orphans = Vec::new();
        for address in self.values() {
            let street = address.complete_street_name(false);
            if !seen.contains(&street) {
                seen.insert(street.clone());
                if !other.contains_street(&street) {
                    orphans.push(street);
                }
            }
        }
        orphans
    }

    /// The `citify` method takes county address naming conventions and converts them to city
    /// naming conventions.
    fn citify(&mut self) {
        trace!("Running Citify");
        for address in self.values_mut() {
            let comp_street = address.complete_street_name(false);
            if comp_street == "NE BEAVILLA VIEW" {
                trace!("Fixing Beavilla View");
                *address.street_name_mut() = "BEAVILLA".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::VIEW);
            }
            if comp_street == "COLUMBIA CREST" {
                trace!("Fixing Columbia Crest");
                *address.street_name_mut() = "COLUMBIA".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::CREST);
            }
            if comp_street == "SE FORMOSA GARDENS" {
                trace!("Fixing Formosa Gardens");
                *address.street_name_mut() = "FORMOSA".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::GARDENS);
            }
            if comp_street == "SE HILLTOP VIEW" {
                trace!("Fixing Hilltop View");
                *address.street_name_mut() = "HILLTOP".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::VIEW);
            }
            if comp_street == "MARILEE ROW" {
                trace!("Fixing Marilee Row");
                *address.street_name_mut() = "MARILEE".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::ROW);
            }
            if comp_street == "MEADOW GLEN" {
                trace!("Fixing Meadow Glen");
                *address.street_name_mut() = "MEADOW".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::GLEN);
            }
            if comp_street == "ROBERTSON CREST" {
                trace!("Fixing Robertson Crest");
                *address.street_name_mut() = "ROBERTSON".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::CREST);
            }
            if comp_street == "NE QUAIL CROSSING" {
                trace!("Fixing Quail Crossing");
                *address.street_name_mut() = "QUAIL".to_owned();
                *address.street_type_mut() = Some(StreetNamePostType::CROSSING);
            }
        }
    }

    /// The `LexisNexis` method produces the LexisNexis table showing dispatch jurisdiction for
    /// address ranges within the City of Grants Pass.
    fn lexis_nexis(&self, other: &Self) -> Clean<LexisNexis> {
        LexisNexis::from_addresses(self, other)
    }
}

/// The `CommonAddress` struct defines the fields of a valid address, following the FGDC standard,
/// with the inclusion of NENA-required fields for emergency response.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommonAddress {
    /// The `number` field represents the address number component of the complete address
    /// number.
    pub number: i64,
    /// The `number_suffix` field represents the address number suffix component of the complete
    /// address number.
    pub number_suffix: Option<String>,
    /// The `directional` field represents the street name pre directional component of the
    /// complete street name.
    pub directional: Option<StreetNamePreDirectional>,
    /// The `pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    pub pre_modifier: Option<String>,
    /// The `pre_type` field represents the street name pre type component of the complete street
    /// name.
    pub pre_type: Option<String>,
    /// The `separator` field represents the separator element component of the complete street
    /// name.
    pub separator: Option<String>,
    /// The `street_name` field represents the street name component of the complete street name.
    pub street_name: String,
    /// The `street_type` field represents the street name post type component of the complete street
    /// name.
    pub street_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_id` field represents the subaddress identifier component of the complete
    /// subaddress.
    pub subaddress_id: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    pub floor: Option<i64>,
    /// The `building` field represents the building identifier, corresponding to the `Building` field from the NENA standard.
    pub building: Option<String>,
    /// The `zip` field represents the postal zip code of the address.
    pub zip: i64,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    pub postal_community: String,
    /// The `state` field represents the state name component of the address.
    pub state: String,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: AddressStatus,
}

impl Address for CommonAddress {
    fn number(&self) -> i64 {
        self.number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.directional
    }

    fn street_name_pre_modifier(&self) -> &Option<String> {
        &self.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<String> {
        &mut self.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<String> {
        &self.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<String> {
        &mut self.pre_type
    }

    fn street_name_separator(&self) -> &Option<String> {
        &self.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<String> {
        &mut self.separator
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.street_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_id
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.subaddress_id
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.floor
    }

    fn building(&self) -> &Option<String> {
        &self.building
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.building
    }

    fn zip(&self) -> i64 {
        self.zip
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.zip
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.postal_community
    }

    fn state(&self) -> &String {
        &self.state
    }

    fn state_mut(&mut self) -> &mut String {
        &mut self.state
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

impl Vectorized<CommonAddress> for CommonAddresses {
    fn values(&self) -> &Vec<CommonAddress> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<CommonAddress> {
        &mut self.records
    }

    fn into_values(self) -> Vec<CommonAddress> {
        self.records
    }
}

impl<T: Address> From<&T> for CommonAddress {
    fn from(address: &T) -> Self {
        let number = address.number();
        let number_suffix = address.number_suffix().clone();
        let directional = *address.directional();
        let pre_modifier = address.street_name_pre_modifier().clone();
        let pre_type = address.street_name_pre_type().clone();
        let separator = address.street_name_separator().clone();
        let street_name = address.street_name().clone();
        let street_type = *address.street_type();
        let subaddress_type = *address.subaddress_type();
        let subaddress_id = address.subaddress_id().clone();
        let floor = *address.floor();
        let building = address.building().clone();
        let zip = address.zip();
        let postal_community = address.postal_community().clone();
        let state = address.state().clone();
        let status = *address.status();
        Self {
            number,
            number_suffix,
            directional,
            pre_modifier,
            pre_type,
            separator,
            street_name,
            street_type,
            subaddress_type,
            subaddress_id,
            floor,
            building,
            zip,
            postal_community,
            state,
            status,
        }
    }
}

/// The `CommonAddresses` struct holds a vector of type [`CommonAddress`].
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CommonAddresses {
    /// The `records` field contains a vector of type [`CommonAddress`].
    pub records: Vec<CommonAddress>,
}

impl Addresses<CommonAddress> for CommonAddresses {}

impl Portable<CommonAddresses> for CommonAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

impl<T: Address + Clone> From<&[T]> for CommonAddresses {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(CommonAddress::from)
            .collect::<Vec<CommonAddress>>();
        Self { records }
    }
}

/// The `PartialAddress` struct contains optional fields so that incomplete or missing data can be
/// compared against [`Addresses`] or [`PartialAddresses`] for potential matches.  Used to help
/// match address information that does not parse into a full valid address.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartialAddress {
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    pub address_number: Option<i64>,
    /// The `number_suffix` field represents the address number suffix component of the complete
    /// address number.
    pub address_number_suffix: Option<String>,
    /// The `directional` field represents the street name pre directional component of the
    /// complete street name.
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    pub pre_modifier: Option<String>,
    /// The `pre_type` field represents the street name pre type component of the complete street
    /// name.
    pub pre_type: Option<String>,
    /// The `separator` field represents the separator element component of the complete street
    /// name.
    pub separator: Option<String>,
    /// The `street_name` field represents the street name component of the complete street name.
    pub street_name: Option<String>,
    /// The `street_type` field represents the street name post type component of the complete street
    /// name.
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_id` field represents the subaddress identifier component of the complete
    /// subaddress.
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    pub floor: Option<i64>,
    /// The `building` field represents the building identifier, corresponding to the `Building` field from the NENA standard.
    pub building: Option<String>,
    /// The `zip` field represents the postal zip code of the address.
    pub zip_code: Option<i64>,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    pub postal_community: Option<String>,
    /// The `state` field represents the state name component of the address.
    pub state_name: Option<String>,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: Option<AddressStatus>,
}

impl PartialAddress {
    /// Creates an empty new `PartialAddress` with all fields set to None.
    pub fn new() -> Self {
        PartialAddress::default()
    }

    /// The `address_number` field represents the address number component of the complete address
    /// number.  This function returns the value of the field.
    pub fn address_number(&self) -> Option<i64> {
        self.address_number
    }

    /// The `address_number_suffix` field represents the address number suffix component of the
    /// complete address number.  This function returns the cloned value of the field.
    pub fn address_number_suffix(&self) -> Option<String> {
        self.address_number_suffix.clone()
    }

    /// The `street_name_pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    pub fn street_name_pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    /// The `street_name` field represents the street name component of the complete street name.
    /// This function returns the cloned value of the field.
    pub fn street_name(&self) -> Option<String> {
        self.street_name.clone()
    }

    /// The `street_name_post_type` field represents the street name posttype component of the complete street
    /// name.  This function returns the cloned value of the field.
    pub fn street_name_post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.  This function returns the cloned value of the field.
    pub fn subaddress_type(&self) -> Option<SubaddressType> {
        self.subaddress_type
    }

    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.  This function returns the cloned value of the field.
    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.clone()
    }

    /// The `building` field represents the unique identifier for a building.  This function
    /// returns the cloned value of the field.
    pub fn building(&self) -> Option<String> {
        self.building.clone()
    }

    /// The `floor` field represents the floor of the building on which the address point is located.  This function returns the value of the field.
    pub fn floor(&self) -> Option<i64> {
        self.floor
    }

    /// Sets the value of the `address_number` field to Some(`value`).
    pub fn set_address_number(&mut self, value: i64) {
        self.address_number = Some(value);
    }

    /// Sets the value of the `address_number_suffix` field to Some(`value`).
    pub fn set_address_number_suffix(&mut self, value: Option<&str>) {
        if let Some(suffix) = value {
            self.address_number_suffix = Some(suffix.to_owned());
        } else {
            self.address_number_suffix = None;
        }
    }

    /// Sets the value of the `street_name_pre_directional` field to Some(`value`).
    pub fn set_pre_directional(&mut self, value: &StreetNamePreDirectional) {
        self.street_name_pre_directional = Some(value.to_owned());
    }

    /// Sets the value of the `street_name` field to Some(`value`).
    pub fn set_street_name(&mut self, value: &str) {
        self.street_name = Some(value.to_owned());
    }

    /// Sets the value of the `street_name_post_type` field to Some(`value`).
    pub fn set_post_type(&mut self, value: &StreetNamePostType) {
        self.street_name_post_type = Some(value.to_owned());
    }

    /// Sets the value of the `subaddress_type` field to Some(`value`).
    pub fn set_subaddress_type(&mut self, value: &SubaddressType) {
        self.subaddress_type = Some(value.to_owned());
    }

    /// Sets the value of the `subaddress_identifier` field to Some(`value`).
    pub fn set_subaddress_identifier(&mut self, value: &str) {
        self.subaddress_identifier = Some(value.to_owned());
    }

    /// Returns a String representing the address label, consisting of the complete address number,
    /// complete street name and complete subaddress, used to produce map or mailing labels.
    pub fn label(&self) -> String {
        let mut address = "".to_owned();
        if let Some(address_number) = self.address_number() {
            address.push_str(&format!("{}", address_number));
        }
        if let Some(address_number_suffix) = self.address_number_suffix() {
            address.push(' ');
            address.push_str(&address_number_suffix);
        }
        if let Some(pre_directional) = self.street_name_pre_directional() {
            address.push(' ');
            address.push_str(&format!("{:?}", pre_directional));
        }
        if let Some(street_name) = self.street_name() {
            address.push(' ');
            address.push_str(&street_name);
        }
        if let Some(post_type) = self.street_name_post_type() {
            address.push(' ');
            address.push_str(&format!("{:?}", post_type));
        }
        if let Some(subtype) = self.subaddress_type() {
            address.push(' ');
            address.push_str(&format!("{:?}", subtype));
        }
        if let Some(subaddress_identifier) = self.subaddress_identifier() {
            address.push(' ');
            address.push_str(&subaddress_identifier);
        }
        address
    }
}

/// The `PartialAddresses` struct holds a `records` field that contains a vector of type
/// [`PartialAddress`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartialAddresses {
    records: Vec<PartialAddress>,
}

impl PartialAddresses {
    /// Creates a new `PartialAddresses` struct from the provided `records`, a vector of
    /// [`PartialAddress`] objects.
    pub fn new(records: Vec<PartialAddress>) -> Self {
        Self { records }
    }
}

impl Portable<PartialAddresses> for PartialAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

impl Vectorized<PartialAddress> for PartialAddresses {
    fn values(&self) -> &Vec<PartialAddress> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<PartialAddress> {
        &mut self.records
    }

    fn into_values(self) -> Vec<PartialAddress> {
        self.records
    }
}

impl From<Vec<PartialAddress>> for PartialAddresses {
    fn from(records: Vec<PartialAddress>) -> Self {
        PartialAddresses { records }
    }
}

impl From<&FireInspections> for PartialAddresses {
    fn from(fire_inspections: &FireInspections) -> Self {
        PartialAddresses::from(
            fire_inspections
                .records()
                .iter()
                .map(|r| r.address())
                .collect::<Vec<PartialAddress>>(),
        )
    }
}

/// Deltas - Measuring the distance between points based upon matching values.
/// The `label` field of `AddressDelta` holds the matching value and the `delta`
/// field holds the distance between matching points.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AddressDelta {
    /// Addresses match by address label.
    pub label: String,
    /// Distance between points representing the same address.
    pub delta: f64,
    /// Reference latitude from the subject address.
    pub latitude: f64,
    /// Reference longitude from the subject address.
    pub longitude: f64,
}

impl AddressDelta {
    /// Initiates a new `AddressDelta` struct from the provided input values.
    pub fn new<T: Address + Point>(address: &T, delta: f64) -> Self {
        AddressDelta {
            label: address.label(),
            delta,
            latitude: address.y(),
            longitude: address.x(),
        }
    }
}

/// The `AddressDeltas` struct holds a `records` field that contains a vector of type
/// [`AddressDelta`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AddressDeltas {
    /// The `records` field holds a vector of type [`AddressDelta`].
    pub records: Vec<AddressDelta>,
}

impl Vectorized<AddressDelta> for AddressDeltas {
    fn values(&self) -> &Vec<AddressDelta> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<AddressDelta> {
        &mut self.records
    }

    fn into_values(self) -> Vec<AddressDelta> {
        self.records
    }
}

impl Portable<AddressDeltas> for AddressDeltas {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}
