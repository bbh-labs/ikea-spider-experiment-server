extern crate iron;
extern crate router;
extern crate persistent;
extern crate postgres;
extern crate chrono;
extern crate rustc_serialize;

// Iron
use iron::prelude::*;
use iron::typemap::Key;

// Router
use router::Router;

// Persistent
use persistent::Write;

// Postgres
use postgres::{Connection, SslMode};

// Chrono
use chrono::*;

// JSON
use rustc_serialize::json;

#[derive(Copy, Clone)]
pub struct DatabaseConnection;

impl Key for DatabaseConnection { type Value = Connection; }

#[derive(RustcEncodable)]
struct Product {
    id: String,
    name: String,
    typ: String,
    country: String,
    unit: String,
    price: String,
    metric: String,
    image_url: String,
    url: String,
	department: String,
	category: String,
	subcategory: String,
	department_url: String,
	category_url: String,
	subcategory_url: String,
	created_at: DateTime<UTC>,
	updated_at: DateTime<UTC>,
}

fn products_handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok)))
}

fn product_handler(req: &mut Request) -> IronResult<Response> {
    // Get database handle
    let mutex = req.get::<Write<DatabaseConnection>>().unwrap();
    let conn = mutex.lock().unwrap();

    let ref query = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    for row in &conn.query("SELECT * FROM product WHERE id = $1 LIMIT 1", &[&query]).unwrap() {
        let product = Product {
            id: row.get(0),
            name: row.get(1),
            typ: row.get(2),
            country: row.get(3),
            price: row.get(4),
            unit: row.get(5),
            metric: row.get(6),
            url: row.get(7),
            image_url: row.get(8),
            department: row.get(9),
            category: row.get(10),
            subcategory: row.get(11),
            department_url: row.get(12),
            category_url: row.get(13),
            subcategory_url: row.get(14),
            created_at: row.get(15),
            updated_at: row.get(16),
        };

        if let Ok(json_output) = json::encode(&product) {
            return Ok(Response::with((iron::status::Ok, json_output)));
        }
    }

    Ok(Response::with((iron::status::NotFound, "")))
}

fn main() {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None).unwrap();

    let mut router = Router::new();
    router.get("/products", products_handler);
    router.get("/product/:id", product_handler);

    let mut chain = Chain::new(router);
    chain.link(Write::<DatabaseConnection>::both(conn));

    Iron::new(chain).http("localhost:8080").unwrap();
}
