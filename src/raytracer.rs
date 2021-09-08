use std::cmp::Ordering;

use crate::utilities::{Matrix3, Vector3};

pub struct SceneInformation{
    pub lamp: Vector3,
    pub camera_pos: Vector3,
    pub camera_matrix: Matrix3
}