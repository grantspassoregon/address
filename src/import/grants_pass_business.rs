//! The `grants_pass_business` module contains data types for importing business license reports
//! for the City of Grants Pass.
use crate::prelude::*;
use aid::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops;

/// The `BusinessRaw` struct contains business license records. Serves as an intermediary for
/// creating a [`Business`] struct when reading the data in from a csv.  Mainly this involves
/// parsing the `street_address_label` from a String into a `PartialAddress`.
/// The fields correspond to the export format from the GIS layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BusinessRaw {
    company_name: String,
    contact_name: Option<String>,
    dba: Option<String>,
    street_address_label: String,
    license: String,
    industry_code: i32,
    industry_name: String,
    sector_code: i32,
    sector_name: String,
    subsector_code: i32,
    subsector_name: Option<String>,
    tourism: Option<String>,
    district: Option<String>,
}

/// The `BusinessesRaw` struct is a wrapper for a vector of type [`BusinessRaw`].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BusinessesRaw {
    // The `records` field holds a vector of type [`BusinessRaw`].
    records: Vec<BusinessRaw>,
}

impl BusinessesRaw {
    /// Writes the contents of the struct to a csv file at location `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(BusinessesRaw { records })
    }

    /// The `records` method returns the cloned value of the `records` field, containing a vector
    /// of type [`BusinessRaw`].
    pub fn records(&self) -> Vec<BusinessRaw> {
        self.records.clone()
    }
}

impl ops::Deref for BusinessesRaw {
    type Target = Vec<BusinessRaw>;

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl ops::DerefMut for BusinessesRaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.records
    }
}

/// The `Business` struct holds query information for active business licenses, for access in GIS.
#[derive(Clone, Debug)]
pub struct Business {
    // The official name of the company.
    company_name: String,
    // The contact for the company.
    contact_name: Option<String>,
    // The business alias of the company.
    dba: Option<String>,
    // The situs address of the business.
    address: PartialAddress,
    // The license identifier.
    license: String,
    // The NAICS industry code of the business.
    industry_code: i32,
    // The NAICS industry code description.
    industry_name: String,
    // The NAICS sector code of the business.
    sector_code: i32,
    // The NAICS sector code description.
    sector_name: String,
    // The NAICS subsector code.
    subsector_code: i32,
    // The NAICS subsector code description.
    subsector_name: Option<String>,
    // Broad business categories used to drive symbolization in a GIS map.
    tourism: Option<String>,
    // The business district name of the GC zone, if in a GC zone.
    district: Option<String>,
}

impl Business {
    /// The `company_name` method returns the cloned value of the `company_name` field, which
    /// contains the company name.
    pub fn company_name(&self) -> String {
        self.company_name.to_owned()
    }

    /// The `contact_name` method returns the cloned value of the `contact_name` field, which
    /// contains the contact name for the business.
    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.clone()
    }

    /// The `dba` method returns the cloned value of the `dba` field, which contains the business
    /// alias name.
    pub fn dba(&self) -> Option<String> {
        self.dba.clone()
    }

    /// The `address` method returns the cloned value of the `address` field, which contains a
    /// [`PartialAddress`] constructed from the provided business address.
    pub fn address(&self) -> PartialAddress {
        self.address.clone()
    }

    /// The `license` method returns the cloned value of the `license` field, which contains the
    /// license identifier assigned to the business.
    pub fn license(&self) -> String {
        self.license.to_owned()
    }

    /// The `industry_code` method returns the NAICS industry code.
    pub fn industry_code(&self) -> i32 {
        self.industry_code
    }

    /// The `industry_name` method returns the NAICS industry code description.
    pub fn industry_name(&self) -> String {
        self.industry_name.to_owned()
    }

    /// The `sector_code` method returns the NAICS sector code.
    pub fn sector_code(&self) -> i32 {
        self.sector_code
    }

    /// The `sector_name` method returns the NAICS sector code description.
    pub fn sector_name(&self) -> String {
        self.sector_name.to_owned()
    }

    /// The `subsector_code` method returns the NAICS subsector code.
    pub fn subsector_code(&self) -> i32 {
        self.subsector_code
    }

    /// The `subsector_name` method returns the NAICS subsector code description.
    pub fn subsector_name(&self) -> Option<String> {
        self.subsector_name.clone()
    }

    /// The `tourism` method clones the value of the `tourism` field, which contains broad business categories used to drive symbology in a GIS map.
    pub fn tourism(&self) -> Option<String> {
        self.tourism.clone()
    }

    /// The `district` method returns the cloned value of the `district` field, which contains the
    /// district name associated with the GC zone, if located in GC.
    pub fn district(&self) -> Option<String> {
        self.district.clone()
    }
}

impl TryFrom<BusinessRaw> for Business {
    type Error = Bandage;

    // The `try_from` method does the heavy lifting converting a [`BusinessRaw`] struct to a
    // [`Business`] type.  Errors if the address parsing fails.
    fn try_from(raw: BusinessRaw) -> Result<Self, Self::Error> {
        // Attempt to parse the address label to a [`PartialAddress`].
        match parse_address(&raw.street_address_label) {
            // Return the conversion on success.
            Ok((_, address)) => Ok(Business {
                company_name: raw.company_name,
                contact_name: raw.contact_name,
                dba: raw.dba,
                address,
                license: raw.license,
                industry_code: raw.industry_code,
                industry_name: raw.industry_name,
                sector_code: raw.sector_code,
                sector_name: raw.sector_name,
                subsector_code: raw.subsector_code,
                subsector_name: raw.subsector_name,
                tourism: raw.tourism,
                district: raw.district,
            }),
            // Throw an error if parsing fails.
            Err(_) => Err(Bandage::Parse),
        }
    }
}

/// The `Businesses` struct is a wrapper around a vector of type [`Business`].
/// This struct contains business licenses that have mapped to valid addresses.
#[derive(Debug, Clone)]
pub struct Businesses {
    records: Vec<Business>,
}

impl Businesses {
    /// Writes the contents to a csv file at location `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Clean<Self> {
        let raw = BusinessesRaw::from_csv(path)?;
        let mut records = Vec::new();
        for record in raw.records {
            records.push(Business::try_from(record)?);
        }
        Ok(Businesses { records })
    }
}

// Since we are wrapping a vector, we want the struct to deref to the underlying vector.
impl ops::Deref for Businesses {
    type Target = Vec<Business>;

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

// We also implemented deref_mut to be able to modify the underlying vector.
impl ops::DerefMut for Businesses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.records
    }
}
