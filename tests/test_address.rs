use address::address_components::*;
use address::business::*;
use address::data::*;
use tracing::info;

#[test]
fn load_city_addresses() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");

    let file = "c:/users/erose/documents/addresses_20230411.csv";
    let addresses = CityAddresses::from_csv(file)?;
    info!(
        "City addresses loaded: {} entries.",
        addresses.records.len()
    );
    Ok(())
}

#[test]
fn load_county_addresses() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");

    let file = "p:/county_addresses.csv";
    let addresses = CountyAddresses::from_csv(file)?;
    info!(
        "City addresses loaded: {} entries.",
        addresses.records.len()
    );
    info!("Record 1059:");
    info!("{:?}", addresses.records[1058]);
    info!("Record 28091:");
    info!("{:?}", addresses.records[28090]);
    info!("Record 19424:");
    info!("{:?}", addresses.records[19423]);
    Ok(())
}

#[test]
fn read_gp2022_addresses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file = "c:/users/erose/Documents/addresses_2022.csv";
    let addresses = GrantsPass2022Addresses::from_csv(file)?;
    info!(
        "City addresses loaded: {} entries.",
        addresses.records.len()
    );
    let addresses = Addresses::from(addresses);
    info!("Addresses converted: {} entries.", addresses.records.len());
    Ok(())
}

#[test]
fn read_business_licenses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file = "c:/users/erose/Documents/active_business_licenses.csv";
    let licenses = BusinessLicenses::from_csv(file)?;
    info!(
        "Business licenses loaded: {} entries.",
        licenses.records.len()
    );
    // info!("Record 171:");
    // info!("{:?}", licenses.records[171]);
    assert!(licenses.records[0].pre_directional() == Some(StreetNamePreDirectional::NORTHEAST));
    info!("NE reads.");
    assert!(licenses.records[3].pre_directional() == Some(StreetNamePreDirectional::NORTHWEST));
    info!("NW reads.");
    assert!(licenses.records[1].pre_directional() == Some(StreetNamePreDirectional::SOUTHEAST));
    info!("SE reads.");
    assert!(licenses.records[109].pre_directional() == Some(StreetNamePreDirectional::SOUTHEAST));
    info!("SOUTHEAST reads.");
    assert!(licenses.records[0].post_type() == Some(StreetNamePostType::STREET));
    info!("ST reads.");
    assert!(licenses.records[1].post_type() == Some(StreetNamePostType::STREET));
    info!("St reads.");
    assert!(licenses.records[109].post_type() == Some(StreetNamePostType::STREET));
    info!("STREET reads.");
    assert!(licenses.records[171].post_type() == Some(StreetNamePostType::AVENUE));
    info!("Ave reads.");
    assert!(licenses.records[88].post_type() == Some(StreetNamePostType::BOULEVARD));
    info!("BOULEVARD reads.");
    assert!(licenses.records[134].post_type() == Some(StreetNamePostType::DRIVE));
    info!("Dr reads.");
    assert!(licenses.records[5].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("HWY reads.");
    assert!(licenses.records[214].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("Hwy reads.");
    assert!(licenses.records[405].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("HIGHWAY reads.");
    Ok(())
}

#[test]
fn match_city_address() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::new(
        &source_addresses.records[0].clone(),
        &target_addresses.records,
    );
    info!("Record 0 is: {:?}", match_records.records[0]);

    Ok(())
}

#[test]
fn match_business_addresses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");
    let business_path = "c:/users/erose/documents/active_business_licenses.csv";
    let city_path = "c:/users/erose/documents/addresses_20230411.csv";
    let business_addresses = BusinessLicenses::from_csv(business_path)?;
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let target_addresses = Addresses::from(city_addresses);
    let match_records = BusinessMatchRecords::compare(&business_addresses, &target_addresses);
    info!("Records: {:?}", match_records.records.len());

    Ok(())
}

#[test]
fn match_business_address_chain() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");
    let business_path = "c:/users/erose/documents/active_business_licenses.csv";
    let city_path = "c:/users/erose/documents/addresses_20230411.csv";
    let city2022_path = "c:/users/erose/documents/addresses_2022.csv";
    let business_addresses = BusinessLicenses::from_csv(business_path)?;
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let city2022_addresses = GrantsPass2022Addresses::from_csv(city2022_path)?;
    let target_addresses = Addresses::from(city_addresses);
    let other_addresses = Addresses::from(city2022_addresses);
    let match_records = BusinessMatchRecords::compare_chain(
        &business_addresses,
        &[&target_addresses, &other_addresses],
    );
    info!("Records: {:?}", match_records.records.len());

    Ok(())
}

#[test]
fn match_city_addresses() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::compare(
        &source_addresses.records[(0..10)].to_vec(),
        &target_addresses.records,
    );
    info!("Records: {:?}", match_records.records);

    Ok(())
}

#[test]
fn filter_status() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::compare(
        &source_addresses.records[(0..10)].to_vec(),
        &target_addresses.records,
    );
    let filtered = match_records.filter("status");
    info!("Records: {:?}", filtered.records);

    Ok(())
}

#[test]
fn filter_missing() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::compare(
        &source_addresses.records[(0..100)].to_vec(),
        &target_addresses.records,
    );
    let filtered = match_records.filter("missing");
    info!("Records: {:?}", filtered.records);

    Ok(())
}
