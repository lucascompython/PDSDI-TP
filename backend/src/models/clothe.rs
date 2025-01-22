use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

impl From<i16> for Color {
    fn from(value: i16) -> Self {
        match value {
            0 => Color::Red,
            1 => Color::Orange,
            2 => Color::Yellow,
            3 => Color::Green,
            4 => Color::Blue,
            5 => Color::Purple,
            6 => Color::Pink,
            7 => Color::Brown,
            8 => Color::Black,
            9 => Color::White,
            10 => Color::Gold,
            11 => Color::Gray,
            _ => panic!("Invalid color value"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

impl From<i16> for Category {
    fn from(value: i16) -> Self {
        match value {
            0 => Category::Shirt,
            1 => Category::Pants,
            2 => Category::Shorts,
            3 => Category::Dress,
            4 => Category::Skirt,
            5 => Category::Jacket,
            6 => Category::Sweater,
            7 => Category::Shoes,
            8 => Category::Hat,
            9 => Category::Gloves,
            10 => Category::Scarf,
            _ => panic!("Invalid category value"),
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
