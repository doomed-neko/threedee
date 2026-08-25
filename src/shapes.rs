use crate::{
    raylib::{Color, DrawLineEx, DrawTriangle},
    vec::{Vec3, Vector2},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub verts: Vec<Vec3>,
    pub faces: Vec<Vec<Vec3>>,
}
impl Shape {
    pub const fn new(verts: Vec<Vec3>, faces: Vec<Vec<Vec3>>) -> Self {
        Self { verts, faces }
    }
    pub fn draw<T>(&self, f: T)
    where
        T: Fn(Vec3) -> Vec3,
    {
        let colors = [
            Color::YELLOW,
            Color::PINK,
            Color::MAROON,
            Color::SKYBLUE,
            Color::PURPLE,
            Color::BEIGE,
            Color::BROWN,
            Color::DARKBROWN,
        ];

        let mut i = 0;
        self.faces.iter().for_each(|face| {
            let v0 = f(face[0]);
            let v1 = f(face[1]);
            let v2 = f(face[2]);
            let l = Vec3::new(0., 0., 01.);
            let n = (v1 - v0).cross_product(v2 - v0);
            if l.dot_product(-n) >= 0. {
                let pts: Vec<Vector2> = face
                    .iter()
                    .map(|&v| f(v).project2d().screen_cords().to_vec())
                    .collect();
                pts.chunks_exact(4).for_each(|pts| {
                    draw_triangle_double_sided(
                        pts[0],
                        pts[2],
                        pts[3],
                        colors[i / 100 % colors.len()],
                    );
                    draw_triangle_double_sided(
                        pts[0],
                        pts[1],
                        pts[2],
                        colors[i / 100 % colors.len()],
                    );
                });
                // if pts.len() >= 4 {
                //     draw_triangle_double_sided(pts[0], pts[2], pts[3], colors[i % colors.len()]);
                // } else if pts.len() >= 3 {
                //     draw_triangle_double_sided(pts[0], pts[1], pts[2], colors[i % colors.len()]);
                // }
                // for vert in pts.windows(2) {
                // DrawLineEx(vert[0], vert[1], 3.0, Color::GREEN);
                // }
            }
            i += 1;
        });
    }
}

fn draw_triangle_double_sided(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    DrawTriangle(v1, v2, v3, color);
    DrawTriangle(v3, v2, v1, color);
}
pub fn cube() -> Shape {
    let verts = vec![
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.5, -0.5, 0.5),
        //
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
    ];
    let faces = [
        vec![0, 1, 2, 3, 0], // Front
        vec![5, 4, 7, 6, 5], // Back
        vec![4, 5, 1, 0, 4], // Top
        vec![2, 6, 7, 3, 2], // Bottom
        vec![4, 0, 3, 7, 4], // Right
        vec![1, 5, 6, 2, 1], // Left
    ]
    .into_iter()
    .map(|face| {
        face.into_iter()
            .map(|idx| verts[idx])
            .collect::<Vec<Vec3>>()
    })
    .collect();
    Shape { verts, faces }
}

pub fn pyramid() -> Shape {
    let verts = vec![
        Vec3::new(0.0, 0.5, 0.0),    // 0: Top Apex
        Vec3::new(0.5, -0.5, -0.5),  // 1: Back-Right
        Vec3::new(-0.5, -0.5, -0.5), // 2: Back-Left
        Vec3::new(0.5, -0.5, 0.5),   // 3: Front-Right
        Vec3::new(-0.5, -0.5, 0.5),  // 4: Front-Left
    ];

    let faces = [
        vec![3, 4, 2, 1, 3], // Square Base (Quad)
        vec![0, 3, 1, 0],    // Right Side (Triangle)
        vec![0, 1, 2, 0],    // Back Side (Triangle)
        vec![0, 2, 4, 0],    // Left Side (Triangle)
        vec![0, 4, 3, 0],    // Front Side (Triangle)
    ]
    .into_iter()
    .map(|face| {
        face.into_iter()
            .map(|idx| verts[idx])
            .collect::<Vec<Vec3>>()
    })
    .collect();

    Shape { verts, faces }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MeshBuilder {
    pub verts: Vec<Vec3>,
    pub faces: Vec<Vec<Vec3>>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            verts: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, v: Vec3) -> usize {
        self.verts.push(v);
        self.verts.len() - 1
    }

    pub fn add_face(&mut self, indices: &[usize]) {
        let mut face_verts: Vec<Vec3> = indices.iter().map(|&idx| self.verts[idx]).collect();
        if let Some(&first) = face_verts.first() {
            face_verts.push(first);
        }
        self.faces.push(face_verts);
    }

    pub fn build(self) -> Shape {
        Shape {
            verts: self.verts,
            faces: self.faces,
        }
    }
}

pub fn regular_prism(sides: usize, radius: f32, height: f32) -> Shape {
    let mut builder = MeshBuilder::new();
    let half_h = height / 2.0;

    for i in 0..sides {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (sides as f32);
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        builder.add_vertex(Vec3::new(x, half_h, z));
        builder.add_vertex(Vec3::new(x, -half_h, z));
    }

    let top_cap: Vec<usize> = (0..sides).map(|i| i * 2).collect();
    let mut bot_cap: Vec<usize> = (0..sides).map(|i| i * 2 + 1).collect();
    bot_cap.reverse();

    builder.add_face(&top_cap);
    builder.add_face(&bot_cap);

    for i in 0..sides {
        let next = (i + 1) % sides;
        let top_curr = i * 2;
        let bot_curr = i * 2 + 1;
        let top_next = next * 2;
        let bot_next = next * 2 + 1;

        builder.add_face(&[top_curr, top_next, bot_next, bot_curr]);
    }

    builder.build()
}
