// TODO: Make this multithreaded
// TODO: Make this nostd
// TODO: Possible remove the filename field and just make it ext
// TODO: Fix the typescript types
use std::io::{self, Read, Write};

#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen,
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Clothe {
    pub id: u16, // 2 bytes
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub name: String, // Variable size, 1 byte for size
    pub color: u8, // 4 bits
    pub category: u8, // 4 bits
    pub user_id: u16, // 15 bits
    pub is_for_hot_weather: bool, // 1 bit
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub file_name: String, // Variable size, 1 byte for size
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub file: Vec<u8>, // Variable size, 3 bytes for size
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Clothe {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: u16,
        name: String,
        color: u8,
        category: u8,
        user_id: u16,
        is_for_hot_weather: bool,
        file_name: String,
        file: Vec<u8>,
    ) -> Self {
        Self {
            id,
            name,
            color,
            category,
            user_id,
            is_for_hot_weather,
            file_name,
            file,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }

    #[wasm_bindgen(getter)]
    pub fn file(&self) -> Vec<u8> {
        self.file.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_file(&mut self, file: Vec<u8>) {
        self.file = file;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer).unwrap();
        buffer
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        let mut reader: &[u8] = data;
        Clothe::deserialize(&mut reader).unwrap()
    }
}

impl Clothe {
    pub fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.id.to_be_bytes())?; // id (2 bytes)
        writer.write_all(&[self.name.len() as u8])?; // name size
        writer.write_all(self.name.as_bytes())?; // name

        let combined = (self.color << 4) | self.category; // color and category (1 byte)
        writer.write_all(&[combined])?;

        let user_id_with_flag = ((self.user_id & 0x7FFF) << 1) | (self.is_for_hot_weather as u16); // user_id and flag
        writer.write_all(&user_id_with_flag.to_be_bytes())?;

        writer.write_all(&[self.file_name.len() as u8])?; // file name size
        writer.write_all(self.file_name.as_bytes())?; // file name

        let file_size = self.file.len();
        writer.write_all(&(file_size as u32).to_be_bytes()[1..])?; // file size (3 bytes)
        writer.write_all(&self.file)?; // file

        Ok(())
    }

    pub fn deserialize(reader: &mut impl Read) -> io::Result<Self> {
        let mut id_bytes = [0; 2];
        reader.read_exact(&mut id_bytes)?;
        let id = u16::from_be_bytes(id_bytes);

        let mut name_size = [0; 1];
        reader.read_exact(&mut name_size)?;
        let mut name_bytes = vec![0; name_size[0] as usize];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes).unwrap();

        let mut combined = [0; 1];
        reader.read_exact(&mut combined)?;
        let color = combined[0] >> 4;
        let category = combined[0] & 0x0F;

        let mut user_id_bytes = [0; 2];
        reader.read_exact(&mut user_id_bytes)?;
        let user_id_with_flag = u16::from_be_bytes(user_id_bytes);
        let user_id = user_id_with_flag >> 1;
        let is_for_hot_weather = (user_id_with_flag & 0x1) != 0;

        let mut file_name_size = [0; 1];
        reader.read_exact(&mut file_name_size)?;
        let mut file_name_bytes = vec![0; file_name_size[0] as usize];
        reader.read_exact(&mut file_name_bytes)?;
        let file_name = String::from_utf8(file_name_bytes).unwrap();

        let mut file_size_bytes = [0; 3];
        reader.read_exact(&mut file_size_bytes)?;
        let file_size = u32::from_be_bytes([
            0,
            file_size_bytes[0],
            file_size_bytes[1],
            file_size_bytes[2],
        ]) as usize;
        let mut file = vec![0; file_size];
        reader.read_exact(&mut file)?;

        Ok(Self {
            id,
            name,
            color,
            category,
            user_id,
            is_for_hot_weather,
            file_name,
            file,
        })
    }
}

pub fn serialize_clothes(clothes: &[Clothe], writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(&[clothes.len() as u8])?;
    for clothe in clothes {
        clothe.serialize(writer)?;
    }
    Ok(())
}

pub fn deserialize_clothes(reader: &mut impl Read) -> io::Result<Vec<Clothe>> {
    let mut num_clothes = [0; 1];
    reader.read_exact(&mut num_clothes)?;
    let mut clothes = Vec::with_capacity(num_clothes[0] as usize);
    for _ in 0..num_clothes[0] {
        clothes.push(Clothe::deserialize(reader)?);
    }
    Ok(clothes)
}

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn serializeClothesToBytes(clothes: JsValue) -> Vec<u8> {
    let clothes: Vec<Clothe> = serde_wasm_bindgen::from_value(clothes).unwrap();

    let mut buffer = Vec::new();
    serialize_clothes(&clothes, &mut buffer).unwrap();
    buffer
}

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn deserializeClothesFromBytes(data: &[u8]) -> Vec<Clothe> {
    let mut reader: &[u8] = data;
    deserialize_clothes(&mut reader).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_single_clothe() {
        let clothe = Clothe {
            id: 12345,
            name: "T-Shirt".to_string(),
            color: 2,
            category: 3,
            user_id: 105,
            is_for_hot_weather: true,
            file_name: "tshirt.png".to_string(),
            file: vec![1, 2, 3, 4, 5],
        };

        let mut buffer = Vec::new();
        clothe.serialize(&mut buffer).unwrap();

        let mut reader: &[u8] = &buffer;
        let deserialized = Clothe::deserialize(&mut reader).unwrap();

        assert_eq!(clothe, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_multiple_clothes() {
        let clothes = vec![
            Clothe {
                id: 1,
                name: "T-Shirt".to_string(),
                color: 2,
                category: 3,
                user_id: 100,
                is_for_hot_weather: true,
                file_name: "tshirt.png".to_string(),
                file: vec![1, 2, 3, 4, 5],
            },
            Clothe {
                id: 2,
                name: "Jacket".to_string(),
                color: 1,
                category: 4,
                user_id: 101,
                is_for_hot_weather: false,
                file_name: "jacket.png".to_string(),
                file: vec![6, 7, 8, 9, 10],
            },
        ];

        let mut buffer = Vec::new();
        serialize_clothes(&clothes, &mut buffer).unwrap();

        let mut reader: &[u8] = &buffer;
        let deserialized = deserialize_clothes(&mut reader).unwrap();

        assert_eq!(clothes, deserialized);
    }
}
