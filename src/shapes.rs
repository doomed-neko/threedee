use crate::{
    raylib::{Color, DrawLineEx},
    vec::V2,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub verts: Vec<V2>,
    pub faces: Vec<Vec<V2>>,
}
impl Shape {
    pub const fn new(verts: Vec<V2>, faces: Vec<Vec<V2>>) -> Self {
        Self { verts, faces }
    }
    pub fn draw<T>(&self, f: T)
    where
        T: Fn(V2) -> V2,
    {
        self.faces.iter().for_each(|face| {
            face.windows(2).for_each(|vert| {
                DrawLineEx(f(vert[0]).to_vec(), f(vert[1]).to_vec(), 3., Color::GREEN);
            });
        });
    }
}

pub fn cube() -> Shape {
    let verts = vec![
        V2::new(0.5, 0.5, 0.5),
        V2::new(-0.5, 0.5, 0.5),
        V2::new(-0.5, -0.5, 0.5),
        V2::new(0.5, -0.5, 0.5),
        //
        V2::new(0.5, 0.5, -0.5),
        V2::new(-0.5, 0.5, -0.5),
        V2::new(-0.5, -0.5, -0.5),
        V2::new(0.5, -0.5, -0.5),
    ];
    let faces = [
        vec![0, 1, 2, 3, 0],
        vec![4, 5, 6, 7, 4],
        vec![0, 4],
        vec![1, 5],
        vec![2, 6],
        vec![3, 7],
    ]
    .into_iter()
    .map(|face| face.into_iter().map(|idx| verts[idx]).collect::<Vec<V2>>())
    .collect();
    Shape { verts, faces }
}

pub fn pyramid() -> Shape {
    let verts = vec![
        V2::new(0., 0.5, 0.),
        V2::new(0.5, -0.5, -0.5),
        V2::new(-0.5, -0.5, -0.5),
        V2::new(0.5, -0.5, 0.5),
        V2::new(-0.5, -0.5, 0.5),
    ];
    let faces = verts
        .iter()
        .flat_map(|&i| verts.iter().map(move |&j| vec![i, j]))
        .collect();
    Shape { verts, faces }
}
