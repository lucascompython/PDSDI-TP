use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Brown,
    Black,
    White,
    Gold,
    Gray,
}

impl std::str::FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Color::Red),
            "Orange" => Ok(Color::Orange),
            "Yellow" => Ok(Color::Yellow),
            "Green" => Ok(Color::Green),
            "Blue" => Ok(Color::Blue),
            "Purple" => Ok(Color::Purple),
            "Pink" => Ok(Color::Pink),
            "Brown" => Ok(Color::Brown),
            "Black" => Ok(Color::Black),
            "White" => Ok(Color::White),
            "Gold" => Ok(Color::Gold),
            "Gray" => Ok(Color::Gray),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Category {
    Shirt,
    Pants,
    Shorts,
    Dress,
    Skirt,
    Jacket,
    Sweater,
    Shoes,
    Hat,
    Gloves,
    Scarf,
}

impl std::str::FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Shirt" => Ok(Category::Shirt),
            "Pants" => Ok(Category::Pants),
            "Shorts" => Ok(Category::Shorts),
            "Dress" => Ok(Category::Dress),
            "Skirt" => Ok(Category::Skirt),
            "Jacket" => Ok(Category::Jacket),
            "Sweater" => Ok(Category::Sweater),
            "Shoes" => Ok(Category::Shoes),
            "Hat" => Ok(Category::Hat),
            "Gloves" => Ok(Category::Gloves),
            "Scarf" => Ok(Category::Scarf),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Clothe {
    pub id: i32,
    pub name: String,
    pub color: Color,
    pub category: Category,
    pub user_id: i32,
    pub is_hot_weather: bool,
}
