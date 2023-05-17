use std::convert::identity;

use nalgebra as na;
struct Transform {
    m: na::Matrix4<f32>,
    m_inv: na::Matrix4<f32>,
}

impl Transform {
    pub fn new(mat: Option<na::Matrix4<f32>>) -> Self {
        if mat == None {
            return Transform {
                m: na::Matrix4::<f32>::identity(),
                m_inv: na::Matrix4::<f32>::identity(),
            };
        }
        Transform {
            m: mat.unwrap(),
            m_inv: (mat.unwrap()).try_inverse().unwrap(),
        }
    }
}
