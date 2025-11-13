use axum::{Json, extract::Query};
use serde::Deserialize;

use crate::bs::{black_scholes_price, SurfaceData};

#[derive(Deserialize)]
pub struct SurfaceParams {
    pub s: Option<f64>,
    pub k: Option<f64>,
    pub r: Option<f64>,
    pub q: Option<f64>,
}

pub async fn call_surface(Query(params): Query<SurfaceParams>) -> Json<SurfaceData> {
    Json(make_surface(true, params))
}

pub async fn put_surface(Query(params): Query<SurfaceParams>) -> Json<SurfaceData> {
    Json(make_surface(false, params))
}

fn make_surface(is_call: bool, params: SurfaceParams) -> SurfaceData {
    // Defaults if sliders don't send anything
    let s = params.s.unwrap_or(100.0);
    let k = params.k.unwrap_or(100.0);
    let r = params.r.unwrap_or(0.02);
    let q = params.q.unwrap_or(0.0);

    let t_vals: Vec<f64> = (0..=40)
        .map(|i| 0.05 + 0.05 * i as f64) // 0.05 .. ~2.05
        .collect();
    let sigma_vals: Vec<f64> = (0..=40)
        .map(|i| 0.05 + 0.01 * i as f64) // 0.05 .. 0.45
        .collect();

    let mut z = Vec::with_capacity(sigma_vals.len());

    for &sig in &sigma_vals {
        let mut row = Vec::with_capacity(t_vals.len());
        for &t in &t_vals {
            row.push(black_scholes_price(s, k, t, r, sig, q, is_call));
        }
        z.push(row);
    }

    SurfaceData {
        t: t_vals,
        sigma: sigma_vals,
        z,
    }
}
