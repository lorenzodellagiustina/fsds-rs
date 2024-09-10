use fsds_rs::{client, types::CarControls};
use std::{thread::sleep, time::Duration};

/// The name of the vehicle to control.
const VEHICLE_NAME: &str = "FSCar";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // ---------- //
    // CONNECTION //
    // ---------- //
    // Connect to the simulator.
    let mut client = client::FSDSClient::init(None, None)
        .await
        .expect("Cannot establish a connection with the simulator");
    // Check network connection, exit if not connected.
    client.ping().await?;
    // Enable control of the vehicle via the API.
    client.enable_api_control(VEHICLE_NAME).await?;

    // ---------------- //
    // CONTROL THE CAR! //
    // ---------------- //
    // Set the throttle to 1.0.
    let mut controls = CarControls::default();
    controls.throttle = 1.0;
    client.set_car_controls(controls, VEHICLE_NAME).await;

    // Loop to keep the program running.
    loop {
        sleep(Duration::from_secs(1));
    }
}
