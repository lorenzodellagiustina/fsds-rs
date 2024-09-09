//! This module contains the definitions of the input and output types used to
//! communicate with the simulator.
//!
//! Struct are serialized to `msgpack_rpc::Value::Map` and vice versa in order
//! to be sent and received from the simulator.

use fsds_rs_derive::IntoValue;
use msgpack_rpc::Value;
use std::{
    any::Any,
    ops::{Add, Div, DivAssign, Mul, MulAssign, Sub},
};
use struct_iterable::Iterable;

/// Utility to convert any type to a `msgpack_rpc::Value`.
fn any_to_value(value: &dyn Any) -> Value {
    if let Some(value) = value.downcast_ref::<u64>() {
        Value::from(*value)
    } else if let Some(value) = value.downcast_ref::<f64>() {
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
        Value::from(value as u64)
    }
}

/// --------- ///
/// VECTOR 3R ///
/// --------- ///
#[derive(Copy, Clone, Default, Iterable, IntoValue)]
pub struct Vector3r {
    x_val: f64,
    y_val: f64,
    z_val: f64,
}

impl Vector3r {
    pub fn nan_vector3r() -> Self {
        Self {
            x_val: f64::NAN,
            y_val: f64::NAN,
            z_val: f64::NAN,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x_val * other.x_val + self.y_val * other.y_val + self.z_val * other.z_val
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x_val: self.y_val * other.z_val - self.z_val * other.y_val,
            y_val: self.z_val * other.x_val - self.x_val * other.z_val,
            z_val: self.x_val * other.y_val - self.y_val * other.x_val,
        }
    }

    pub fn get_length(&self) -> f64 {
        (self.x_val.powi(2) + self.y_val.powi(2) + self.z_val.powi(2)).sqrt()
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
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

impl DivAssign<f64> for Vector3r {
    fn div_assign(&mut self, other: f64) {
        self.x_val /= other;
        self.y_val /= other;
        self.z_val /= other;
    }
}

impl MulAssign<f64> for Vector3r {
    fn mul_assign(&mut self, other: f64) {
        self.x_val *= other;
        self.y_val *= other;
        self.z_val *= other;
    }
}

/// ----------- ///
/// QUATERNIONR ///
/// ----------- ///

#[derive(Copy, Clone, Default, Iterable, IntoValue)]
pub struct Quaternionr {
    w_val: f64,
    x_val: f64,
    y_val: f64,
    z_val: f64,
}

impl Quaternionr {
    pub fn new(w_val: f64, x_val: f64, y_val: f64, z_val: f64) -> Self {
        Self {
            w_val,
            x_val,
            y_val,
            z_val,
        }
    }

    pub fn nan_quaternionr() -> Self {
        Self {
            w_val: f64::NAN,
            x_val: f64::NAN,
            y_val: f64::NAN,
            z_val: f64::NAN,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.w_val * other.w_val
            + self.x_val * other.x_val
            + self.y_val * other.y_val
            + self.z_val * other.z_val
    }

    pub fn cross(&self, other: &Self) -> Self {
        let mut diff = *self * *other - *other * *self;
        diff /= 2.0;
        diff
    }

    pub fn outer_product(&self, other: &Self) -> Self {
        let mut double = self.inverse() * *other - other.inverse() * *self;
        double /= 2.0;
        double
    }

    pub fn rotate(&self, other: &Quaternionr) -> Result<Self, anyhow::Error> {
        if other.get_length() == 1.0 {
            return Ok(*other * *self * other.inverse());
        }

        Err(anyhow::anyhow!("Quaternion is not normalized"))
    }

    pub fn conjugate(&self) -> Self {
        Self {
            w_val: self.w_val,
            x_val: -self.x_val,
            y_val: -self.y_val,
            z_val: -self.z_val,
        }
    }

    pub fn star(&self) -> Self {
        self.conjugate()
    }

    pub fn inverse(&self) -> Self {
        let mut star = self.star();
        star /= self.dot(self);

        star
    }

    pub fn sgn(&self) -> Quaternionr {
        let mut self_deref = *self;
        self_deref /= self.get_length();

        self_deref
    }

    pub fn get_length(&self) -> f64 {
        (self.w_val.powi(2) + self.x_val.powi(2) + self.y_val.powi(2) + self.z_val.powi(2)).sqrt()
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

impl Sub for Quaternionr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            w_val: self.w_val - other.w_val,
            x_val: self.x_val - other.x_val,
            y_val: self.y_val - other.y_val,
            z_val: self.z_val - other.z_val,
        }
    }
}

impl Mul for Quaternionr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (t, x, y, z) = (self.w_val, self.x_val, self.y_val, self.z_val);
        let (a, b, c, d) = (other.w_val, other.x_val, other.y_val, other.z_val);

        Self {
            w_val: a * t - b * x - c * y - d * z,
            x_val: b * t + a * x + d * y - c * z,
            y_val: c * t + a * y + b * z - d * x,
            z_val: d * t + z * a + c * x - b * y,
        }
    }
}

impl Div for Quaternionr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

impl DivAssign<f64> for Quaternionr {
    fn div_assign(&mut self, other: f64) {
        self.w_val /= other;
        self.x_val /= other;
        self.y_val /= other;
        self.z_val /= other;
    }
}

impl From<Vector3r> for Quaternionr {
    fn from(value: Vector3r) -> Self {
        Self {
            w_val: value.x_val,
            x_val: value.y_val,
            y_val: value.z_val,
            z_val: 0.0,
        }
    }
}

/// ---- ///
/// POSE ///
/// ---- ///
#[derive(Copy, Clone, Default, Iterable, IntoValue)]
pub struct Pose {
    position: Vector3r,
    orientation: Quaternionr,
}

impl Pose {
    pub fn new(posizion_val: Option<Vector3r>, orientation_val: Option<Quaternionr>) -> Self {
        Self {
            position: posizion_val.unwrap_or(Vector3r::nan_vector3r()),
            orientation: orientation_val.unwrap_or(Quaternionr::nan_quaternionr()),
        }
    }

    pub fn nan_pose() -> Self {
        Self {
            position: Vector3r::nan_vector3r(),
            orientation: Quaternionr::nan_quaternionr(),
        }
    }
}

/// --------- ///
/// GEO POINT ///
/// --------- ///
#[derive(Copy, Clone, Default, Iterable, IntoValue)]
pub struct GeoPoint {
    latitude: f64,
    longitude: f64,
    altitude: f64,
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

/// -------------- ///
/// IMAGE RESPONSE ///
/// -------------- ///
#[derive(Iterable, IntoValue)]
pub struct ImageResponse {
    image_data: (u8, f64),
    camera_position: Vector3r,
    camera_orientation: Quaternionr,
    timestamp: u64, // TODO: SystemTime?
    message: String,
    pixels_as_float: f64,
    compress: bool,
    width: u64,
    height: u64,
    image_type: ImageType,
}

impl Default for ImageResponse {
    fn default() -> Self {
        Self {
            image_data: (0, 0.0),
            camera_position: Default::default(),
            camera_orientation: Default::default(),
            timestamp: 0,
            message: "".to_string(),
            pixels_as_float: 0.0,
            compress: true,
            width: 0,
            height: 0,
            image_type: ImageType::Scene,
        }
    }
}

/// ------------ ///
/// CAR CONTROLS ///
/// ------------ ///
#[derive(Iterable, IntoValue)]
pub struct CarControls {
    pub throttle: f64,
    pub steering: f64,
    pub brake: f64,
    pub handbrake: bool,
    pub is_manual_gear: bool,
    pub manual_gear: u64,
    pub gear_immediate: bool,
}

impl Default for CarControls {
    fn default() -> Self {
        Self {
            throttle: Default::default(),
            steering: Default::default(),
            brake: Default::default(),
            handbrake: false,
            is_manual_gear: false,
            manual_gear: Default::default(),
            gear_immediate: true,
        }
    }
}

/// ---------------- ///
/// KINEMATICS STATE ///
/// ---------------- ///
#[derive(Iterable, IntoValue, Default)]
pub struct KinematicsState {
    position: Vector3r,
    orientation: Quaternionr,
    linear_velocity: Vector3r,
    angular_velocity: Vector3r,
    linear_acceleration: Vector3r,
    angular_acceleration: Vector3r,
}

/// ----------------- ///
/// ENVIRONMENT STATE ///
/// ----------------- ///
#[derive(Iterable, IntoValue, Default)]
pub struct EnvironmentState {
    pub position: Vector3r,
    pub geo_point: GeoPoint,
    pub gravity: Vector3r,
    pub air_pressure: f64,
    pub temperature: f64,
    pub air_density: f64,
}

/// -------------- ///
/// COLLISION INFO ///
/// -------------- ///
#[derive(Iterable, IntoValue)]
pub struct CollisionInfo {
    pub has_collided: bool,
    pub normal: Vector3r,
    pub impact_point: Vector3r,
    pub position: Vector3r,
    pub penetration_depth: f64,
    pub timestamp: f64, // TODO: SystemTime?
    pub object_name: String,
    pub object_id: u64,
}

/// --------- ///
/// CAR STATE ///
/// --------- ///
#[derive(Iterable, IntoValue)]
pub struct CarState {
    pub speed: f64,
    pub kinematics_estimated: KinematicsState,
    pub timestamp: u64, // TODO: SystemTime?
}

/// ----------- ///
/// POSITION 2D ///
/// ----------- ///
#[derive(Iterable, IntoValue, Default)]
pub struct Position2D {
    pub x_val: f64,
    pub y_val: f64,
}

/// ------------- ///
/// REFEREE STATE ///
/// ------------- ///
#[derive(Iterable, IntoValue, Default)]
pub struct RefereeState {
    pub doo_counter: u64,
    pub laps: f64,
    pub initial_position: Position2D,
    pub cones: Vec<Position2D>,
}

// TODO:
// ----------------- ///
// PROJECTION MATRIX ///
// ----------------- ///
// #[derive(Iterable, IntoValue, Default)]
// pub struct ProjectionMatrix {
//     pub matrix: Vec<_>,
// }
