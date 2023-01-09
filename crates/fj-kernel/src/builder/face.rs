use fj_math::Point;

use crate::partial::{Partial, PartialCycle, PartialFace};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior polygon, from the provided points
    fn add_interior_polygon_from_points(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );
}

impl FaceBuilder for PartialFace {
    fn add_interior_polygon_from_points(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let mut cycle = PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        };
        cycle.update_as_polygon_from_points(points);

        self.interiors.push(Partial::from_partial(cycle));
    }
}
