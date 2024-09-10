//! This module contains the FSDSClient struct which is used to interact with
//! the FSDS server.
//!
//! The FSDSClient struct provides all the API methods available to interact
//! with the simulator.

use msgpack_rpc::{Client, Value};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::types::{CarControls, ImageRequest, ImageType};

pub struct FSDSClient {
    client: Client,
}

impl FSDSClient {
    pub async fn init(addr: Option<&str>, _timeout_value: Option<u64>) -> anyhow::Result<Self> {
        let addr = addr.unwrap_or("127.0.0.1:41451");

        // Create a client with the specified timeout if needed.
        let stream = TcpStream::connect(&addr).await?;

        let client = Client::new(stream.compat());

        Ok(FSDSClient { client })
    }

    /// Reset the vehicle to its original starting state.
    ///
    /// Note that you must call `enable_api_control` again after the call to
    /// reset.
    pub async fn reset(&mut self) -> Result<Value, anyhow::Error> {
        self.client
            .request("reset", &[])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// If connection is established then this call will return Ok(_) otherwise
    /// it will be blocked until timeout.
    pub async fn ping(&mut self) -> Result<Value, anyhow::Error> {
        self.client
            .request("ping", &[])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Enables API control for vehicle corresponding to vehicle_name.
    pub async fn enable_api_control(&mut self, vehicle_name: &str) -> Result<Value, anyhow::Error> {
        self.client
            .request("enableApiControl", &[true.into(), vehicle_name.into()])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Disable API control for vehicle corresponding to vehicle_name.
    pub async fn disable_api_control(
        &mut self,
        vehicle_name: &str,
    ) -> Result<Value, anyhow::Error> {
        self.client
            .request("enableApiControl", &[false.into(), vehicle_name.into()])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Returns true if API control is established.
    ///
    /// If false (which is default) then API calls would be ignored. After a
    /// successful call to `enableApiControl`, `isApiControlEnabled` should
    /// return true.
    pub async fn is_api_control_enabled(
        &mut self,
        vehicle_name: &str,
    ) -> Result<Value, anyhow::Error> {
        self.client
            .request("isApiControlEnabled", &[vehicle_name.into()])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Get a single image.
    ///
    /// Returns bytes of png format image which can be dumped into a binary file
    /// to create .png image.
    ///
    /// See https://microsoft.github.io/AirSim/image_apis/ for details.
    pub async fn sim_get_image(
        &mut self,
        camera_name: &str,
        image_type: ImageType,
        vehicle_name: &str,
    ) -> Result<Value, anyhow::Error> {
        self.client
            .request(
                "simGetImage",
                &[camera_name.into(), image_type.into(), vehicle_name.into()],
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Get multiple images.
    ///
    /// See https://microsoft.github.io/AirSim/image_apis/ for details and
    /// examples.
    pub async fn sim_get_images(
        &mut self,
        requests: &[ImageRequest],
        vehicle_name: &str,
    ) -> Result<Value, anyhow::Error> {
        self.client
            .request(
                "simGetImages",
                &[
                    Value::Array(requests.iter().map(|r| r.clone().into()).collect()),
                    vehicle_name.into(),
                ],
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Get Ground truth kinematics of the vehicle.
    pub async fn sim_get_ground_truth_kinematics(
        &mut self,
        vehicle_name: &str,
    ) -> Result<Value, anyhow::Error> {
        self.client
            .request("simGetGroundTruthKinematics", &[vehicle_name.into()])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn set_car_controls(&mut self, controls: CarControls, vehicle_name: &str) {
        self.client
            .request("setCarControls", &[controls.into(), vehicle_name.into()]);
    }

    pub async fn get_car_state(&mut self, vehicle_name: &str) -> Result<Value, anyhow::Error> {
        self.client
            .request("getCarState", &[vehicle_name.into()])
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}
