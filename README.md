# CRUD API

PORT: 8080

## ENDPOINTS

### GET

```bash
http://localhost:8080/properties
```

Result:
``[]``

### GET

```bash
http://localhost:8080/property/{id}
```

Result:

```json
[{
  "id": 1,
  "street": "av 9",
   "city": "Mexico",
   "number": 3,
   "floor": 2,
   "postal_code": 527,
   "square_meters": 125.30,
   "number_of_bathrooms": 3,
   "number_of_rooms": 4,
   "property_type": "Apartment"
}]
```

### POST

```bash
http://localhost:8080/property
```

**BODY**:

```json
{
  "id": 1,
  "street": "av 9",
  "city": "Mexico",
  "number": 3,
  "floor": 2,
  "postal_code": 527,
  "square_meters": 125.30,
  "number_of_bathrooms": 3,
  "number_of_rooms": 4,
  "property_type": "Apartment"
}
```

## Run With Docker Compose

```bash
docker compose up
```

Runs de PostgresDB and backend services
