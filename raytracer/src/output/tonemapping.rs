use crate::utils::vector::*;

const GAMMA: f32 = 2.2;

// https://64.github.io/tonemapping/

// REINHARD
#[allow(dead_code)]
pub fn reinhard(c: Color) -> Color {
    (c / (c + 1.0)).pow(1.0 / GAMMA)
}

// FILMIC ACES

const _ACES_INPUT_MATRIX: [Vec3; 3] = [
    Vec3 {
        x: 0.59719,
        y: 0.35458,
        z: 0.04823,
    },
    Vec3 {
        x: 0.07600,
        y: 0.90834,
        z: 0.01566,
    },
    Vec3 {
        x: 0.02840,
        y: 0.13383,
        z: 0.83777,
    },
];

const _ACES_OUTPUT_MATRIX: [Vec3; 3] = [
    Vec3 {
        x: 1.60475,
        y: -0.53108,
        z: -0.07367,
    },
    Vec3 {
        x: -0.10208,
        y: 1.10813,
        z: -0.00605,
    },
    Vec3 {
        x: -0.00327,
        y: -0.07276,
        z: 1.07602,
    },
];

fn mul(m: [Vec3; 3], v: &Vec3) -> Vec3 {
    Vec3::new(
        m[0].x * v.x + m[0].y * v.y + m[0].z * v.z,
        m[1].x * v.y + m[1].y * v.y + m[1].z * v.z,
        m[2].x * v.y + m[2].y * v.y + m[2].z * v.z,
    )
}

fn rtt_and_odt_fit(v: Vec3) -> Vec3 {
    let a = v * (v + 0.0245786) - 0.000090537;
    let b = v * (0.983729 * v + 0.4329510) + 0.238081;
    return a / b;
}

#[allow(dead_code)]
pub fn aces(c: Color) -> Color {
    let a = mul(_ACES_INPUT_MATRIX, &c);
    let b = rtt_and_odt_fit(a);
    mul(_ACES_OUTPUT_MATRIX, &b).pow(1.0 / GAMMA)
}

// FILMIC UNREAL
// This one has Gamma Correction baked in.

#[allow(dead_code)]
pub fn unreal(x: Color) -> Color {
    let mut c = x / (x + 0.155) * 1.019;
    c.x = c.x.min(1.0);
    c.y = c.y.min(1.0);
    c.z = c.z.min(1.0);
    return c;
}
