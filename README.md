# Interstellar Delivery System

This project is a simple interstellar package delivery tracking system built with Rocket, a web framework for Rust. It allows users to request package deliveries, track packages, and filter packages by destination or speed. The system includes authentication via an `X-Interstellar-Token` header.

## Using Rocket-Ts for generating the ts interfaces

See how we use it here [rocket.ts](ROCKET.md).
And more about [rocket-ts](https://github.com/Kindness-Works/rocket-ts) here.

## Features

-   **Deliver Package**: Request a new package delivery to a specified destination with a specified speed.
-   **Track Package**: Track a package by its unique package ID.
-   **List Packages by Destination**: Retrieve a list of packages going to a specified destination.
-   **List Packages by Speed**: Retrieve a list of packages with a specified speed.
-   **Package Count**: Get the total number of packages.

## Endpoints

### `POST /deliver`

Request a new package delivery.

-   **Headers**:
    -   `X-Interstellar-Token`: Authentication token.
-   **Request Body** (JSON):
    ```json
    {
    	"destination": "Mars",
    	"speed": "fast"
    }
    ```
-   **Response** (JSON):
    ```json
    {
    	"package_id": "abc123xyz",
    	"status": "Package en route to Mars with fast"
    }
    ```

### `GET /track/<package_id>`

Track a package by its unique ID.

-   **Headers**:
    -   `X-Interstellar-Token`: Authentication token.
-   **Response** (JSON):
    ```json
    {
    	"package_id": "abc123xyz",
    	"destination": "Mars",
    	"current_location": "Earth",
    	"speed": "fast",
    	"tracking_info": "Package is currently at Earth en route to Mars"
    }
    ```

### `GET /packages/destination/<destination>`

List all packages going to a specified destination.

-   **Headers**:
    -   `X-Interstellar-Token`: Authentication token.
-   **Response** (JSON):
    ```json
    {
    	"packages": [
    		{
    			"package_id": "abc123xyz",
    			"destination": "Mars",
    			"current_location": "Earth",
    			"speed": "fast"
    		},
    		{
    			"package_id": "def456uvw",
    			"destination": "Mars",
    			"current_location": "Moon",
    			"speed": "slow"
    		}
    	]
    }
    ```

### `GET /packages/speed/<speed>`

List all packages with a specified speed.

-   **Headers**:
    -   `X-Interstellar-Token`: Authentication token.
-   **Response** (JSON):
    ```json
    {
    	"packages": [
    		{
    			"package_id": "abc123xyz",
    			"destination": "Mars",
    			"current_location": "Earth",
    			"speed": "fast"
    		},
    		{
    			"package_id": "ghi789rst",
    			"destination": "Venus",
    			"current_location": "Earth",
    			"speed": "fast"
    		}
    	]
    }
    ```

### `GET /packages/count`

Get the total count of all packages.

-   **Headers**:
    -   `X-Interstellar-Token`: Authentication token.
-   **Response** (JSON):
    ```json
    {
    	"package_count": 3
    }
    ```

## Setup

1. **Clone the repository**:

    ```sh
    git clone https://github.com/AvaterClasher/interstellar-delivery.git
    cd interstellar-delivery
    ```

2. **Install dependencies**:
   Ensure you have Rust and Cargo installed. Then run:

    ```sh
    cargo build
    ```

3. **Run the application**:

    ```sh
    cargo run
    ```

4. **Make requests**:
   Use tools like `curl`, Postman, or any HTTP client to interact with the endpoints.

## Authentication

This application uses a simple header-based authentication. All requests must include the `X-Interstellar-Token` header.

## Example `curl` Commands

-   **Deliver Package**:

    ```sh
    curl -X POST http://localhost:8000/deliver \
         -H "Content-Type: application/json" \
         -H "X-Interstellar-Token: your_token_here" \
         -d '{"destination": "Mars", "speed": "fast"}'
    ```

-   **Track Package**:

    ```sh
    curl -X GET http://localhost:8000/track/abc123xyz \
         -H "X-Interstellar-Token: your_token_here"
    ```

-   **List Packages by Destination**:

    ```sh
    curl -X GET http://localhost:8000/packages/destination/Mars \
         -H "X-Interstellar-Token: your_token_here"
    ```

-   **List Packages by Speed**:

    ```sh
    curl -X GET http://localhost:8000/packages/speed/fast \
         -H "X-Interstellar-Token: your_token_here"
    ```

-   **Get Package Count**:
    ```sh
    curl -X GET http://localhost:8000/packages/count \
         -H "X-Interstellar-Token: your_token_here"
    ```

## License

This project is licensed under the [MIT License](LICENSE.md) file for details.
