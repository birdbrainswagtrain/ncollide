use crate::math::{Isometry, Point};
use na::{self, RealField};
use crate::query::Contact;
use crate::shape::{Plane, SupportMap};

/// Contact between a plane and a support-mapped shape (Cuboid, ConvexHull, etc.)
pub fn plane_against_support_map<N: RealField, G: ?Sized + SupportMap<N>>(
    mplane: &Isometry<N>,
    plane: &Plane<N>,
    mother: &Isometry<N>,
    other: &G,
    prediction: N,
) -> Option<Contact<N>>
{
    let plane_normal = mplane * plane.normal();
    let plane_center = Point::from(mplane.translation.vector);
    let deepest = other.support_point_toward(mother, &-plane_normal);

    let distance = plane_normal.dot(&(plane_center - deepest));

    if distance > -prediction {
        let c1 = deepest + *plane_normal * distance;

        Some(Contact::new(c1, deepest, plane_normal, distance))
    } else {
        None
    }
}

/// Contact between a support-mapped shape (Cuboid, ConvexHull, etc.) and a plane.
pub fn support_map_against_plane<N: RealField, G: ?Sized + SupportMap<N>>(
    mother: &Isometry<N>,
    other: &G,
    mplane: &Isometry<N>,
    plane: &Plane<N>,
    prediction: N,
) -> Option<Contact<N>>
{
    plane_against_support_map(mplane, plane, mother, other, prediction).map(|mut c| {
        c.flip();
        c
    })
}
