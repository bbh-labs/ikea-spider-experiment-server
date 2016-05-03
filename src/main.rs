extern crate iron;
extern crate router;
extern crate persistent;
extern crate urlencoded;
extern crate postgres;
extern crate chrono;
extern crate rustc_serialize;
extern crate url;
extern crate getopts;

// Std
use std::env;

// Iron
use iron::prelude::*;
use iron::headers;
use iron::status;
use iron::typemap::Key;

// Router
use router::Router;

// Persistent
use persistent::Write;

// Urlencoded
use urlencoded::UrlEncodedQuery;

// Postgres
use postgres::{Connection, SslMode};

// URL
use url::percent_encoding::*;

// Chrono
use chrono::*;

// JSON
use rustc_serialize::json;

// Getopts
use getopts::Options;

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
	created_at: String,
	updated_at: String,
}

#[derive(RustcEncodable)]
struct Market {
    country: String,
    products: Vec<Product>,
}

fn categories_handler(req: &mut Request) -> IronResult<Response> {
    // Get database handle
    let mutex = req.get::<Write<DatabaseConnection>>().unwrap();
    let conn = mutex.lock().unwrap();

    let ref country = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("country") {
                Some(country) => format!("%{}%", country[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let mut categories = Vec::new();

    for row in &conn.query("SELECT category FROM product WHERE country ILIKE $1 GROUP BY category", &[country]).unwrap() {
        let category: String = row.get(0);
        categories.push(category);
    }

    if let Ok(json_output) = json::encode(&categories) {
        let mut response = Response::with((status::Ok, json_output));
        response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
        return Ok(response);
    }

    Ok(Response::with((status::Ok)))
}

fn subcategories_handler(req: &mut Request) -> IronResult<Response> {
    // Get database handle
    let mutex = req.get::<Write<DatabaseConnection>>().unwrap();
    let conn = mutex.lock().unwrap();

    let ref category = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("category") {
                Some(category) => format!("%{}%", category[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let ref country = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("country") {
                Some(country) => format!("%{}%", country[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let mut subcategories = Vec::new();

    for row in &conn.query("SELECT subcategory FROM product WHERE category ILIKE $1 AND country ILIKE $2 GROUP BY subcategory", &[category, country]).unwrap() {
        let subcategory: String = row.get(0);
        subcategories.push(subcategory);
    }

    if let Ok(json_output) = json::encode(&subcategories) {
        let mut response = Response::with((status::Ok, json_output));
        response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
        return Ok(response);
    }

    Ok(Response::with((status::NotFound)))
}

fn products_handler(req: &mut Request) -> IronResult<Response> {
    // Get database handle
    let mutex = req.get::<Write<DatabaseConnection>>().unwrap();
    let conn = mutex.lock().unwrap();

    let ref department = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("department") {
                Some(department) => format!("%{}%", department[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let ref category = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("category") {
                Some(category) => format!("%{}%", category[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let ref subcategory = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("subcategory") {
                Some(subcategory) => format!("%{}%", subcategory[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let ref country = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("country") {
                Some(country) => format!("%{}%", country[0]),
                None => "%%".to_string(),
            }
        },
        Err(_) => "%%".to_string(),
    };

    let mut products = Vec::new();

    for row in &conn.query(
            "SELECT * FROM product
             WHERE department ILIKE $1 AND category ILIKE $2 AND subcategory ILIKE $3 AND country ILIKE $4
             ORDER BY name ASC",
            &[department, category, subcategory, country]
        ).unwrap() {
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
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };

        let created_at: DateTime<UTC> = row.get(15);
        let updated_at: DateTime<UTC> = row.get(16);

        let mut product = product;
        product.created_at = created_at.to_rfc2822();
        product.updated_at = updated_at.to_rfc2822();

        products.push(product);
    }

    if let Ok(json_output) = json::encode(&products) {
        let mut response = Response::with((status::Ok, json_output));
        response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
        return Ok(response);
    }

    Ok(Response::with((status::NotFound)))
}

fn product_handler(req: &mut Request) -> IronResult<Response> {
    // Get database handle
    let mutex = req.get::<Write<DatabaseConnection>>().unwrap();
    let conn = mutex.lock().unwrap();
    
    let mut products = Vec::new();

    let ref ids = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    let ids_vec: Vec<&str> = ids.split(",").collect();
    if ids_vec.len() == 0 {
        return Ok(Response::with(status::BadRequest));
    } else if ids_vec.len() == 1 {
        for row in &conn.query("SELECT * FROM product WHERE id = $1", &[&ids]).unwrap() {
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
                created_at: "".to_string(),
                updated_at: "".to_string(),
            };

            let created_at: DateTime<UTC> = row.get(15);
            let updated_at: DateTime<UTC> = row.get(16);

            let mut product = product;
            product.created_at = created_at.to_rfc2822();
            product.updated_at = updated_at.to_rfc2822();

            products.push(product);
        }

        if let Ok(json_output) = json::encode(&products) {
            let mut response = Response::with((status::Ok, json_output));
            response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
            return Ok(response);
        }
    } else if ids_vec.len() > 1 {
        let mut index = 0;
        let mut ids_str = String::new();
        for id in &ids_vec {
            ids_str.push_str(&format!("'{}'", id));
            if index < ids_vec.len() - 1 {
                ids_str.push_str(",");
            }
            index += 1;
        }

        let mut countries = Vec::new();
        for row in &conn.query(&format!("SELECT country FROM product WHERE id IN ({}) GROUP BY country", &ids_str), &[]).unwrap() {
            let country: String = row.get(0);
            countries.push(country);
        }

        let mut markets = Vec::new();
        for country in countries {
            let mut products = Vec::<Product>::new();
            for row in &conn.query(&format!("SELECT * FROM product WHERE id IN ({}) AND country LIKE $1", &ids_str), &[&country]).unwrap() {
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
                    created_at: "".to_string(),
                    updated_at: "".to_string(),
                };

                let created_at: DateTime<UTC> = row.get(15);
                let updated_at: DateTime<UTC> = row.get(16);

                let mut product = product;
                product.created_at = created_at.to_rfc2822();
                product.updated_at = updated_at.to_rfc2822();

                products.push(product);
            }

            markets.push(Market{ country: country, products: products });
        }

        if let Ok(json_output) = json::encode(&markets) {
            let mut response = Response::with((status::Ok, json_output));
            response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
            return Ok(response);
        }
    }

    if let Ok(json_output) = json::encode(&products) {
        let mut response = Response::with((status::Ok, json_output));
        response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
        return Ok(response);
    }
    
    Ok(Response::with((status::NotFound, "")))
}

fn product_handler_with_query(req: &mut Request) -> IronResult<Response> {
    // Get database handle
    let mutex = req.get::<Write<DatabaseConnection>>().unwrap();
    let conn = mutex.lock().unwrap();

    let ids = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            match hashmap.get("id") {
                Some(ids) => {
                    if ids.len() > 0 {
                        ids
                    } else {
                        return Ok(Response::with((status::NotFound, "")));
                    }
                },
                None => return Ok(Response::with((status::NotFound, ""))),
            }
        },
        Err(_) => return Ok(Response::with((status::NotFound, ""))),
    };

    if ids.len() == 1 {
        let mut products = Vec::new();

        for row in &conn.query("SELECT * FROM product WHERE id = $1", &[&ids[0]]).unwrap() {
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
                created_at: "".to_string(),
                updated_at: "".to_string(),
            };

            let created_at: DateTime<UTC> = row.get(15);
            let updated_at: DateTime<UTC> = row.get(16);

            let mut product = product;
            product.created_at = created_at.to_rfc2822();
            product.updated_at = updated_at.to_rfc2822();

            products.push(product);
        }

        if let Ok(json_output) = json::encode(&products) {
            let mut response = Response::with((status::Ok, json_output));
            response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
            return Ok(response);
        }
    } else if ids.len() > 1 {
        let mut index = 0;
        let mut ids_str = String::new();
        for id in ids {
            ids_str.push_str(&format!("'{}'", id));
            if index < ids.len() - 1 {
                ids_str.push_str(",");
            }
            index += 1;
        }

        let mut countries = Vec::new();
        for row in &conn.query(&format!("SELECT country FROM product WHERE id in ({}) GROUP BY country", &ids_str), &[]).unwrap() {
            let country: String = row.get(0);
            countries.push(country);
        }

        let mut markets = Vec::new();
        for country in countries {
            let mut products = Vec::<Product>::new();
            for row in &conn.query(&format!("SELECT * FROM product WHERE id in ({}) AND country LIKE $1", &ids_str), &[&country]).unwrap() {
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
                    created_at: "".to_string(),
                    updated_at: "".to_string(),
                };

                let created_at: DateTime<UTC> = row.get(15);
                let updated_at: DateTime<UTC> = row.get(16);

                let mut product = product;
                product.created_at = created_at.to_rfc2822();
                product.updated_at = updated_at.to_rfc2822();

                products.push(product);
            }

            markets.push(Market{ country: country, products: products });
        }

        if let Ok(json_output) = json::encode(&markets) {
            let mut response = Response::with((status::Ok, json_output));
            response.headers.set(headers::AccessControlAllowOrigin::Value("*".to_string()));
            return Ok(response);
        }
    }
    
    Ok(Response::with((status::NotFound, "")))
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Parse program arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("",
                "dbhost",
                "set database host",
                "DBHOST");
    opts.optopt("",
                "dbport",
                "set database port",
                "DBPORT");
    opts.optopt("",
                "dbuser",
                "set database username",
                "DBUSER");
    opts.optopt("",
                "dbpass",
                "set database password",
                "DBPASS");
    opts.optopt("",
                "host",
                "set server host",
                "HOST");
    opts.optopt("",
                "port",
                "set server port",
                "PORT");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let dbhost: String = match matches.opt_str("dbhost") {
        Some(t) => t,
        None => "localhost".to_string(),
    };

    let dbport: String = match matches.opt_str("dbport") {
        Some(t) => t,
        None => "5432".to_string(),
    };

    let dbuser: String = match matches.opt_str("dbuser") {
        Some(t) => percent_encode(t.as_bytes(), QUERY_ENCODE_SET).collect::<String>(),
        None => "postgres".to_string(),
    };

    let dbpass: String = match matches.opt_str("dbpass") {
        Some(t) => format!(":{}", percent_encode(t.as_bytes(), QUERY_ENCODE_SET)),
        None => "".to_string(),
    };

    let conn = Connection::connect(format!("postgres://{}{}@{}:{}", dbuser, dbpass, dbhost, dbport).as_str(), SslMode::None).unwrap();

    let mut router = Router::new();
    router.get("/categories", categories_handler);
    router.get("/subcategories", subcategories_handler);
    router.get("/products", products_handler);
    router.get("/product", product_handler_with_query);
    router.get("/product/:id", product_handler);

    let mut chain = Chain::new(router);
    chain.link(Write::<DatabaseConnection>::both(conn));

    let host: String = match matches.opt_str("host") {
        Some(t) => t,
        None => "localhost".to_string(),
    };

    let port: String = match matches.opt_str("port") {
        Some(t) => t,
        None => "8080".to_string(),
    };

    let address = format!("{}:{}", host, port);
    println!("Serving at {}", address);

    Iron::new(chain).http(address.as_str()).unwrap();
}
