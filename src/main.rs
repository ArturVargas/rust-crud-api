use controllers::house::{
    handle_delete_request, handle_get_all_request, handle_get_request, handle_post_request,
    handle_put_request,
};
use log::{debug, error, info, log_enabled, Level};
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod controllers;
mod models;

#[macro_use]
extern crate serde_derive;

/// Client is used to connect to the database.
/// NoTls is used to connect to the database without TLS.
/// PostgresError is the error type returned by the Postgres driver.
/// TcpListener and TcpStream to create a TCP server.
/// Read and Write are used to read and write from a TCP stream.
/// env is used to read the environment variables.
/// the #[macro_use] attribute is used to import the serde_derive macro.
/// We will use it to derive our model's Serialize and Deserialize traits.

//Model: students struct with id, name, email, address
// #[derive(Serialize, Deserialize)]
// struct Student {
//     id: Option<i32>,
//     name: String,
//     email: String,
//     address: String,
// }

/// CONSTANTS
///
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    env_logger::init();
    //Set Database
    if let Err(e) = set_database() {
        error!("Error setting database: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    info!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                error!("Unable to connect: {}", e);
            }
        }
    }
}

//handle requests
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /property") => handle_post_request(r),
                r if r.starts_with("GET /property/") => handle_get_request(r),
                r if r.starts_with("GET /properties") => handle_get_all_request(r),
                r if r.starts_with("PUT /property/") => handle_put_request(r),
                r if r.starts_with("DELETE /property/") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

//db setup
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect("postgresql://postgres:postgres@db:5432/postgres", NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS houses (
            id SERIAL PRIMARY KEY,
            street VARCHAR NOT NULL,
            city VARCHAR NOT NULL,
            number integer NOT NULL,
            floor integer NOT NULL,
            postal_code integer NOT NULL,
            square_meters decimal NOT NULL,
            number_of_bathrooms integer NOT NULL,
            number_of_rooms integer NOT NULL,
            property_type VARCHAR NOT NULL
        )
        ",
    )?;
    Ok(())
}
