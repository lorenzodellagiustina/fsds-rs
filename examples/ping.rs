use fsds_rs::{client, types::CarControls};
use std::{thread::sleep, time::Duration};

/// The name of the vehicle to control.
const VEHICLE_NAME: &str = "FSCar";

#[tokio::main]
async fn main() {
    let mut client = client::FSDSClient::init(None, None)
        .await
        .expect("Cannot establish a connection with the simulator");

    // Trying connection.
    client.ping().await;

    client.enable_api_control(true, VEHICLE_NAME).await;

    let mut controls = CarControls::default();

    controls.throttle = 1.0;
    let value = client.set_car_controls(controls, VEHICLE_NAME).await;
    println!("{:?}", value);

    loop {
        sleep(Duration::from_secs(1));
    }
}
