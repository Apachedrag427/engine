mod lerp;

pub use lerp::Lerp;

mod bounds;
mod cframe;
mod color;
mod quat;
mod rect;
mod tri;
mod vector;

pub use bounds::Bounds;
pub use bounds::CoordinateBounds;
pub use cframe::CFrame;
pub use color::Color;
pub use quat::Quat;
pub use quat::RotationOrder;
pub use rect::CoordinateRect;
pub use rect::Rect;
pub use tri::CoordinateTriangle2d;
pub use tri::Triangle2d;
pub use tri::Triangle3d;
pub use tri::Winding;
pub use vector::Coordinate2d;
pub use vector::Vector2;
pub use vector::Vector3;
