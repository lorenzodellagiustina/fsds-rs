//! This module contains the definitions of the input and output types used to
//! communicate with the simulator.
//!
//! Struct are serialized to `msgpack_rpc::Value::Map` and vice versa in order
//! to be sent and received from the simulator.

use fsds_rs_derive::IntoValue;
use msgpack_rpc::Value;
use std::{any::Any, ops::{Add, Div, DivAssign, Mul, MulAssign, Sub}};
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

/// ---------- ///
/// IMAGE TYPE ///
/// ---------- ///
#[derive(Clone, Copy)]
pub enum ImageType {
    Scene = 0,
    DepthPlanner = 1,
    DepthPerspective = 2,
    DepthVis = 3,
    DisparityNormalized = 4,
    Segmentation = 5,
    SurfaceNormals = 6,
    Infrared = 7,
}

impl From<ImageType> for Value {
    fn from(value: ImageType) -> Self {
        Value::from(value as u32)
    }
}

/// --------- ///
/// VECTOR 3R ///
/// --------- ///
#[derive(Copy, Clone, Default, Iterable, IntoValue)]
struct Vector3r {
    x_val: f32,
    y_val: f32,
    z_val: f32,
}

impl Vector3r {
    pub fn nan_vector3r() -> Self {
        Self {
            x_val: f32::NAN,
            y_val: f32::NAN,
            z_val: f32::NAN,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x_val * other.x_val + self.y_val * other.y_val + self.z_val * other.z_val
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x_val: self.y_val * other.z_val - self.z_val * other.y_val,
            y_val: self.z_val * other.x_val - self.x_val * other.z_val,
            z_val: self.x_val * other.y_val - self.y_val * other.x_val,
        }
    }

    pub fn get_length(&self) -> f32 {
        (self.x_val.powi(2) + self.y_val.powi(2) + self.z_val.powi(2)).sqrt()
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        (*self - *other).get_length()
    }
}

impl Add for Vector3r {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x_val: self.x_val + other.x_val,
            y_val: self.y_val + other.y_val,
            z_val: self.z_val + other.z_val,
        }
    }
}

impl Sub for Vector3r {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x_val: self.x_val - other.x_val,
            y_val: self.y_val - other.y_val,
            z_val: self.z_val - other.z_val,
        }
    }
}

impl DivAssign<f32> for Vector3r {
    fn div_assign(&mut self, other: f32) {
        self.x_val /= other;
        self.y_val /= other;
        self.z_val /= other;
    }
}

impl MulAssign<f32> for Vector3r {
    fn mul_assign(&mut self, other: f32) {
        self.x_val *= other;
        self.y_val *= other;
        self.z_val *= other;
    }
}

/// ----------- ///
/// QUATERNIONR ///
/// ----------- ///

#[derive(Copy, Clone, Default, Iterable, IntoValue)]
struct Quaternionr {
    w_val: f32,
    x_val: f32,
    y_val: f32,
    z_val: f32,
}

impl Quaternionr {
    pub fn new(w_val: f32, x_val: f32, y_val: f32, z_val: f32) -> Self {
        Self {
            w_val,
            x_val,
            y_val,
            z_val,
        }
    }

    pub fn nan_quaternionr() -> Self {
        Self {
            w_val: f32::NAN,
            x_val: f32::NAN,
            y_val: f32::NAN,
            z_val: f32::NAN,
        }
    }
}

impl Add for Quaternionr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w_val: self.w_val + other.w_val,
            x_val: self.x_val + other.x_val,
            y_val: self.y_val + other.y_val,
            z_val: self.z_val + other.z_val,
        }
    }
}

impl Mul for Quaternionr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        todo!()
    }
}



impl From<Vector3r> for Quaternionr {
    fn from(value: Vector3r) -> Self {
        Self {
            w_val: 0.0,
            x_val: value.x_val,
            y_val: value.y_val,
            z_val: value.z_val,
        }
    }
}

/// ------------- ///
/// IMAGE REQUEST ///
/// ------------- ///
#[derive(Iterable, Clone, IntoValue)]
pub struct ImageRequest {
    camera_name: String,
    image_type: ImageType,
    pixels_as_float: bool,
    compress: bool,
}

impl<'a> Default for ImageRequest {
    fn default() -> Self {
        Self {
            camera_name: "0".to_string(),
            image_type: ImageType::Scene,
            pixels_as_float: false,
            compress: false,
        }
    }
}

/// ------------ ///
/// CAR CONTROLS ///
/// ------------ ///
#[derive(Iterable, IntoValue)]
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

/// ---------------- ///
/// KINEMATICS STATE ///
/// ---------------- ///

struct KinematicsState {
    position: Vector3r,
    orientation: Quaternionr,
    linear_velocity: Vector3r,
    angular_velocity: Vector3r,
    linear_acceleration: Vector3r,
    angular_acceleration: Vector3r,
}
