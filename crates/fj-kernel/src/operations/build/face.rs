use fj_math::Point;

use crate::{
    objects::{Cycle, Face, HalfEdge, Objects, Surface},
    operations::Insert,
    services::Service,
    storage::Handle,
};

use super::{BuildHalfEdge, BuildSurface};

/// Build a [`Face`]
pub trait BuildFace {
    /// Build a triangle
    fn triangle(
        points: [impl Into<Point<3>>; 3],
        objects: &mut Service<Objects>,
    ) -> Triangle {
        let [a, b, c] = points.map(Into::into);

        let surface = Surface::plane_from_points([a, b, c]).insert(objects);
        let (exterior, edges) = {
            let half_edges = [[a, b], [b, c], [c, a]].map(|points| {
                let half_edge = HalfEdge::line_segment_from_global_points(
                    points, &surface, None, objects,
                );

                half_edge.insert(objects)
            });

            let cycle = Cycle::new(half_edges.clone()).insert(objects);

            (cycle, half_edges)
        };

        let face = Face::new(surface, exterior, [], None);

        Triangle { face, edges }
    }
}

impl BuildFace for Face {}

/// A triangle
///
/// Returned by [`BuildFace::triangle`].
pub struct Triangle {
    /// The face that forms the triangle
    pub face: Face,

    /// The edges of the triangle
    pub edges: [Handle<HalfEdge>; 3],
}
