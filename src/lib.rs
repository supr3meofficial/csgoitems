use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

/// Describes an ItemSkin rarity
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SkinRarity {
    Covert,
    Classified,
    Restricted,
    MilSpec,
    IndustrialGrade,
    ConsumerGrade,
    Other(String),
}

impl SkinRarity {
    fn from_str(str: &str) -> Self {
        match str {
            "Rare Special Items" => SkinRarity::Covert,
            "Covert Skins" => SkinRarity::Covert,
            "Classified Skins" => SkinRarity::Classified,
            "Restricted Skins" => SkinRarity::Restricted,
            "Mil-Spec Skins" => SkinRarity::MilSpec,
            "Industrial Grade Skins" => SkinRarity::IndustrialGrade,
            "Consumer Grade Skins" => SkinRarity::ConsumerGrade,
            _ => SkinRarity::Other(str.into()),
        }
    }
}

impl std::fmt::Display for SkinRarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkinRarity::Covert => write!(f, "Covert"),
            SkinRarity::Classified => write!(f, "Classified"),
            SkinRarity::Restricted => write!(f, "Restricted"),
            SkinRarity::MilSpec => write!(f, "Mil-Spec"),
            SkinRarity::IndustrialGrade => write!(f, "Industrial Grade"),
            SkinRarity::ConsumerGrade => write!(f, "Consumer Grade"),
            SkinRarity::Other(rarity) => write!(f, "Unimplemented rarity: {}", rarity),
        }
    }
}

/// Describes a possible `ItemSkin` wear.
/// Also, holds a link to the associated wear `ItemSkin` image.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SkinWear {
    FactoryNew(String),
    MinimalWear(String),
    FieldTested(String),
    WellWorn(String),
    BattleScarred(String),
    Vanilla(String),
}

impl std::fmt::Display for SkinWear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkinWear::FactoryNew(_) => write!(f, "Factory New"),
            SkinWear::MinimalWear(_) => write!(f, "Minimal Wear"),
            SkinWear::FieldTested(_) => write!(f, "Field-Tested"),
            SkinWear::WellWorn(_) => write!(f, "Well-Worn"),
            SkinWear::BattleScarred(_) => write!(f, "Battle-Scarred"),
            SkinWear::Vanilla(_) => write!(f, "Not Painted"),
        }
    }
}

impl SkinWear {
    fn new(wear_name: String, wear_img: String) -> Self {
        match wear_name.as_str() {
            "Factory New" => SkinWear::FactoryNew(wear_img),
            "Minimal Wear" => SkinWear::MinimalWear(wear_img),
            "Field-Tested" => SkinWear::FieldTested(wear_img),
            "Well-Worn" => SkinWear::WellWorn(wear_img),
            "Battle-Scarred" => SkinWear::BattleScarred(wear_img),
            _ => SkinWear::Vanilla(wear_img),
        }
    }
}

/// Describes an `ItemSkin` type
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ItemSkinType {
    Knife,
    Glove,
    Weapon,
}

impl std::fmt::Display for ItemSkinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemSkinType::Knife => write!(f, "Knife"),
            ItemSkinType::Glove => write!(f, "Gloves"),
            ItemSkinType::Weapon => write!(f, "Weapon"),
        }
    }
}

impl ItemSkinType {
    fn from_str(str: &str) -> Self {
        let item_types: Vec<&str> = vec!["Bayonet", "Karambit", "Daggers", "Knife", "Gloves"];
        let mut item_type = "Other";

        for itype in item_types.into_iter() {
            if str.contains(itype) {
                item_type = itype;
                break;
            }
        }

        match item_type {
            "Bayonet" => ItemSkinType::Knife,
            "Karambit" => ItemSkinType::Knife,
            "Daggers" => ItemSkinType::Knife,
            "Knife" => ItemSkinType::Knife,
            "Gloves" => ItemSkinType::Glove,
            _ => ItemSkinType::Weapon,
        }
    }
}

/// Represents an Item Skin
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ItemSkin {
    name: String,
    item_type: ItemSkinType,
    rarity: SkinRarity,
    description: String,
    lore: String,
    can_be_souvenir: bool,
    can_be_stattrak: bool,
    wears: Vec<SkinWear>,
}

impl ItemSkin {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn item_type(&self) -> &ItemSkinType {
        &self.item_type
    }
    pub fn rarity(&self) -> &SkinRarity {
        &self.rarity
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn lore(&self) -> &str {
        &self.lore
    }
    pub fn can_be_souvenir(&self) -> &bool {
        &self.can_be_souvenir
    }
    pub fn can_be_stattrak(&self) -> &bool {
        &self.can_be_stattrak
    }
    pub fn wears(&self) -> &Vec<SkinWear> {
        &self.wears
    }
}

// Constructors
impl ItemSkin {
    // Also takes in rarity due to the way the json is formatted
    fn from_json_value(rarity: SkinRarity, value: &JsonValue) -> Self {
        let name = value["name"].as_str().unwrap().to_string();
        let item_type = ItemSkinType::from_str(&name);
        let description = value["desc"].as_str().unwrap().to_string();
        let lore = value["lore"].as_str().unwrap().to_string();
        let can_be_souvenir = value["can_be_souvenir"].as_bool().unwrap();
        let can_be_stattrak = value["can_be_stattrak"].as_bool().unwrap();

        let wear_map = value["wears"].as_object().unwrap();
        let mut wears: Vec<SkinWear> = vec![];
        for (wear_name, wear_img) in wear_map {
            wears.push(SkinWear::new(
                wear_name.to_owned(),
                wear_img.as_str().unwrap().to_string(),
            ));
        }

        Self {
            name,
            item_type,
            rarity,
            description,
            lore,
            can_be_souvenir,
            can_be_stattrak,
            wears,
        }
    }
}

/// Describes an `ItemContainer` type
#[derive(Eq, PartialEq, Debug)]
pub enum ItemContainerType {
    Collection,
    SkinCase,
    SouvenirPackage,
    Package,
    Other(String),
}

impl ItemContainerType {
    fn from_str(str: &str) -> Self {
        let item_types: Vec<&str> = vec!["Collection", "Case", "Souvenir Package", "Package"];
        let mut ct = "Unknown";

        for item_type in item_types.into_iter() {
            if str.contains(item_type) {
                ct = item_type;
                break;
            }
        }

        match ct {
            "Collection" => ItemContainerType::Collection,
            "Case" => ItemContainerType::SkinCase,
            "Souvenir" => ItemContainerType::SouvenirPackage,
            "Package" => ItemContainerType::Package,
            _ => ItemContainerType::Other(str.into()),
        }
    }
}

/// Represents an Item Container
#[derive(Eq, PartialEq, Debug)]
pub struct ItemContainer {
    name: String,
    item_type: ItemContainerType,
    icon: String,
    content: Vec<ItemSkin>,
}

impl ItemContainer {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn item_type(&self) -> &ItemContainerType {
        &self.item_type
    }
    pub fn icon(&self) -> &str {
        &self.icon
    }
    pub fn content(&self) -> &Vec<ItemSkin> {
        &self.content
    }
}

impl ItemContainer {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ItemContainer, Box<dyn std::error::Error>> {
        // Read the file to a string
        let data = fs::read(path)?;

        // Read the JSON contents of the file
        let data: JsonValue = serde_json::from_slice(&data)?;

        let name = data["name"].as_str().unwrap().to_string();
        let item_type = ItemContainerType::from_str(&name);
        let icon = data["image_url"].as_str().unwrap().to_string();
        let mut content: Vec<ItemSkin> = vec![];

        let map_content = data["content"].as_object().unwrap();
        for (rarity_group_string, skin_array) in map_content {
            let skin_array = skin_array.as_array().unwrap();

            for skin_obj in skin_array {
                let rarity = SkinRarity::from_str(rarity_group_string);
                content.push(ItemSkin::from_json_value(rarity, skin_obj));
            }
        }

        // Return the ItemContainer.
        Ok(ItemContainer {
            name,
            item_type,
            icon,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ItemContainer;
    use crate::ItemContainerType;

    #[test]
    fn it_works() {
        let c = ItemContainer::from_file("resources/cases/json/x-ray_p250_package.json").unwrap();
        assert_eq!(c.name, "X-Ray P250 Package");
        assert_eq!(c.item_type, ItemContainerType::Package);
        assert_eq!(c.icon, "https://steamcommunity-a.akamaihd.net/economy/image/-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXU5A1PIYQNqhpOSV-fRPasw8rsS091PDtH5O_1FAthwfTNP2kTvomzzYHdlqLxZb7XlW4IuJwk3u2S8NWl3QS1_EA6YT2iddeXdgIgIQaHHJc2aVo/256fx256f");
    }

    #[test]
    fn json_format_1() {
        let c = ItemContainer::from_file("resources/cases/json/glove_case.json").unwrap();
        println!("{:#?}", c);
    }

    #[test]
    fn json_format_2() {
        let c = ItemContainer::from_file("resources/cases/json/x-ray_p250_package.json").unwrap();
        println!("{:#?}", c);
    }
}
