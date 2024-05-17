use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::launch;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::{Json, Value as JsonValue};
use rocket::{get, post, routes};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
struct DeliveryRequest {
    destination: String,
    speed: String,
}

#[derive(Debug)]
struct PackageInfo {
    destination: String,
    current_location: String,
    speed: String,
}

lazy_static! {
    static ref PACKAGES: Mutex<HashMap<String, PackageInfo>> = Mutex::new(HashMap::new());
}

pub struct InterstellarToken;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for InterstellarToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if request.headers().contains("X-Interstellar-Token") {
            request::Outcome::Success(InterstellarToken)
        } else {
            request::Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[post("/deliver", data = "<delivery_request>")]
fn deliver(delivery_request: Json<DeliveryRequest>, _token: InterstellarToken) -> Json<JsonValue> {
    let package_id = generate_package_id();
    let journey_status = simulate_journey(&delivery_request.destination, &delivery_request.speed);

    let package_info = PackageInfo {
        destination: delivery_request.destination.clone(),
        current_location: "Earth".to_string(),
        speed: delivery_request.speed.clone(),
    };

    PACKAGES
        .lock()
        .unwrap()
        .insert(package_id.clone(), package_info);

    let response_data = json!({
        "package_id": package_id,
        "status": journey_status,
    });
    Json(response_data)
}

#[get("/track/<package_id>")]
fn track(package_id: String, _token: InterstellarToken) -> Json<JsonValue> {
    let packages = PACKAGES.lock().unwrap();
    if let Some(package_info) = packages.get(&package_id) {
        let response_data = json!({
            "package_id": package_id,
            "destination": package_info.destination,
            "current_location": package_info.current_location,
            "speed": package_info.speed,
            "tracking_info": format!("Package is currently at {} en route to {}", package_info.current_location, package_info.destination),
        });
        Json(response_data)
    } else {
        Json(json!({"error": "Package ID not found"}))
    }
}

#[get("/packages/destination/<destination>")]
fn packages_by_destination(destination: String, _token: InterstellarToken) -> Json<JsonValue> {
    let packages = PACKAGES.lock().unwrap();
    let filtered_packages: Vec<_> = packages
        .iter()
        .filter(|(_, info)| info.destination == destination)
        .map(|(id, info)| {
            json!({
                "package_id": id,
                "destination": info.destination,
                "current_location": info.current_location,
                "speed": info.speed,
            })
        })
        .collect();

    Json(json!({ "packages": filtered_packages }))
}

// List all packages by speed
#[get("/packages/speed/<speed>")]
fn packages_by_speed(speed: String, _token: InterstellarToken) -> Json<JsonValue> {
    let packages = PACKAGES.lock().unwrap();
    let filtered_packages: Vec<_> = packages
        .iter()
        .filter(|(_, info)| info.speed == speed)
        .map(|(id, info)| {
            json!({
                "package_id": id,
                "destination": info.destination,
                "current_location": info.current_location,
                "speed": info.speed,
            })
        })
        .collect();

    Json(json!({ "packages": filtered_packages }))
}

// Get package count
#[get("/packages/count")]
fn package_count(_token: InterstellarToken) -> Json<JsonValue> {
    let packages = PACKAGES.lock().unwrap();
    let count = packages.len();
    
    Json(json!({ "package_count": count }))
}

fn generate_package_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let rng = thread_rng();
    let package_id: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    package_id
}

fn simulate_journey(destination: &str, speed: &str) -> String {
    format!("Package en route to {} with {}", destination, speed)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![deliver, track, packages_by_destination, packages_by_speed, package_count],
    )
}
