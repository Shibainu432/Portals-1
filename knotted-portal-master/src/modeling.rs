pub struct Triangle {
    pub vertices: [nalgebra::Vector3<f32>; 3],
    pub center: Option<nalgebra::Vector3<f32>>,

    pub colors: [[f32; 4]; 6],

    pub ambient_factor: f32,
    pub diffuse_factor: f32,
}

impl Triangle {
    pub fn center(&self) -> nalgebra::Vector3<f32> {
        if let Some(center) = self.center {
            center
        } else {
            let [v1, v2, v3] = self.vertices;
            (v1 + v2 + v3) / 3.0
        }
    }
}

mod trefoil {
    fn trefoil(t: f32) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(
            t.sin() + 2. * (2. * t).sin(),
            t.cos() - 2. * (2. * t).cos(),
            (3. * t).sin(),
        )
    }

    fn trefoil_derivative(t: f32) -> nalgebra::Vector3<f32> {
        nalgebra::Vector3::new(
            t.cos() + 4. * (2. * t).cos(),
            -t.sin() + 4. * (2. * t).sin(),
            3. * (3. * t).cos(),
        )
    }

    // Warning: theta = 0 is on the seam between worlds.
    pub fn trefoil_tube(t: f32, theta: f32) -> nalgebra::Vector3<f32> {
        let [dx, dy, _]: [f32; 3] = trefoil_derivative(t).into();

        let (s, c) = theta.sin_cos();
        trefoil(t)
            + 0.2
                * (nalgebra::Vector3::new(dy, -dx, 0.).normalize() * s - nalgebra::Vector3::z() * c)
    }
}

pub fn trefoil() -> impl Iterator<Item = Triangle> {
    const TAU: f32 = 2. * std::f32::consts::PI;

    let ambient_factor = 1.0;
    let diffuse_factor = 0.0;

    let f = |a: usize, b: usize| {
        let t = a as f32 * TAU / 96.;
        let u = (4 * b + 1) as f32 * TAU / 48.;
        trefoil::trefoil_tube(t, 4. * t + u)
    };

    (0..96).flat_map(move |a| {
        const R: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const G: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const B: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let colors = match a {
            28..=59 => [B, G, G, B, R, R], // Arc C
            60..=91 => [R, R, B, G, G, B], // Arc A
            _ => [G, B, R, R, B, G],       // Arc B
        };

        (0..12).flat_map(move |b| {
            let v0 = f(a, b);
            let v1 = f(a + 1, b);
            let v2 = f(a, b + 1);
            let v3 = f(a + 1, b + 1);

            let t0 = Triangle {
                vertices: [v0, v1, v2],
                center: None,
                colors,
                ambient_factor,
                diffuse_factor,
            };
            let t1 = Triangle {
                vertices: [v3, v2, v1],
                center: None,
                colors,
                ambient_factor,
                diffuse_factor,
            };

            std::iter::once(t0).chain(std::iter::once(t1))
        })
    })
}

pub fn subdivide(triangles: Vec<Triangle>, levels: usize) -> Vec<Triangle> {
    let mut current = triangles;
    for _ in 0..levels {
        let mut next = Vec::with_capacity(current.len() * 4);
        for tri in current {
            let v0 = tri.vertices[0];
            let v1 = tri.vertices[1];
            let v2 = tri.vertices[2];
            let m01 = (v0 + v1) * 0.5;
            let m12 = (v1 + v2) * 0.5;
            let m20 = (v2 + v0) * 0.5;
            
            next.push(Triangle { vertices: [v0, m01, m20], center: tri.center, colors: tri.colors, ambient_factor: tri.ambient_factor, diffuse_factor: tri.diffuse_factor });
            next.push(Triangle { vertices: [v1, m12, m01], center: tri.center, colors: tri.colors, ambient_factor: tri.ambient_factor, diffuse_factor: tri.diffuse_factor });
            next.push(Triangle { vertices: [v2, m20, m12], center: tri.center, colors: tri.colors, ambient_factor: tri.ambient_factor, diffuse_factor: tri.diffuse_factor });
            next.push(Triangle { vertices: [m01, m12, m20], center: tri.center, colors: tri.colors, ambient_factor: tri.ambient_factor, diffuse_factor: tri.diffuse_factor });
        }
        current = next;
    }
    current
}

pub fn skybox() -> impl IntoIterator<Item = Triangle> {
    let colors = [
        [0.05, 0.05, 0.1, 1.0], // Cosmic Void
        [0.2, 0.4, 0.2, 1.0], // Ethereal Forest
        [0.5, 0.1, 0.1, 1.0], // Volcanic Ash
        [0.0, 0.2, 0.4, 1.0], // Deep Ocean
        [0.8, 0.7, 0.4, 1.0], // Desert
        [0.9, 0.95, 1.0, 1.0], // Ice
    ];

    let ambient_factor = 1.0;
    let diffuse_factor = 0.0;

    let v0 = nalgebra::Vector3::new(-100., -100., 100.);
    let v1 = nalgebra::Vector3::new(-100., 100., -100.);
    let v2 = nalgebra::Vector3::new(100., -100., -100.);
    let v3 = nalgebra::Vector3::new(100., 100., 100.);
    let tris = vec![
        Triangle {
            vertices: [v2, v1, v0],
            center: None,
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v0, v1, v3],
            center: None,
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v3, v2, v0],
            center: None,
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v1, v2, v3],
            center: None,
            colors,
            ambient_factor,
            diffuse_factor,
        },
    ];
    subdivide(tris, 5)
}

pub fn ground() -> impl IntoIterator<Item = Triangle> {
    let colors = [
        [0.1, 0.1, 0.1, 1.0], // Cosmic Void
        [0.1, 0.5, 0.2, 1.0], // Ethereal Forest
        [0.2, 0.05, 0.05, 1.0], // Volcanic Ash
        [0.0, 0.1, 0.3, 1.0], // Deep Ocean
        [0.6, 0.5, 0.2, 1.0], // Desert
        [0.8, 0.9, 1.0, 1.0], // Ice
    ];

    let ambient_factor = 0.2;
    let diffuse_factor = 0.8;

    let v0 = nalgebra::Vector3::new(-100., -100., -2.);
    let v1 = nalgebra::Vector3::new(100., -100., -2.);
    let v2 = nalgebra::Vector3::new(100., 100., -2.);
    let v3 = nalgebra::Vector3::new(-100., 100., -2.);
    let tris = vec![
        Triangle {
            vertices: [v0, v1, v2],
            center: None,
            colors,
            ambient_factor,
            diffuse_factor,
        },
        Triangle {
            vertices: [v2, v3, v0],
            center: None,
            colors,
            ambient_factor,
            diffuse_factor,
        },
    ];
    subdivide(tris, 5)
}

pub fn ball(
    center: nalgebra::Vector3<f32>,
    world: i32,
    color: [f32; 4],
) -> impl Iterator<Item = Triangle> {
    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    const PHI: f32 = 1.618_034;

    let ur = center + 0.1 * nalgebra::Vector3::new(1.0, 0.0, PHI);
    let dr = center + 0.1 * nalgebra::Vector3::new(1.0, 0.0, -PHI);
    let ul = center + 0.1 * nalgebra::Vector3::new(-1.0, 0.0, PHI);
    let dl = center + 0.1 * nalgebra::Vector3::new(-1.0, 0.0, -PHI);
    let rf = center + 0.1 * nalgebra::Vector3::new(PHI, 1.0, 0.0);
    let lf = center + 0.1 * nalgebra::Vector3::new(-PHI, 1.0, 0.0);
    let rb = center + 0.1 * nalgebra::Vector3::new(PHI, -1.0, 0.0);
    let lb = center + 0.1 * nalgebra::Vector3::new(-PHI, -1.0, 0.0);
    let fu = center + 0.1 * nalgebra::Vector3::new(0.0, PHI, 1.0);
    let bu = center + 0.1 * nalgebra::Vector3::new(0.0, -PHI, 1.0);
    let fd = center + 0.1 * nalgebra::Vector3::new(0.0, PHI, -1.0);
    let bd = center + 0.1 * nalgebra::Vector3::new(0.0, -PHI, -1.0);

    vec![
        [ul, ur, fu],
        [ur, ul, bu],
        [dl, dr, bd],
        [dr, dl, fd],
        [rb, rf, ur],
        [rf, rb, dr],
        [lb, lf, dl],
        [lf, lb, ul],
        [fd, fu, rf],
        [fu, fd, lf],
        [bd, bu, lb],
        [bu, bd, rb],
        [fu, lf, ul],
        [fu, ur, rf],
        [fd, dl, lf],
        [fd, rf, dr],
        [bu, ul, lb],
        [bu, rb, ur],
        [bd, lb, dl],
        [bd, dr, rb],
    ]
    .into_iter()
    .map(move |vertices| Triangle {
        vertices,
        center: Some(center),
        colors,
        ambient_factor: 0.2,
        diffuse_factor: 0.8,
    })
}

pub fn pyramid(
    center: nalgebra::Vector3<f32>,
    world: i32,
    color: [f32; 4],
    size: f32,
) -> impl Iterator<Item = Triangle> {
    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    let v0 = center + size * nalgebra::Vector3::new(-1.0, -1.0, 0.0);
    let v1 = center + size * nalgebra::Vector3::new(1.0, -1.0, 0.0);
    let v2 = center + size * nalgebra::Vector3::new(1.0, 1.0, 0.0);
    let v3 = center + size * nalgebra::Vector3::new(-1.0, 1.0, 0.0);
    let top = center + size * nalgebra::Vector3::new(0.0, 0.0, 1.5);

    vec![
        [v0, v1, top],
        [v1, v2, top],
        [v2, v3, top],
        [v3, v0, top],
        [v2, v1, v0],
        [v0, v3, v2],
    ]
    .into_iter()
    .map(move |vertices| Triangle {
        vertices,
        center: Some(center),
        colors,
        ambient_factor: 0.2,
        diffuse_factor: 0.8,
    })
}

pub fn mountain(
    center: nalgebra::Vector3<f32>,
    world: i32,
    color: [f32; 4],
    size: f32,
) -> impl Iterator<Item = Triangle> {
    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    let v0 = center + size * nalgebra::Vector3::new(-1.0, -1.0, 0.0);
    let v1 = center + size * nalgebra::Vector3::new(1.0, -1.0, 0.0);
    let v2 = center + size * nalgebra::Vector3::new(1.0, 1.0, 0.0);
    let v3 = center + size * nalgebra::Vector3::new(-1.0, 1.0, 0.0);
    
    let m01 = center + size * nalgebra::Vector3::new(0.0, -1.0, 0.3);
    let m12 = center + size * nalgebra::Vector3::new(1.0, 0.0, 0.3);
    let m23 = center + size * nalgebra::Vector3::new(0.0, 1.0, 0.3);
    let m30 = center + size * nalgebra::Vector3::new(-1.0, 0.0, 0.3);

    let top = center + size * nalgebra::Vector3::new(0.0, 0.0, 1.5);

    vec![
        [v0, m01, top], [m01, v1, top],
        [v1, m12, top], [m12, v2, top],
        [v2, m23, top], [m23, v3, top],
        [v3, m30, top], [m30, v0, top],
        [v2, v1, v0], [v0, v3, v2],
    ]
    .into_iter()
    .map(move |vertices| Triangle {
        vertices,
        center: Some(center),
        colors,
        ambient_factor: 0.2,
        diffuse_factor: 0.8,
    })
}

pub fn scaled_ball(
    center: nalgebra::Vector3<f32>,
    world: i32,
    color: [f32; 4],
    size: f32,
) -> impl Iterator<Item = Triangle> {
    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    const PHI: f32 = 1.618_034;

    let ur = center + size * nalgebra::Vector3::new(1.0, 0.0, PHI);
    let dr = center + size * nalgebra::Vector3::new(1.0, 0.0, -PHI);
    let ul = center + size * nalgebra::Vector3::new(-1.0, 0.0, PHI);
    let dl = center + size * nalgebra::Vector3::new(-1.0, 0.0, -PHI);
    let rf = center + size * nalgebra::Vector3::new(PHI, 1.0, 0.0);
    let lf = center + size * nalgebra::Vector3::new(-PHI, 1.0, 0.0);
    let rb = center + size * nalgebra::Vector3::new(PHI, -1.0, 0.0);
    let lb = center + size * nalgebra::Vector3::new(-PHI, -1.0, 0.0);
    let fu = center + size * nalgebra::Vector3::new(0.0, PHI, 1.0);
    let bu = center + size * nalgebra::Vector3::new(0.0, -PHI, 1.0);
    let fd = center + size * nalgebra::Vector3::new(0.0, PHI, -1.0);
    let bd = center + size * nalgebra::Vector3::new(0.0, -PHI, -1.0);

    vec![
        [ul, ur, fu], [ur, ul, bu], [dl, dr, bd], [dr, dl, fd],
        [rb, rf, ur], [rf, rb, dr], [lb, lf, dl], [lf, lb, ul],
        [fd, fu, rf], [fu, fd, lf], [bd, bu, lb], [bu, bd, rb],
        [fu, lf, ul], [fu, ur, rf], [fd, dl, lf], [fd, rf, dr],
        [bu, ul, lb], [bu, rb, ur], [bd, lb, dl], [bd, dr, rb],
    ]
    .into_iter()
    .map(move |vertices| Triangle {
        vertices,
        center: Some(center),
        colors,
        ambient_factor: 0.2,
        diffuse_factor: 0.8,
    })
}

pub fn ringed_planet(center: nalgebra::Vector3<f32>, world: i32, color: [f32; 4]) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    triangles.extend(scaled_ball(center, world, color, 0.5));
    for i in 0..16 {
        let a1 = i as f32 * std::f32::consts::PI / 8.0;
        let a2 = (i + 1) as f32 * std::f32::consts::PI / 8.0;
        let r1 = 0.7;
        let r2 = 1.0;
        let v0 = center + nalgebra::Vector3::new(a1.cos() * r1, a1.sin() * r1, 0.0);
        let v1 = center + nalgebra::Vector3::new(a2.cos() * r1, a2.sin() * r1, 0.0);
        let v2 = center + nalgebra::Vector3::new(a1.cos() * r2, a1.sin() * r2, 0.0);
        let v3 = center + nalgebra::Vector3::new(a2.cos() * r2, a2.sin() * r2, 0.0);
        let mut colors = [[0.0; 4]; 6];
        colors[world as usize] = [0.8, 0.8, 0.9, 0.8];
        triangles.push(Triangle { vertices: [v0, v1, v2], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
        triangles.push(Triangle { vertices: [v2, v1, v3], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
        triangles.push(Triangle { vertices: [v0, v2, v1], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
        triangles.push(Triangle { vertices: [v2, v3, v1], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    }
    triangles.into_iter()
}

pub fn mushroom(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    triangles.extend(box_shape(center + nalgebra::Vector3::new(0.0, 0.0, 0.2), world, [0.8, 0.8, 0.7, 1.0], nalgebra::Vector3::new(0.1, 0.1, 0.2)));
    triangles.extend(pyramid(center + nalgebra::Vector3::new(0.0, 0.0, 0.4), world, [0.8, 0.2, 0.2, 1.0], 0.4));
    triangles.into_iter()
}

pub fn volcano(center: nalgebra::Vector3<f32>, world: i32, size: f32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let color = [0.15, 0.1, 0.1, 1.0];
    let lava_color = [1.0, 0.3, 0.0, 1.0];
    
    let v0 = center + size * nalgebra::Vector3::new(-1.0, -1.0, 0.0);
    let v1 = center + size * nalgebra::Vector3::new(1.0, -1.0, 0.0);
    let v2 = center + size * nalgebra::Vector3::new(1.0, 1.0, 0.0);
    let v3 = center + size * nalgebra::Vector3::new(-1.0, 1.0, 0.0);
    
    let top_size = size * 0.3;
    let t0 = center + nalgebra::Vector3::new(-top_size, -top_size, size * 1.2);
    let t1 = center + nalgebra::Vector3::new(top_size, -top_size, size * 1.2);
    let t2 = center + nalgebra::Vector3::new(top_size, top_size, size * 1.2);
    let t3 = center + nalgebra::Vector3::new(-top_size, top_size, size * 1.2);

    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;
    let mut lava_colors = [[0.0; 4]; 6];
    lava_colors[world as usize] = lava_color;

    let sides = vec![
        [v0, v1, t1], [v0, t1, t0],
        [v1, v2, t2], [v1, t2, t1],
        [v2, v3, t3], [v2, t3, t2],
        [v3, v0, t0], [v3, t0, t3],
    ];
    for vertices in sides {
        triangles.push(Triangle { vertices, center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    }
    let pool = vec![[t0, t1, t2], [t0, t2, t3]];
    for vertices in pool {
        triangles.push(Triangle { vertices, center: Some(center), colors: lava_colors, ambient_factor: 0.8, diffuse_factor: 0.2 });
    }
    triangles.extend(scaled_ball(center + nalgebra::Vector3::new(0.0, 0.0, size * 1.4), world, lava_color, 0.2));
    triangles.extend(scaled_ball(center + nalgebra::Vector3::new(0.1, 0.1, size * 1.7), world, [1.0, 0.6, 0.0, 1.0], 0.15));
    triangles.into_iter()
}

pub fn ruin(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let color = [0.8, 0.7, 0.6, 1.0];
    triangles.extend(box_shape(center + nalgebra::Vector3::new(-1.0, 0.0, 1.0), world, color, nalgebra::Vector3::new(0.2, 0.2, 1.0)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(1.0, 0.0, 0.8), world, color, nalgebra::Vector3::new(0.2, 0.2, 0.8)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(0.0, 0.0, 2.1), world, color, nalgebra::Vector3::new(1.4, 0.2, 0.2)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(0.5, 0.5, 0.2), world, color, nalgebra::Vector3::new(0.3, 0.3, 0.2)));
    triangles.into_iter()
}

pub fn snowman(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let white = [0.9, 0.9, 0.9, 1.0];
    let orange = [1.0, 0.5, 0.0, 1.0];
    triangles.extend(scaled_ball(center + nalgebra::Vector3::new(0.0, 0.0, 0.6), world, white, 0.4));
    triangles.extend(scaled_ball(center + nalgebra::Vector3::new(0.0, 0.0, 1.2), world, white, 0.3));
    triangles.extend(scaled_ball(center + nalgebra::Vector3::new(0.0, 0.0, 1.7), world, white, 0.2));
    triangles.extend(pyramid(center + nalgebra::Vector3::new(0.2, 0.0, 1.7), world, orange, 0.1));
    triangles.into_iter()
}

pub fn grass(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let color = [0.2, 0.8, 0.2, 1.0];
    for i in 0..5 {
        let angle = i as f32 * std::f32::consts::PI * 2.0 / 5.0;
        let dx = angle.cos() * 0.2;
        let dy = angle.sin() * 0.2;
        let tip = center + nalgebra::Vector3::new(dx * 2.0, dy * 2.0, 0.5);
        let base1 = center + nalgebra::Vector3::new(dx + dy * 0.1, dy - dx * 0.1, 0.0);
        let base2 = center + nalgebra::Vector3::new(dx - dy * 0.1, dy + dx * 0.1, 0.0);
        let mut colors = [[0.0; 4]; 6];
        colors[world as usize] = color;
        triangles.push(Triangle { vertices: [center, base1, tip], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
        triangles.push(Triangle { vertices: [center, tip, base2], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    }
    triangles.into_iter()
}

pub fn fish(center: nalgebra::Vector3<f32>, world: i32, color: [f32; 4], angle: f32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let (s, c) = angle.sin_cos();
    let forward = nalgebra::Vector3::new(c, s, 0.0);
    let right = nalgebra::Vector3::new(-s, c, 0.0);
    let up = nalgebra::Vector3::new(0.0, 0.0, 1.0);
    
    let nose = center + forward * 0.4;
    let tail_base = center - forward * 0.4;
    let top = center + up * 0.2;
    let bottom = center - up * 0.1;
    let left = center - right * 0.1;
    let right_pt = center + right * 0.1;
    
    let tail_top = tail_base - forward * 0.2 + up * 0.2;
    let tail_bottom = tail_base - forward * 0.2 - up * 0.2;

    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    // Body front
    triangles.push(Triangle { vertices: [nose, right_pt, top], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [nose, top, left], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [nose, bottom, right_pt], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [nose, left, bottom], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    
    // Body back
    triangles.push(Triangle { vertices: [tail_base, top, right_pt], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [tail_base, left, top], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [tail_base, right_pt, bottom], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [tail_base, bottom, left], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    
    // Tail
    triangles.push(Triangle { vertices: [tail_base, tail_bottom, tail_top], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });

    triangles.into_iter()
}

pub fn coral(center: nalgebra::Vector3<f32>, world: i32, color: [f32; 4]) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    triangles.extend(box_shape(center + nalgebra::Vector3::new(0.0, 0.0, 0.5), world, color, nalgebra::Vector3::new(0.1, 0.1, 0.5)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(0.2, 0.0, 0.8), world, color, nalgebra::Vector3::new(0.3, 0.1, 0.1)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(0.4, 0.0, 1.0), world, color, nalgebra::Vector3::new(0.1, 0.1, 0.3)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(-0.2, 0.1, 0.6), world, color, nalgebra::Vector3::new(0.3, 0.1, 0.1)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(-0.4, 0.1, 0.9), world, color, nalgebra::Vector3::new(0.1, 0.1, 0.4)));
    triangles.into_iter()
}

pub fn tent(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let color = [0.9, 0.8, 0.6, 1.0];
    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    let v0 = center + nalgebra::Vector3::new(-1.0, -1.0, 0.0);
    let v1 = center + nalgebra::Vector3::new(1.0, -1.0, 0.0);
    let v2 = center + nalgebra::Vector3::new(1.0, 1.0, 0.0);
    let v3 = center + nalgebra::Vector3::new(-1.0, 1.0, 0.0);
    let top1 = center + nalgebra::Vector3::new(-0.5, 0.0, 1.0);
    let top2 = center + nalgebra::Vector3::new(0.5, 0.0, 1.0);

    triangles.push(Triangle { vertices: [v0, v1, top2], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [v0, top2, top1], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [v3, top1, top2], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [v3, top2, v2], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [v0, top1, v3], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });
    triangles.push(Triangle { vertices: [v1, v2, top2], center: Some(center), colors, ambient_factor: 0.2, diffuse_factor: 0.8 });

    triangles.into_iter()
}

pub fn igloo(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let color = [0.9, 0.95, 1.0, 1.0];
    triangles.extend(scaled_ball(center, world, color, 1.5));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(1.2, 0.0, 0.0), world, color, nalgebra::Vector3::new(0.5, 0.6, 0.6)));
    triangles.into_iter()
}

pub fn pine_tree(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    // Trunk
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(0.0, 0.0, 0.5),
        world,
        [0.3, 0.2, 0.1, 1.0],
        nalgebra::Vector3::new(0.15, 0.15, 0.5),
    ));
    // Leaves (snowy)
    triangles.extend(pyramid(
        center + nalgebra::Vector3::new(0.0, 0.0, 1.0),
        world,
        [0.8, 0.9, 0.9, 1.0],
        1.2,
    ));
    triangles.extend(pyramid(
        center + nalgebra::Vector3::new(0.0, 0.0, 1.8),
        world,
        [0.85, 0.95, 0.95, 1.0],
        0.9,
    ));
    triangles.extend(pyramid(
        center + nalgebra::Vector3::new(0.0, 0.0, 2.5),
        world,
        [0.9, 1.0, 1.0, 1.0],
        0.6,
    ));
    triangles.into_iter()
}

pub fn satellite(center: nalgebra::Vector3<f32>, world: i32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let silver = [0.8, 0.8, 0.8, 1.0];
    let blue = [0.2, 0.4, 0.8, 1.0];
    
    // Body
    triangles.extend(box_shape(center, world, silver, nalgebra::Vector3::new(0.3, 0.3, 0.5)));
    // Solar panels
    triangles.extend(box_shape(center + nalgebra::Vector3::new(1.0, 0.0, 0.0), world, blue, nalgebra::Vector3::new(0.8, 0.5, 0.05)));
    triangles.extend(box_shape(center + nalgebra::Vector3::new(-1.0, 0.0, 0.0), world, blue, nalgebra::Vector3::new(0.8, 0.5, 0.05)));
    // Antenna
    triangles.extend(pyramid(center + nalgebra::Vector3::new(0.0, 0.0, 0.5), world, silver, 0.2));
    
    triangles.into_iter()
}

pub fn crystal(center: nalgebra::Vector3<f32>, world: i32, color: [f32; 4], size: f32) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    triangles.extend(pyramid(center, world, color, size));
    let mut bottom = pyramid(center, world, color, size).collect::<Vec<_>>();
    for t in &mut bottom {
        for v in &mut t.vertices {
            v.z = center.z - (v.z - center.z);
        }
        t.vertices.swap(1, 2);
    }
    triangles.extend(bottom);
    triangles.into_iter()
}

pub fn tree(
    center: nalgebra::Vector3<f32>,
    world: i32,
) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    // Trunk
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(0.0, 0.0, 0.5),
        world,
        [0.4, 0.2, 0.1, 1.0],
        nalgebra::Vector3::new(0.2, 0.2, 0.5),
    ));
    // Leaves
    triangles.extend(pyramid(
        center + nalgebra::Vector3::new(0.0, 0.0, 1.0),
        world,
        [0.1, 0.8, 0.2, 1.0],
        1.5,
    ));
    triangles.extend(pyramid(
        center + nalgebra::Vector3::new(0.0, 0.0, 2.0),
        world,
        [0.1, 0.9, 0.2, 1.0],
        1.2,
    ));
    triangles.extend(pyramid(
        center + nalgebra::Vector3::new(0.0, 0.0, 3.0),
        world,
        [0.2, 1.0, 0.3, 1.0],
        0.9,
    ));
    // Apples
    triangles.extend(ball(center + nalgebra::Vector3::new(0.5, 0.5, 1.0), world, [1.0, 0.1, 0.1, 1.0]));
    triangles.extend(ball(center + nalgebra::Vector3::new(-0.5, 0.2, 1.5), world, [1.0, 0.1, 0.1, 1.0]));
    triangles.extend(ball(center + nalgebra::Vector3::new(0.2, -0.6, 2.0), world, [1.0, 0.1, 0.1, 1.0]));
    triangles.into_iter()
}

pub fn cactus(
    center: nalgebra::Vector3<f32>,
    world: i32,
) -> impl Iterator<Item = Triangle> {
    let mut triangles = Vec::new();
    let color = [0.2, 0.7, 0.2, 1.0];
    // Main body
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(0.0, 0.0, 1.0),
        world,
        color,
        nalgebra::Vector3::new(0.2, 0.2, 1.0),
    ));
    // Left arm horizontal
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(-0.4, 0.0, 0.8),
        world,
        color,
        nalgebra::Vector3::new(0.2, 0.15, 0.15),
    ));
    // Left arm vertical
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(-0.6, 0.0, 1.1),
        world,
        color,
        nalgebra::Vector3::new(0.15, 0.15, 0.4),
    ));
    // Right arm horizontal
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(0.4, 0.0, 1.2),
        world,
        color,
        nalgebra::Vector3::new(0.2, 0.15, 0.15),
    ));
    // Right arm vertical
    triangles.extend(box_shape(
        center + nalgebra::Vector3::new(0.6, 0.0, 1.6),
        world,
        color,
        nalgebra::Vector3::new(0.15, 0.15, 0.5),
    ));
    // Flowers
    triangles.extend(ball(center + nalgebra::Vector3::new(-0.6, 0.0, 1.6), world, [1.0, 0.2, 0.5, 1.0]));
    triangles.extend(ball(center + nalgebra::Vector3::new(0.6, 0.0, 2.2), world, [1.0, 0.2, 0.5, 1.0]));
    triangles.extend(ball(center + nalgebra::Vector3::new(0.0, 0.0, 2.1), world, [1.0, 0.2, 0.5, 1.0]));
    triangles.into_iter()
}

pub fn environment() -> impl IntoIterator<Item = Triangle> {
    let mut triangles = Vec::new();

    // World 0: Cosmic Void (floating stars and asteroids)
    for i in 0..40 {
        let x = (i as f32 * 13.0 % 60.0) - 30.0;
        let y = (i as f32 * 17.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        let z = (i as f32 * 23.0 % 30.0) + 5.0;
        let color = if i % 3 == 0 {
            [0.8, 0.8, 1.0, 1.0] // Blueish star
        } else if i % 3 == 1 {
            [1.0, 0.9, 0.8, 1.0] // Yellowish star
        } else {
            [1.0, 1.0, 1.0, 1.0] // White star
        };
        if i % 5 == 0 {
            triangles.extend(ringed_planet(nalgebra::Vector3::new(x, y, z), 0, color));
        } else {
            triangles.extend(scaled_ball(nalgebra::Vector3::new(x, y, z), 0, color, 0.2));
        }
    }
    for i in 0..15 {
        let x = (i as f32 * 19.0 % 60.0) - 30.0;
        let y = (i as f32 * 29.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        let z = (i as f32 * 31.0 % 20.0) + 10.0;
        triangles.extend(satellite(nalgebra::Vector3::new(x, y, z), 0));
    }

    // World 1: Ethereal Forest (trees, bushes, rocks)
    for i in 0..25 {
        let x = (i as f32 * 11.0 % 50.0) - 25.0;
        let y = (i as f32 * 19.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(tree(nalgebra::Vector3::new(x, y, -2.0), 1));
    }
    for i in 0..20 {
        let x = (i as f32 * 17.0 % 50.0) - 25.0;
        let y = (i as f32 * 23.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(mushroom(nalgebra::Vector3::new(x, y, -2.0), 1));
    }
    for i in 0..30 {
        let x = (i as f32 * 23.0 % 50.0) - 25.0;
        let y = (i as f32 * 29.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(grass(nalgebra::Vector3::new(x, y, -2.0), 1));
    }
    for i in 0..15 {
        let x = (i as f32 * 31.0 % 50.0) - 25.0;
        let y = (i as f32 * 37.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        // Rock
        triangles.extend(box_shape(
            nalgebra::Vector3::new(x, y, -1.8),
            1,
            [0.5, 0.5, 0.5, 1.0],
            nalgebra::Vector3::new(0.4, 0.5, 0.3),
        ));
    }

    // World 2: Volcanic Ash (lava spikes, floating embers, obsidian rocks)
    for i in 0..15 {
        let x = (i as f32 * 7.0 % 50.0) - 25.0;
        let y = (i as f32 * 29.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(volcano(nalgebra::Vector3::new(x, y, -2.0), 2, 2.0 + (i as f32 % 3.0)));
    }
    for i in 0..15 {
        let x = (i as f32 * 11.0 % 50.0) - 25.0;
        let y = (i as f32 * 37.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(mountain(nalgebra::Vector3::new(x, y, -2.0), 2, [0.9, 0.2, 0.0, 1.0], 1.5 + (i as f32 % 2.0)));
    }
    for i in 0..40 {
        let x = (i as f32 * 13.0 % 50.0) - 25.0;
        let y = (i as f32 * 17.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        let z = (i as f32 * 19.0 % 15.0) - 1.0;
        triangles.extend(ball(nalgebra::Vector3::new(x, y, z), 2, [1.0, 0.5, 0.0, 0.8]));
    }
    for i in 0..20 {
        let x = (i as f32 * 23.0 % 50.0) - 25.0;
        let y = (i as f32 * 31.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(box_shape(
            nalgebra::Vector3::new(x, y, -1.5),
            2,
            [0.1, 0.1, 0.1, 1.0],
            nalgebra::Vector3::new(0.6, 0.5, 0.7),
        ));
    }

    // World 3: Deep Ocean (bubbles, seaweed, rocks)
    for i in 0..50 {
        let x = (i as f32 * 31.0 % 60.0) - 30.0;
        let y = (i as f32 * 37.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        let z = (i as f32 * 41.0 % 20.0) - 1.0;
        triangles.extend(scaled_ball(nalgebra::Vector3::new(x, y, z), 3, [0.5, 0.8, 1.0, 0.6], 0.15));
    }
    for i in 0..20 {
        let x = (i as f32 * 13.0 % 60.0) - 30.0;
        let y = (i as f32 * 19.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(crystal(nalgebra::Vector3::new(x, y, -1.0), 3, [0.8, 0.2, 0.6, 1.0], 1.0 + (i as f32 % 2.0)));
    }
    for i in 0..30 {
        let x = (i as f32 * 43.0 % 60.0) - 30.0;
        let y = (i as f32 * 47.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(coral(nalgebra::Vector3::new(x, y, -2.0), 3, [0.8, 0.4, 0.4, 1.0]));
    }
    for i in 0..20 {
        let x = (i as f32 * 53.0 % 60.0) - 30.0;
        let y = (i as f32 * 59.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        let z = (i as f32 * 61.0 % 10.0) - 1.0;
        let angle = i as f32 * 0.5;
        triangles.extend(fish(nalgebra::Vector3::new(x, y, z), 3, [0.2, 0.6, 0.9, 1.0], angle));
    }

    // World 4: Desert (pyramids, cacti, ruins)
    for i in 0..10 {
        let x = (i as f32 * 43.0 % 60.0) - 30.0;
        let y = (i as f32 * 47.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(mountain(
            nalgebra::Vector3::new(x, y, -2.0),
            4,
            [0.9, 0.8, 0.3, 1.0],
            4.0 + (i as f32 % 3.0),
        ));
    }
    for i in 0..15 {
        let x = (i as f32 * 17.0 % 60.0) - 30.0;
        let y = (i as f32 * 23.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(ruin(nalgebra::Vector3::new(x, y, -2.0), 4));
    }
    for i in 0..25 {
        let x = (i as f32 * 61.0 % 60.0) - 30.0;
        let y = (i as f32 * 67.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(cactus(nalgebra::Vector3::new(x, y, -2.0), 4));
    }
    for i in 0..15 {
        let x = (i as f32 * 71.0 % 60.0) - 30.0;
        let y = (i as f32 * 73.0 % 60.0) - 30.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(tent(nalgebra::Vector3::new(x, y, -2.0), 4));
    }

    // World 5: Ice (snowmen, ice crystals, snowflakes)
    for i in 0..15 {
        let x = (i as f32 * 53.0 % 50.0) - 25.0;
        let y = (i as f32 * 59.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(snowman(nalgebra::Vector3::new(x, y, -2.0), 5));
    }
    for i in 0..20 {
        let x = (i as f32 * 13.0 % 50.0) - 25.0;
        let y = (i as f32 * 17.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(igloo(nalgebra::Vector3::new(x, y, -2.0), 5));
    }
    for i in 0..30 {
        let x = (i as f32 * 61.0 % 50.0) - 25.0;
        let y = (i as f32 * 67.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        triangles.extend(pine_tree(nalgebra::Vector3::new(x, y, -2.0), 5));
    }
    for i in 0..50 {
        let x = (i as f32 * 71.0 % 50.0) - 25.0;
        let y = (i as f32 * 73.0 % 50.0) - 25.0;
        if x * x + y * y < 100.0 { continue; }
        let z = (i as f32 * 79.0 % 15.0) - 1.0;
        triangles.extend(crystal(nalgebra::Vector3::new(x, y, z), 5, [0.8, 0.9, 1.0, 0.8], 0.3));
    }

    triangles
}

pub fn box_shape(
    center: nalgebra::Vector3<f32>,
    world: i32,
    color: [f32; 4],
    size: nalgebra::Vector3<f32>,
) -> impl Iterator<Item = Triangle> {
    let mut colors = [[0.0; 4]; 6];
    colors[world as usize] = color;

    let dx = nalgebra::Vector3::new(size.x, 0.0, 0.0);
    let dy = nalgebra::Vector3::new(0.0, size.y, 0.0);
    let dz = nalgebra::Vector3::new(0.0, 0.0, size.z);

    let v000 = center - dx - dy - dz;
    let v100 = center + dx - dy - dz;
    let v010 = center - dx + dy - dz;
    let v110 = center + dx + dy - dz;
    let v001 = center - dx - dy + dz;
    let v101 = center + dx - dy + dz;
    let v011 = center - dx + dy + dz;
    let v111 = center + dx + dy + dz;

    vec![
        // Bottom
        [v100, v000, v010], [v010, v110, v100],
        // Top
        [v001, v101, v111], [v111, v011, v001],
        // Front
        [v000, v100, v101], [v101, v001, v000],
        // Back
        [v110, v010, v011], [v011, v111, v110],
        // Left
        [v010, v000, v001], [v001, v011, v010],
        // Right
        [v100, v110, v111], [v111, v101, v100],
    ]
    .into_iter()
    .map(move |vertices| Triangle {
        vertices,
        center: Some(center),
        colors,
        ambient_factor: 0.2,
        diffuse_factor: 0.8,
    })
}
