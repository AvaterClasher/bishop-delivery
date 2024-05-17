use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::{Json, Value as JsonValue};
use rocket::{get, post, routes};
use serde::Deserialize;
use serde_json::json;
use rocket::launch;

#[derive(Debug, Deserialize)]
struct DeliveryRequest {
    destination: String,
    speed: String,
}

pub struct InterstellarToken;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for InterstellarToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Check if the custom header X-Interstellar-Token is present in the request
        if request.headers().contains("X-Interstellar-Token") {
            request::Outcome::Success(InterstellarToken)
        } else {
            request::Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[post("/deliver", data = "<delivery_request>")]
fn deliver(delivery_request: Json<DeliveryRequest>, _token: InterstellarToken) -> Json<JsonValue> {
    // Simulate package journey
    let package_id = generate_package_id();
    let journey_status = simulate_journey(&delivery_request.destination, &delivery_request.speed);
    let response_data = json!({
        "package_id": package_id,
        "status": journey_status,
    });
    Json(response_data)
}

#[get("/track/<package_id>")]
fn track(package_id: String, _token: InterstellarToken) -> Json<JsonValue> {
    // Retrieve package tracking information (simulate)
    let tracking_info = retrieve_tracking_info(&package_id);
    let response_data = json!({
        "package_id": package_id,
        "tracking_info": tracking_info,
        "destination": "Earth",
    });
    Json(response_data)
}

fn generate_package_id() -> String {
    // Generate a unique package ID (for simulation)
    // You can use any logic here to generate IDs
    // For simplicity, we'll just generate a random ID
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let package_id: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    package_id
}

fn simulate_journey(destination: &str, speed: &str) -> String {
    // Simulate package journey (for demonstration)
    // This function can include complex logic to simulate
    // the time it takes for delivery based on destination
    // and speed selected by the user
    // For simplicity, we'll return a fixed message
    format!("Package en route to {}", destination)
}

fn retrieve_tracking_info(package_id: &str) -> String {
    // Retrieve tracking information (for simulation)
    // This function could interact with a database or
    // external API to fetch real tracking information
    // For simplicity, we'll return a fixed message
    format!("Tracking information for package {}", package_id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![deliver, track])
}
