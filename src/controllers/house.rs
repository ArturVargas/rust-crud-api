use postgres::{Client, NoTls};

use crate::models::house::House;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const DB_URL: &str = "postgresql://postgres:postgres@db:5432/postgres";
// let mut client = Client::connect("postgresql://postgres:postgres@db:5432/postgres", NoTls)?;

//handle post request
pub fn handle_post_request(request: &str) -> (String, String) {
    println!("{} -->>>", request);
    match (
        get_property_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(data), Ok(mut client)) => {
            println!("{:?}", data.city);
            client
                .execute(
                  "INSERT INTO houses (street, city, number, floor, postal_code, square_meters, number_of_bathrooms, number_of_rooms, property_type) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    &[&data.street, &data.city, &data.number, &data.floor, &data.postal_code, &data.square_meters, &data.number_of_bathrooms, &data.number_of_rooms, &data.property_type],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Property created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle get request
pub fn handle_get_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM houses WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let house = House {
                        id: row.get(0),
                        street: row.get(1),
                        city: row.get(2),
                        number: row.get(3),
                        floor: row.get(4),
                        postal_code: row.get(5),
                        square_meters: row.get(6),
                        number_of_bathrooms: row.get(7),
                        number_of_rooms: row.get(8),
                        property_type: row.get(9),
                    };

                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&house).unwrap(),
                    )
                }
                _ => (NOT_FOUND.to_string(), "Property not found".to_string()),
            }
        }

        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle get all request
pub fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut houses = Vec::new();

            for row in client.query("SELECT * FROM houses", &[]).unwrap() {
                houses.push(House {
                    id: row.get(0),
                    street: row.get(1),
                    city: row.get(2),
                    number: row.get(3),
                    floor: row.get(4),
                    postal_code: row.get(5),
                    square_meters: row.get(6),
                    number_of_bathrooms: row.get(7),
                    number_of_rooms: row.get(8),
                    property_type: row.get(9),
                });
            }

            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&houses).unwrap(),
            )
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle put request
pub fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        get_property_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(house), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE houses SET street = $1, city = $2, number = $3, floor = $4, postal_code = $5, square_meters = $6, number_of_bathrooms = $7, number_of_rooms = $8, property_type = $9, WHERE id = $10",
                    &[&house.street, &house.city, &house.number, &house.floor, &house.postal_code, &house.square_meters, &house.number_of_bathrooms, &house.number_of_rooms, &house.property_type, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Property updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//handle delete request
pub fn handle_delete_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client
                .execute("DELETE FROM houses WHERE id = $1", &[&id])
                .unwrap();

            //if rows affected is 0, property not found
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "Property not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Property deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

//Get id from request URL
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

//deserialize property from request body without id
fn get_property_request_body(request: &str) -> Result<House, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
