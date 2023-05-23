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
    pub number: i16,
    pub floor: i16,
    pub postal_code: i16,
    pub square_meters: f32,
    pub number_of_bathrooms: i16,
    pub number_of_rooms: i16,
    pub property_type: String,
}
