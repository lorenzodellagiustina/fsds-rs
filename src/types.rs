//! This module contains the definitions of the input and output types used to
//! communicate with the simulator.
//!
//! Struct are serialized to `msgpack_rpc::Value::Map` and vice versa in order
//! to be sent and received from the simulator.
//!
//! Enums are serialized to `msgpack_rpc::Value::Integer` and vice versa.

use fsds_rs_derive::FromIntoValue;
use msgpack_rpc::Value;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

// ---------- //
// IMAGE TYPE //
// ---------- //

/// `ImageType` is the enum that determines the type of images / cameras.
///
/// The enum contains all the AirSim image types, but only the following are
/// currently supported:
///
/// 0) Scene: an RGB image.
/// 2) DepthVis: a depth image.
///
/// Refer to the [FSDS API](https://fs-driverless.github.io/Formula-Student-Driverless-Simulator/v2.2.0/camera/#add-a-camera-to-the-car)
/// and the [AirSim API](https://microsoft.github.io/AirSim/image_apis/#available-imagetype) for more information.
#[derive(Clone, Copy, Debug)]
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

impl TryFrom<Value> for ImageType {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            // TODO: removeunwrap below
            Value::Integer(value) => Ok(match value.as_u64().unwrap() {
                0 => ImageType::Scene,
                1 => ImageType::DepthPlanner,
                2 => ImageType::DepthPerspective,
                3 => ImageType::DepthVis,
                4 => ImageType::DisparityNormalized,
                5 => ImageType::Segmentation,
                6 => ImageType::SurfaceNormals,
                7 => ImageType::Infrared,
                _ => return Err(anyhow::anyhow!("Invalid ImageType")),
            }),
            _ => Err(anyhow::anyhow!("Invalid ImageType")),
        }
    }
}

// --------- //
// VECTOR 3R //
// --------- //
#[derive(Copy, Clone, Default, FromIntoValue, Debug)]
/// A 3D vector with `f64` values.
pub struct Vector3r {
    /// The x value of the vector.
    pub x_val: f64,
    /// The y value of the vector.
    pub y_val: f64,
    /// The z value of the vector.
    pub z_val: f64,
}

impl Vector3r {
    /// Creates a new `Vector3r` with NaN values.
    pub fn nan_vector3r() -> Self {
        Self {
            x_val: f64::NAN,
            y_val: f64::NAN,
            z_val: f64::NAN,
        }
    }

    /// The dot product of two vectors.
    ///
    /// The dot product of two vectors is a scalar value that is the sum of the
    /// products of the corresponding components of the two vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x_val * other.x_val + self.y_val * other.y_val + self.z_val * other.z_val
    }

    /// Cross product of two vectors.
    ///
    /// The cross product of two vectors is a vector that is perpendicular to
    /// both of them. Refer to the [Wikipedia page](https://en.wikipedia.org/wiki/Cross_product)
    /// for more information.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x_val: self.y_val * other.z_val - self.z_val * other.y_val,
            y_val: self.z_val * other.x_val - self.x_val * other.z_val,
            z_val: self.x_val * other.y_val - self.y_val * other.x_val,
        }
    }

    /// Calculate the length of the vector.
    pub fn get_length(&self) -> f64 {
        (self.x_val.powi(2) + self.y_val.powi(2) + self.z_val.powi(2)).sqrt()
    }

    /// Calculate the distance between two vectors.
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

// ----------- //
// QUATERNIONR //
// ----------- //

/// A quaternion with `f64` values.
///
/// A quaternion is a four-dimensional number that can be used to represent
/// rotations in 3D space.

#[derive(Copy, Clone, Default, FromIntoValue, Debug)]
pub struct Quaternionr {
    /// The w value of the quaternion.
    pub w_val: f64,
    /// The x value of the quaternion.
    pub x_val: f64,
    /// The y value of the quaternion.
    pub y_val: f64,
    /// The z value of the quaternion.
    pub z_val: f64,
}

impl Quaternionr {
    /// Creates a new `Quaternionr` with NaN values.
    pub fn nan_quaternionr() -> Self {
        Self {
            w_val: f64::NAN,
            x_val: f64::NAN,
            y_val: f64::NAN,
            z_val: f64::NAN,
        }
    }

    /// The dot product of two quaternions.
    /// 
    /// The dot product of two quaternions is a scalar value that is the sum of
    /// the products of the corresponding components of the two quaternions.
    pub fn dot(&self, other: &Self) -> f64 {
        self.w_val * other.w_val
            + self.x_val * other.x_val
            + self.y_val * other.y_val
            + self.z_val * other.z_val
    }

    /// Cross product of two quaternions.
    /// 
    /// Refer to the [Wikipedia page](https://en.wikipedia.org/wiki/Cross_product#Quaternions)
    /// for more information.
    pub fn cross(&self, other: &Self) -> Self {
        let mut diff = *self * *other - *other * *self;
        diff /= 2.0;
        diff
    }

    /// Outer product of two quaternions.
    pub fn outer_product(&self, other: &Self) -> Self {
        let mut double = self.inverse() * *other - other.inverse() * *self;
        double /= 2.0;
        double
    }

    /// Rotate a quaternion by another quaternion.
    pub fn rotate(&self, other: &Quaternionr) -> Result<Self, anyhow::Error> {
        if other.get_length() == 1.0 {
            return Ok(*other * *self * other.inverse());
        }

        Err(anyhow::anyhow!("Quaternion is not normalized"))
    }

    /// Conjugate of a quaternion.
    pub fn conjugate(&self) -> Self {
        Self {
            w_val: self.w_val,
            x_val: -self.x_val,
            y_val: -self.y_val,
            z_val: -self.z_val,
        }
    }

    /// Star of a quaternion.
    /// 
    /// Alias for the conjugate of a quaternion.
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

    #[allow(clippy::suspicious_arithmetic_impl)]
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
#[derive(Copy, Clone, Default, FromIntoValue)]
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
#[derive(Copy, Clone, Default, FromIntoValue)]
pub struct GeoPoint {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

/// ------------- ///
/// IMAGE REQUEST ///
/// ------------- ///
#[derive(Clone, FromIntoValue, Debug)]
pub struct ImageRequest {
    pub camera_name: String,
    pub image_type: ImageType,
    pub pixels_as_float: bool,
    pub compress: bool,
}

impl Default for ImageRequest {
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
#[derive(FromIntoValue)]
pub struct ImageResponse {
    image_data_uint8: u64,
    image_data_float: f64,
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
            image_data_uint8: 0,
            image_data_float: 0.0,
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
#[derive(FromIntoValue)]
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
#[derive(FromIntoValue, Default, Debug)]
pub struct KinematicsState {
    pub position: Vector3r,
    pub orientation: Quaternionr,
    pub linear_velocity: Vector3r,
    pub angular_velocity: Vector3r,
    pub linear_acceleration: Vector3r,
    pub angular_acceleration: Vector3r,
}

/// ----------------- ///
/// ENVIRONMENT STATE ///
/// ----------------- ///
#[derive(FromIntoValue, Default)]
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
#[derive(FromIntoValue)]
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
#[derive(FromIntoValue)]
pub struct CarState {
    pub speed: f64,
    pub kinematics_estimated: KinematicsState,
    pub timestamp: u64, // TODO: SystemTime?
}

/// ----------- ///
/// POSITION 2D ///
/// ----------- ///
#[derive(FromIntoValue, Default)]
pub struct Position2D {
    pub x_val: f64,
    pub y_val: f64,
}

/// ------------- ///
/// REFEREE STATE ///
/// ------------- ///
#[derive(Default)]
pub struct RefereeState {
    pub doo_counter: u64,
    pub laps: f64,
    pub initial_position: Position2D,
    pub cones: Vec<Position2D>, // TODO: Vec<Position2D> does not implement Into<Value>
}

// TODO:
// ----------------- ///
// PROJECTION MATRIX ///
// ----------------- ///
// #[derive(FromIntoValue, Default)]
// pub struct ProjectionMatrix {
//     pub matrix: Vec<_>,
// }
