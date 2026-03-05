mod quartic;

/*
 ┏━━┓  ┏━━┓
┏┛  ┗┓┏┛  ┗┓
┃    ┗┓    ┃
┃   ┏┛┗┓   ┃
┗┓ ┏┛  ┗┓ ┏┛
 ┗━┃━━━━━━┛
   ┃    ┃
   ┗┓  ┏┛
    ┗━━┛

The boundary of the trefoil portal is parameterized by (sin(t) + 2sin(2t), cos(t)-2cos(2t), sin(3t)).

The trefoil's projection onto the xy-plane is the solution set of this quartic equation:
    4rrrr - 12rry + 16yyy - 27rr + 27 = 0
(rr == xx + yy)

The trefoil lies on this (topological) torus:
    zz = 1 - (rr - 5)^2 / 16

The xy plane is divided into twelve important regions by these inequalities:
    1.) x > 0
    2.) x < y √3
    3.) x < -y √3
    4.) r > 1.5

For a point (x,y,z) on the trefoil, z is positive whenever an even number of these inequalities hold.

The inequalities also allow you to deduce which arc in the knot diagram contains a given point.

                       ┃  (4) holds  │ (4) doesn't ┃
━━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━┿━━━━━━━━━━━━━┫
(2) holds, (1) doesn't ┃    Arc A    │    Arc C    ┃
───────────────────────╂─────────────┼─────────────┨
(1) holds, (3) doesn't ┃    Arc B    │    Arc A    ┃
───────────────────────╂─────────────┼─────────────┨
(3) holds, (2) doesn't ┃    Arc C    │    Arc B    ┃
━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━┷━━━━━━━━━━━━━┛

(A = top left, B = right, C = bottom)


Passing under an arc causes you to switch worlds.

      ╔═══╗       ╔═══╗
      ║ 1 ║───C───║ 2 ║
      ╚═══╝       ╚═══╝
      ╱   ╲       ╱   ╲
     A     B     A     B
    ╱       ╲   ╱       ╲
╔═══╗        ╲ ╱        ╔═══╗
║ 0 ║───C─────╳─────C───║ 3 ║
╚═══╝        ╱ ╲        ╚═══╝
    ╲       ╱   ╲       ╱
     B     A     B     A
      ╲   ╱       ╲   ╱
      ╔═══╗       ╔═══╗
      ║ 5 ║───C───║ 4 ║
      ╚═══╝       ╚═══╝

*/

const SQRT_3: f32 = 1.732_050_8;

const NUM_SPHERES: usize = 6;
const SPHERE_CENTERS: [[f32; 3]; NUM_SPHERES] = [
    [0.0, 0.0, 5.0],
    [0.0, 6.0, 2.0],
    [6.0, 0.0, 2.0],
    [-6.0, 0.0, 2.0],
    [0.0, -6.0, 2.0],
    [0.0, 0.0, 8.0],
];
const SPHERE_RADII: [f32; NUM_SPHERES] = [1.5, 1.5, 1.5, 1.5, 1.5, 1.5];
const SPHERE_WORLD_A: [i32; NUM_SPHERES] = [0, 2, 4, 0, 1, 4];
const SPHERE_WORLD_B: [i32; NUM_SPHERES] = [1, 3, 5, 2, 3, 0];

// If you travel in a straight line from `start` to `end`, in which world do you end up?
#[rustfmt::skip]
pub fn travel(world: &mut i32, start: nalgebra::Vector3<f32>, end: nalgebra::Vector3<f32>) {

    // We define `x(t)`, `y(t)` to be linear polynomials parameterizing the line of travel.
    // Then we calculate `trefoil_projection_quartic(x(t), y(t))`, which is a quartic polynomial in t.
    // If t is a root of that quartic, then (x(t), y(t)) lies on the projection of the trefoil.

    // Linear Polynomials
    let mut x: [f32; 2] = [7777.; 2];
    let mut y: [f32; 2] = [7777.; 2];

    let mut v_xy = (end - start).xy();
    let v_full = end - start;
    let t_max = v_full.norm();
    let mut v = v_full / t_max;
    v_xy /= t_max;

    x[0] = start.x;
    y[0] = start.y;

    x[1] = v_xy.x;
    y[1] = v_xy.y;


    // Quadratic Polynomial
    let mut rr: [f32; 3] = [7777.; 3];
    rr[0] =       x[0] * x[0] +       y[0] * y[0];
    rr[1] = 2.0 * x[0] * x[1] + 2.0 * y[0] * y[1];
    rr[2] =       x[1] * x[1] +       y[1] * y[1];


    // Quartic Polynomial
    let mut poly: [f32; 5] = [7777.; 5];
    poly[0] = 4.0 * (      rr[0] * rr[0]                ) - 12.0 * (rr[0] * y[0]               ) + (16.0 * y[0] * y[0] * y[0]) - 27.0 * rr[0] + 27.0;
    poly[1] = 4.0 * (2.0 * rr[0] * rr[1]                ) - 12.0 * (rr[1] * y[0] + rr[0] * y[1]) + (48.0 * y[0] * y[0] * y[1]) - 27.0 * rr[1];
    poly[2] = 4.0 * (2.0 * rr[0] * rr[2] + rr[1] * rr[1]) - 12.0 * (rr[2] * y[0] + rr[1] * y[1]) + (48.0 * y[0] * y[1] * y[1]) - 27.0 * rr[2];
    poly[3] = 4.0 * (2.0 * rr[1] * rr[2]                ) - 12.0 * (               rr[2] * y[1]) + (16.0 * y[1] * y[1] * y[1]);
    poly[4] = 4.0 * (      rr[2] * rr[2]                );



    let mut roots: [f32; 4] = [6666.; 4];
	let num_roots: usize = quartic::quartic(
		poly[3] / poly[4],
		poly[2] / poly[4],
		poly[1] / poly[4],
		poly[0] / poly[4],
		&mut roots
	);

    let mut ints_t = [0.0; 16];
    let mut ints_type = [0; 16];
    let mut num_ints = 0;

    for &root in roots.iter().take(num_roots) {
        if 0.0 < root && root < t_max {
            ints_t[num_ints] = root;
            ints_type[num_ints] = 0;
            num_ints += 1;
        }
    }

    for i in 0..NUM_SPHERES {
        let center = nalgebra::Vector3::new(SPHERE_CENTERS[i][0], SPHERE_CENTERS[i][1], SPHERE_CENTERS[i][2]);
        let oc = start - center;
        let b = oc.dot(&v);
        let c = oc.dot(&oc) - SPHERE_RADII[i] * SPHERE_RADII[i];
        let discriminant = b * b - c;
        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();
            let t1 = -b - sqrt_d;
            if 0.0 < t1 && t1 < t_max {
                ints_t[num_ints] = t1;
                ints_type[num_ints] = i as i32 + 1;
                num_ints += 1;
            }
        }
    }

    for i in 1..num_ints {
        let key_t = ints_t[i];
        let key_type = ints_type[i];
        let mut j = i as isize - 1;
        while j >= 0 && ints_t[j as usize] > key_t {
            ints_t[(j + 1) as usize] = ints_t[j as usize];
            ints_type[(j + 1) as usize] = ints_type[j as usize];
            j -= 1;
        }
        ints_t[(j + 1) as usize] = key_t;
        ints_type[(j + 1) as usize] = key_type;
    }

    for i in 0..num_ints {
        let t = ints_t[i];
        let typ = ints_type[i];

        if typ == 0 {
            let pos = start.lerp(&end, t / t_max);

            let rr: f32 = pos.x*pos.x + pos.y*pos.y;

            let test1: bool = pos.x > 0.0;
            let test2: bool = pos.x < pos.y * SQRT_3;
            let test3: bool = pos.x < pos.y * -SQRT_3;
            let test4: bool = rr > 2.25;

            let trefoil_z: f32 =
                (1.0 - ((rr - 5.0) * (rr - 5.0) / 16.0)).max(0.0).sqrt() *
                (if test1 ^ test2 ^ test3 ^ test4 {-1.0} else {1.0});

            if pos.z < trefoil_z {
                // Arc A = 1, B = 5, C = 3
                #[allow(clippy::suspicious_else_formatting, clippy::collapsible_if)]
                let mut arc: i32 = if test1
                    {if test3 {3} else {5}} else
                    {if test2 {1} else {3}};
                arc += if test4 {0} else {2};

                // *world = arc - *world;
            }
        } else {
            let s_idx = (typ - 1) as usize;
            let wa = SPHERE_WORLD_A[s_idx];
            let wb = SPHERE_WORLD_B[s_idx];
            let norm_world = world.rem_euclid(6);
            if norm_world == wa {
                *world = *world - norm_world + wb;
            } else if norm_world == wb {
                *world = *world - norm_world + wa;
            }
        }
    }

    *world = world.rem_euclid(6);
}
