//! This module contains the definitions of the input and output types used to
//! communicate with the simulator.
//!
//! Struct are serialized to `msgpack_rpc::Value::Map` and vice versa in order
//! to be sent and received from the simulator.

use msgpack_rpc::Value;
use std::any::Any;
use struct_iterable::Iterable;

/// Utility to convert any type to a `msgpack_rpc::Value`.
fn any_to_value(value: &dyn Any) -> Value {
    if let Some(value) = value.downcast_ref::<u32>() {
        Value::from(*value)
    } else if let Some(value) = value.downcast_ref::<f32>() {
        Value::from(*value)
    } else if let Some(value) = value.downcast_ref::<bool>() {
        Value::from(*value)
    } else {
        Value::Nil
    }
}

#[derive(Iterable)]
pub struct CarControls {
    pub throttle: f32,
    pub steering: f32,
    pub brake: f32,
    pub handbrake: bool,
    pub is_manual_gear: bool,
    pub manual_gear: u32,
    pub gear_immediate: bool,
}

impl Default for CarControls {
    fn default() -> Self {
        Self {
            throttle: Default::default(),
            steering: Default::default(),
            brake: Default::default(),
            handbrake: Default::default(),
            is_manual_gear: Default::default(),
            manual_gear: Default::default(),
            gear_immediate: true,
        }
    }
}

impl From<CarControls> for Value {
    fn from(value: CarControls) -> Self {
        let vec = value
            .iter()
            .map(|(k, v)| {
                v.type_id();
                (k.into(), any_to_value(v))
            })
            .collect();

        Value::Map(vec)
    }
}
