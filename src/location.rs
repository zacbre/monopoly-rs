use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::Player;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
pub enum LocationEnum {
    Go = 1,
    MediterraneanAvenue = 2,
    CommunityChest1 = 3,
    BalticAvenue = 4,
    IncomeTax = 5,
    ReadingRailroad = 6,
    OrientalAvenue = 7,
    Chance1 = 8,
    VermontAvenue = 9,
    ConnecticutAvenue = 10,
    Jail = 11,
    StCharlesPlace = 12,
    ElectricCompany = 13,
    StateAvenue = 14,
    VirginiaAvenue = 15,
    PennsylvaniaRailroad = 16,
    StJamesPlace = 17,
    CommunityChest2 = 18,
    TennesseeAvenue = 19,
    NewYorkAvenue = 20,
    FreeParking = 21,
    KentuckyAvenue = 22,
    Chance2 = 23,
    IndianaAvenue = 24,
    IllinoisAvenue = 25,
    BnORailroad = 26,
    AtlanticAvenue = 27,
    VentnorAvenue = 28,
    WaterWorks = 29,
    MarvinGardens = 30,
    GoToJail = 31,
    PacificAvenue = 32,
    NorthCarolinaAvenue = 33,
    CommunityChest3 = 34,
    PennsylvaniaAvenue = 35,
    ShortLine = 36,
    Chance3 = 37,
    ParkPlace = 38,
    LuxuryTax = 39,
    Boardwalk = 40,
}

#[derive(PartialEq)]
pub enum LocationTypeEnum {
    Go,
    Property,
    Tax,
    None,
    Chance,
    CommunityChest,
    Railroad,
    Utility,
    Jail
}

impl LocationEnum {
    pub fn get_location_type(&self) -> LocationTypeEnum {
        match self {
            LocationEnum::Go => LocationTypeEnum::Go,
            LocationEnum::MediterraneanAvenue => LocationTypeEnum::Property,
            LocationEnum::CommunityChest1 => LocationTypeEnum::CommunityChest,
            LocationEnum::BalticAvenue => LocationTypeEnum::Property,
            LocationEnum::IncomeTax => LocationTypeEnum::Tax,
            LocationEnum::ReadingRailroad => LocationTypeEnum::Railroad,
            LocationEnum::OrientalAvenue => LocationTypeEnum::Property,
            LocationEnum::Chance1 => LocationTypeEnum::Chance,
            LocationEnum::VermontAvenue => LocationTypeEnum::Property,
            LocationEnum::ConnecticutAvenue => LocationTypeEnum::Property,
            LocationEnum::Jail => LocationTypeEnum::Jail,
            LocationEnum::StCharlesPlace => LocationTypeEnum::Property,
            LocationEnum::ElectricCompany => LocationTypeEnum::Utility,
            LocationEnum::StateAvenue => LocationTypeEnum::Property,
            LocationEnum::VirginiaAvenue => LocationTypeEnum::Property,
            LocationEnum::PennsylvaniaRailroad => LocationTypeEnum::Railroad,
            LocationEnum::StJamesPlace => LocationTypeEnum::Property,
            LocationEnum::CommunityChest2 => LocationTypeEnum::CommunityChest,
            LocationEnum::TennesseeAvenue => LocationTypeEnum::Property,
            LocationEnum::NewYorkAvenue => LocationTypeEnum::Property,
            LocationEnum::FreeParking => LocationTypeEnum::None,
            LocationEnum::KentuckyAvenue => LocationTypeEnum::Property,
            LocationEnum::Chance2 => LocationTypeEnum::Chance,
            LocationEnum::IndianaAvenue => LocationTypeEnum::Property,
            LocationEnum::IllinoisAvenue => LocationTypeEnum::Property,
            LocationEnum::BnORailroad => LocationTypeEnum::Railroad,
            LocationEnum::AtlanticAvenue => LocationTypeEnum::Property,
            LocationEnum::VentnorAvenue => LocationTypeEnum::Property,
            LocationEnum::WaterWorks => LocationTypeEnum::Utility,
            LocationEnum::MarvinGardens => LocationTypeEnum::Property,
            LocationEnum::GoToJail => LocationTypeEnum::None,
            LocationEnum::PacificAvenue => LocationTypeEnum::Property,
            LocationEnum::NorthCarolinaAvenue => LocationTypeEnum::Property,
            LocationEnum::CommunityChest3 => LocationTypeEnum::CommunityChest,
            LocationEnum::PennsylvaniaAvenue => LocationTypeEnum::Property,
            LocationEnum::ShortLine => LocationTypeEnum::Railroad,
            LocationEnum::Chance3 => LocationTypeEnum::Chance,
            LocationEnum::ParkPlace => LocationTypeEnum::Property,
            LocationEnum::LuxuryTax => LocationTypeEnum::Tax,
            LocationEnum::Boardwalk => LocationTypeEnum::Property
        }
    }


}

impl From<usize> for LocationEnum {
    fn from(value: usize) -> Self {
        match value {
            1 => LocationEnum::Go,
            2 => LocationEnum::MediterraneanAvenue,
            3 => LocationEnum::CommunityChest1,
            4 => LocationEnum::BalticAvenue,
            5 => LocationEnum::IncomeTax,
            6 => LocationEnum::ReadingRailroad,
            7 => LocationEnum::OrientalAvenue,
            8 => LocationEnum::Chance1,
            9 => LocationEnum::VermontAvenue,
            10 => LocationEnum::ConnecticutAvenue,
            11 => LocationEnum::Jail,
            12 => LocationEnum::StCharlesPlace,
            13 => LocationEnum::ElectricCompany,
            14 => LocationEnum::StateAvenue,
            15 => LocationEnum::VirginiaAvenue,
            16 => LocationEnum::PennsylvaniaRailroad,
            17 => LocationEnum::StJamesPlace,
            18 => LocationEnum::CommunityChest2,
            19 => LocationEnum::TennesseeAvenue,
            20 => LocationEnum::NewYorkAvenue,
            21 => LocationEnum::FreeParking,
            22 => LocationEnum::KentuckyAvenue,
            23 => LocationEnum::Chance2,
            24 => LocationEnum::IndianaAvenue,
            25 => LocationEnum::IllinoisAvenue,
            26 => LocationEnum::BnORailroad,
            27 => LocationEnum::AtlanticAvenue,
            28 => LocationEnum::VentnorAvenue,
            29 => LocationEnum::WaterWorks,
            30 => LocationEnum::MarvinGardens,
            31 => LocationEnum::GoToJail,
            32 => LocationEnum::PacificAvenue,
            33 => LocationEnum::NorthCarolinaAvenue,
            34 => LocationEnum::CommunityChest3,
            35 => LocationEnum::PennsylvaniaAvenue,
            36 => LocationEnum::ShortLine,
            37 => LocationEnum::Chance3,
            38 => LocationEnum::ParkPlace,
            39 => LocationEnum::LuxuryTax,
            40 => LocationEnum::Boardwalk,
            _ => panic!("No location found for this number!")
        }
    }
}

pub struct Property {
    location: LocationEnum,
    location_type: LocationTypeEnum,
    cost: i32,
    owned_by_player_number: Option<usize>,
    houses: i32,
    hotels: i32,
    custom_fn: fn(&mut Player) -> bool,
}

impl Property {
    pub fn new(location: LocationEnum,
               location_type: LocationTypeEnum,
               cost: i32,
               custom_fn: fn(&mut Player) -> bool) -> Self {
        Self {
            location,
            location_type,
            cost,
            owned_by_player_number: None,
            houses: 0,
            hotels: 0,
            custom_fn
        }
    }

    pub fn run_fn(&self, player: &mut Player) -> bool {
        (&self.custom_fn)(player)
    }
}

pub struct Properties {
    pub list: HashMap<usize, Property>
}

impl Properties {
    pub fn add(&mut self, property: Property) {
        self.list.insert(self.list.len() + 1, property);
    }

    pub fn get_property(&mut self, location: LocationEnum) -> &mut Property {
        match self.list.entry(location as usize) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(_) => panic!("Could not find property!")
        }
    }

    pub fn get_new_list() -> Self {
        let mut init = Self {
            list: HashMap::new()
        };
        init.add(Property::new(LocationEnum::Go, LocationTypeEnum::Go, 0, |player| { true }));
        init.add(Property::new(LocationEnum::MediterraneanAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::CommunityChest1, LocationTypeEnum::CommunityChest, 0, |player| { true }));
        init.add(Property::new(LocationEnum::BalticAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::IncomeTax, LocationTypeEnum::Tax, 0, |player| { true }));
        init.add(Property::new(LocationEnum::ReadingRailroad, LocationTypeEnum::Railroad, 0, |player| { true }));
        init.add(Property::new(LocationEnum::OrientalAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::Chance1, LocationTypeEnum::Chance, 0, |player| { true }));
        init.add(Property::new(LocationEnum::VermontAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::ConnecticutAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::Jail, LocationTypeEnum::Jail, 0, |player| { true }));
        init.add(Property::new(LocationEnum::StCharlesPlace, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::ElectricCompany, LocationTypeEnum::Utility, 0, |player| { true }));
        init.add(Property::new(LocationEnum::StateAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::VirginiaAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::PennsylvaniaRailroad, LocationTypeEnum::Railroad, 0, |player| { true }));
        init.add(Property::new(LocationEnum::StJamesPlace, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::CommunityChest2, LocationTypeEnum::CommunityChest, 0, |player| { true }));
        init.add(Property::new(LocationEnum::TennesseeAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::NewYorkAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::FreeParking, LocationTypeEnum::None, 0, |player| { true }));
        init.add(Property::new(LocationEnum::KentuckyAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::Chance2, LocationTypeEnum::Chance, 0, |player| { true }));
        init.add(Property::new(LocationEnum::IndianaAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::IllinoisAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::BnORailroad, LocationTypeEnum::Railroad, 0, |player| { true }));
        init.add(Property::new(LocationEnum::AtlanticAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::VentnorAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::WaterWorks, LocationTypeEnum::Utility, 0, |player| { true }));
        init.add(Property::new(LocationEnum::MarvinGardens, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::GoToJail, LocationTypeEnum::None, 0, |player| { true }));
        init.add(Property::new(LocationEnum::PacificAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::NorthCarolinaAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::CommunityChest3, LocationTypeEnum::CommunityChest, 0, |player| { true }));
        init.add(Property::new(LocationEnum::PennsylvaniaAvenue, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::ShortLine, LocationTypeEnum::Railroad, 0, |player| { true }));
        init.add(Property::new(LocationEnum::Chance3, LocationTypeEnum::Chance, 0, |player| { true }));
        init.add(Property::new(LocationEnum::ParkPlace, LocationTypeEnum::Property, 0, |player| { true }));
        init.add(Property::new(LocationEnum::LuxuryTax, LocationTypeEnum::Tax, 0, |player| { true }));
        init.add(Property::new(LocationEnum::Boardwalk, LocationTypeEnum::Property, 0, |player| { true }));

        init
    }
}