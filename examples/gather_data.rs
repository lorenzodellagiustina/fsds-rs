use std::{fs::File, str::FromStr};

use fsds_rs::{
    client,
    types::{KinematicsState, Vector3r},
};
use msgpack_rpc::Value;

/// The name of the vehicle to control.
const VEHICLE_NAME: &str = "FSCar";
/// The path to the CSV file containing the ground truth cones.
const CSV_PATH: &str =
    "../Formula-Student-Driverless-Simulator/maps/FormulaElectricBelgium/track_droneport.csv";

enum Class {
    Vehicle,
    ConeYellow,
    ConeBlue,
    ConeBigOrange,
}

impl FromStr for Class {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yellow" => Ok(Class::ConeYellow),
            "blue" => Ok(Class::ConeBlue),
            "big_orange" => Ok(Class::ConeBigOrange),
            _ => Err(format!("Unknown class: {}", s)),
        }
    }
}

/// An object in the 3D space.
/// Distances in meters.
struct Object {
    vector: Vector3r,
    class: Class,
}

impl Object {
    fn new(x_val: f64, y_val: f64, z_val: f64, class: Class) -> Self {
        Self {
            vector: Vector3r {
                x_val,
                y_val,
                z_val,
            },
            class,
        }
    }

    fn from_vector3r(vector: Vector3r, class: Class) -> Self {
        Self { vector, class }
    }
}

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

    // Get cones ground truth from CSV file.
    let mut cones_gt = Vec::new();
    // Open and read the CSV file.
    let csv_file = File::open(CSV_PATH)?;
    let mut csv_reader = csv::Reader::from_reader(csv_file);
    for row in csv_reader.records() {
        // Create a dictionary for each row.
        let row = row?;
        let entry = Object::new(
            row[1].parse().unwrap(),
            row[2].parse().unwrap(),
            0.0, // The cones are on the ground.
            row[0].parse().unwrap(),
        );
        // Append the dictionary to the list.
        cones_gt.push(entry);
    }

    // --------- //
    // MAIN LOOP //
    // --------- //
    let mut instant = std::time::Instant::now();
    let mut fps = 0;
    loop {
        // Get the ground truth kinematics of the vehicle.
        let kinematics_gt: KinematicsState = client
            .sim_get_ground_truth_kinematics(VEHICLE_NAME)
            .await?
            .try_into()
            .unwrap();
        let car_position = kinematics_gt.position;
        let car_orientation = kinematics_gt.orientation;

        let _relative_cones_gt = cones_gt
            .iter()
            .map(|cone| cone.vector - car_position)
            .collect::<Vec<_>>();

        // Get onboard image.
        let image = client
            .sim_get_image("cam1", fsds_rs::types::ImageType::Scene, VEHICLE_NAME)
            .await?;

        if let Value::Binary(_image) = image {
            // Save the image to a file.
            //let mut file = File::create("image.png")?;
            //file.write_all(&image)?;

            // Do something with the image and the cones.

            // Calculate the FPS.
            if instant.elapsed().as_secs() >= 2 {
                println!("FPS: {}", fps / 2);
                fps = 0;
                instant = std::time::Instant::now();
            } else {
                fps += 1;
            }
        }
    }
}
