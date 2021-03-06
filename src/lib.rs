extern crate graphics;
extern crate smallvec;
extern crate cgmath;

use cgmath::{Vector2, InnerSpace};
use graphics::math;
use smallvec::SmallVec;

pub static PLAYER: &[[f64; 2]] = &[[0., 1.], [1., -1.], [0., -0.5], [-1., -1.]];
pub static BULLET: &[[f64; 2]] = &[[0., 1.], [1., 0.], [0., -1.], [-1., 0.]];
pub static ASTEROIDS: &[&[[f64; 2]]] = &[
    &[
        [0.04, 0.76],
        [0.92, 0.76],
        [0.84, -0.08],
        [0.44, -0.59],
        [-0.05, -0.39],
        [-0.31, -0.32],
        [-1.04, 0.03],
        [-0.44, 0.58],
    ],
    &[
        [0.07, 0.73],
        [0.59, 0.66],
        [1.14, 0.01],
        [0.68, -0.56],
        [-0.03, -1.16],
        [-0.32, -0.4],
        [-1.15, 0.13],
        [-0.76, 0.61],
    ],
    &[
        [0.11, 0.93],
        [0.36, 0.27],
        [1.31, 0.01],
        [0.67, -0.9],
        [-0.07, -0.59],
        [-0.31, -0.32],
        [-0.42, 0.05],
        [-0.26, 0.26],
    ],
    &[
        [-0.04, 1.09],
        [0.35, 0.3],
        [0.77, -0.02],
        [0.67, -0.53],
        [-0.04, -0.75],
        [-0.52, -0.6],
        [-0.58, -0.05],
        [-0.68, 0.91],
    ],
    &[
        [0.03, 1.39],
        [0.33, 0.35],
        [1.33, -0.11],
        [0.29, -0.37],
        [-0.02, -0.45],
        [-0.54, -0.74],
        [-0.32, 0.02],
        [-0.39, 0.33],
    ],
];

fn min_max<T: PartialOrd + Copy, I: IntoIterator<Item = T>>(iter: I) -> Option<(T, T)> {
    let mut minmax = None;

    for i in iter {
        if let Some((ref mut min, ref mut max)) = minmax {
            if i < *min {
                *min = i;
            }

            if i > *max {
                *max = i;
            }
        } else {
            minmax = Some((i, i))
        }
    }

    minmax
}

pub fn is_colliding(
    a: &[[f64; 2]],
    a_trans: math::Matrix2d<f64>,
    b: &[[f64; 2]],
    b_trans: math::Matrix2d<f64>,
) -> bool {
    fn any_seperated(a: &[Vector2<f64>], b: &[Vector2<f64>]) -> bool {
        for side in a.iter().zip(a[1..].iter().chain(Some(&a[0]))) {
            let side_vec = side.1 - side.0;
            let axis = Vector2 {
                x: -side_vec.y,
                y: side_vec.x,
            };

            if let (Some((a_min, a_max)), Some((b_min, b_max))) =
                (
                    min_max(a.iter().map(|point| point.dot(axis))),
                    min_max(b.iter().map(|point| point.dot(axis))),
                )
            {
                if a_min > b_max || b_min > a_max {
                    return true;
                }
            }
        }

        false
    }

    let a: SmallVec<[Vector2<f64>; 16]> = a.iter()
        .map(|arr| math::transform_pos(a_trans, *arr))
        .map(|arr| {
            Vector2 {
                x: arr[0],
                y: arr[1],
            }
        })
        .collect();
    let b: SmallVec<[Vector2<f64>; 16]> = b.iter()
        .map(|arr| math::transform_pos(b_trans, *arr))
        .map(|arr| {
            Vector2 {
                x: arr[0],
                y: arr[1],
            }
        })
        .collect();

    let seperated = any_seperated(&a, &b) || any_seperated(&b, &a);

    !seperated
}
