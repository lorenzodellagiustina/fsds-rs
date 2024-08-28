//! This module contains the FSDSClient struct which is used to interact with
//! the FSDS server.
//!
//! The FSDSClient struct provides all the API methods available to interact
//! with the simulator.

use msgpack_rpc::{Client, Value};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::types::CarControls;

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

    pub async fn reset(&mut self) {
        self.client.request("reset", &[]);
    }

    pub async fn ping(&mut self) {
        self.client.request("ping", &[]);
    }

    pub async fn enable_api_control(&mut self, is_enabled: bool, vehicle_name: &str) {
        self.client.request(
            "enableApiControl",
            &[is_enabled.into(), vehicle_name.into()],
        );
    }

    pub async fn is_api_control_enabled(&mut self, vehicle_name: &str) {
        self.client
            .request("isApiControlEnabled", &[vehicle_name.into()]);
    }

    pub async fn set_car_controls(&mut self, controls: CarControls, vehicle_name: &str) {
        self.client
            .request("setCarControls", &[controls.into(), vehicle_name.into()]);
    }

    pub async fn get_car_state(&mut self, vehicle_name: &str) -> Result<Value, Value> {
        self.client
            .request("getCarState", &[vehicle_name.into()])
            .await
    }
}
