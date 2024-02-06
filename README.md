# mio

This is a RESTFUl API powering `Bookmark Collection`, a web extension that lets you share your favorite URLs. You can check out the extension [here](https://github.com/dxphilo/bookmark-collector).

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- MySQL database

## Getting Started

### Setting up the Database

1. Create a MySQL database for your project:

   ```sql
   CREATE DATABASE your_database_name;


Copy the `.env.sample` file to `.env` and update the database connection details:

```
DATABASE_URL=mysql://username:password@localhost/your_database_name
```
Replace `username`, `password`, and `your_database_name` with your MySQL credentials.


### Generating Migration
Run the following command to generate a migration for the links table:

```
cargo run --bin migration
```
This will create a new migration file in the migrations directory.

### Applying Migration
Apply the migration to create the links table:

```
cargo run --bin migration -- migrate
```
### Starting the Project

Installing crates
```
cargo build
```

Run the project with:

```
cargo run
```
The project will start, and you can interact with the API.

The backend provides two endpoints:

- GET `/links`: Retrieve a list of shared links.

### Success Response:

```
[
  {
    "id": 1,
    "link": "https://www.julian.com/about",
    "count": 58,
    "created_at": "2024-02-03T23:05:37Z",
    "updated_at": "2024-02-03T23:07:16Z"
  },
  {
    "id": 2,
    "link": "https://www.julian.com",
    "count": 18,
    "created_at": "2024-02-03T23:16:12Z",
    "updated_at": "2024-02-03T23:16:22Z"
  },
  {
    "id": 3,
    "link": "https://www.julian.com/newsletter",
    "count": 36,
    "created_at": "2024-02-03T23:16:35Z",
    "updated_at": "2024-02-03T23:55:07Z"
  }
]
```

### error response

```
{
"error": "error that occured"
}

```

- POST `/links`: Add a new link to the collection.

Request Body Schema:

```
{
  "link": "https://awesome-link.com"
}
```
#### Success Response:

```
// Expected Success Response
{
  "id": 123,
  "link": "https://example.com",
  "count": 1,
  "created_at": "2024-02-03T12:34:56Z",
  "updated_at": "2024-02-03T12:34:56Z"
}

```
#### Error Response:

```
// Expected Error Response
{
  "error": "Failed to share the link. Please try again."
}

```



### Project Structure
`src/:` Source code directory.
`migrations/:` Directory containing database migration files.
Database Schema
sql
```
CREATE TABLE links (
    id INT AUTO_INCREMENT PRIMARY KEY,
    link VARCHAR(255) NOT NULL,
    count INT DEFAULT 1 NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```
Adjust the schema according to your requirements.

### Contributing
Feel free to contribute to the project by opening issues or submitting pull requests.

### License
This project is licensed under the MIT License.




