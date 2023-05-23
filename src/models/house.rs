// enum PropertyType {
//     Apartment,
//     House,
//     Other,
// }

#[derive(Serialize, Deserialize)]
pub struct House {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub number: i32,
    pub floor: i32,
    pub postal_code: i32,
    pub square_meters: i32,
    pub number_of_bathrooms: i32,
    pub number_of_rooms: i32,
    pub property_type: String,
}
