use na::{self, RealField};

use crate::math::{Isometry, Point};
use crate::query::ClosestPoints;
use crate::shape::Plane;
use crate::shape::SupportMap;

/// Closest points between a plane and a support-mapped shape (Cuboid, ConvexHull, etc.)
pub fn plane_against_support_map<N: RealField, G: ?Sized + SupportMap<N>>(
    mplane: &Isometry<N>,
    plane: &Plane<N>,
    mother: &Isometry<N>,
    other: &G,
    margin: N,
) -> ClosestPoints<N>
{
    assert!(
        margin >= na::zero(),
        "The proximity margin must be positive or null."
    );

    let plane_normal = mplane * plane.normal();
    let plane_center = Point::from(mplane.translation.vector);
    let deepest = other.support_point(mother, &-plane_normal);

    let distance = plane_normal.dot(&(plane_center - deepest));

    if distance >= -margin {
        if distance >= na::zero() {
            ClosestPoints::Intersecting
        } else {
            let c1 = deepest + *plane_normal * distance;
            ClosestPoints::WithinMargin(c1, deepest)
        }
    } else {
        ClosestPoints::Disjoint
    }
}

/// Closest points between a support-mapped shape (Cuboid, ConvexHull, etc.) and a plane.
pub fn support_map_against_plane<N: RealField, G: ?Sized + SupportMap<N>>(
    mother: &Isometry<N>,
    other: &G,
    mplane: &Isometry<N>,
    plane: &Plane<N>,
    margin: N,
) -> ClosestPoints<N>
{
    let mut res = plane_against_support_map(mplane, plane, mother, other, margin);
    res.flip();
    res
}
