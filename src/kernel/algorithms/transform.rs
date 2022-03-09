use crate::{
    kernel::{
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
        },
    },
    math::Transform,
};

/// Create a new shape that is a transformed version of an existing one
///
/// # Implementation note
///
/// This code isn't really correct, only transforming the faces of the original
/// shape and not taking care of anything else, but this is more a reflection of
/// the state of `Shape`, with its redundant data.
///
/// Addressing the shortcomings in this method probably doesn't make sense,
/// except as a side effect of addressing the shortcomings of `Shape`.
pub fn transform_shape(mut original: Shape, transform: &Transform) -> Shape {
    let mut transformed = Shape::new();

    for face in original.faces().all() {
        let face = match face.get().clone() {
            Face::Face { cycles, surface } => {
                let mut cycles_trans = Vec::new();

                for cycle in cycles {
                    let mut edges = Vec::new();

                    for edge in &cycle.edges {
                        let curve = transformed
                            .curves()
                            .add_curve(edge.curve.transform(transform));

                        let vertices = edge.vertices.clone().map(|vertices| {
                            vertices.map(|vertex| {
                                let point =
                                    transform.transform_point(&vertex.point);

                                transformed.vertices().add(point)
                            })
                        });

                        let edge = Edge { curve, vertices };
                        let edge = transformed.edges().add(edge);

                        edges.push(edge);
                    }

                    cycles_trans
                        .push(transformed.cycles().add(Cycle { edges }));
                }

                let surface =
                    transformed.surfaces().add(surface.transform(transform));

                Face::Face {
                    cycles: cycles_trans,
                    surface,
                }
            }
            Face::Triangles(mut triangles) => {
                for triangle in &mut triangles {
                    *triangle = transform.transform_triangle(triangle);
                }

                Face::Triangles(triangles)
            }
        };

        transformed.faces().add(face);
    }

    transformed
}
