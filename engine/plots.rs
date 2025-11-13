use plotly::{Layout, Plot, Surface, Scatter};
use plotly::layout::{LayoutScene, Axis};
use plotly::common::{Line, Mode};
use std::f64;

use crate::math::stochasticd::generate_srw;
use crate::finance::options::black_scholes_price;

pub fn plot_srw_ensemble(
    positive_delta: f64,
    negative_delta: f64,
    num_trials: usize,
) {
    let num_paths = 50;

    // Generate all SRWs
    let mut paths: Vec<Vec<f64>> = Vec::with_capacity(num_paths);
    for _ in 0..num_paths {
        let path = generate_srw(positive_delta, negative_delta, num_trials);
        paths.push(path);
    }

    // Assume all paths have the same length
    let len = paths[0].len();
    let time: Vec<usize> = (0..len).collect();

    // Compute pointwise sum of all SRWs
    let mut sum_path = vec![0.0_f64; len];
    for path in &paths {
        for (i, &v) in path.iter().enumerate() {
            sum_path[i] += v;
        }
    }

    let mut plot = Plot::new();

    // Add SRW paths with fading colour
    //
    // We'll fade alpha from 0.15 (faint) to 0.9 (strong) over the 50 paths.
    let alpha_start = 0.15;
    let alpha_end = 0.9;

    for (idx, path) in paths.iter().enumerate() {
        let t = idx as f64 / (num_paths - 1) as f64;
        let alpha = alpha_start + t * (alpha_end - alpha_start);

        // Blue-ish colour with varying alpha
        let color = format!("rgba(80, 160, 255, {alpha:.3})");

        let trace = Scatter::new(time.clone(), path.clone())
            .mode(Mode::Lines)
            .line(
                Line::new()
                    .color(color)
                    .width(1.0),
            );

        plot.add_trace(trace);
    }

    // Add the sum of all SRWs as a bold contrasting line
    let sum_trace = Scatter::new(time.clone(), sum_path)
        .mode(Mode::Lines)
        .line(
            Line::new()
                .color("rgba(255, 80, 80, 1.0)") // strong red
                .width(3.0),
        )
        .name("Sum of all SRWs");

    plot.add_trace(sum_trace);

    let layout = Layout::new()
    .title("Ensemble of Simple Random Walks (50 paths) with Sum")
    .x_axis(
        plotly::layout::Axis::new()
            .title("Step")
    )
    .y_axis(
        plotly::layout::Axis::new()
            .title("Position")
    );

    plot.set_layout(layout);

    // Show in browser
    plot.show();
}

fn bs_grid(s: f64, k: f64, r: f64, q: f64)
    -> (Vec<f64>, Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>)
{
    let t_vals: Vec<f64> = (0..=40).map(|i| 0.05 + 0.05 * i as f64).collect();
    let sigma_vals: Vec<f64> = (0..=40).map(|i| 0.05 + 0.01 * i as f64).collect();

    let mut z_call = Vec::new();
    let mut z_put  = Vec::new();

    for &sigma in &sigma_vals {
        let mut row_c = Vec::new();
        let mut row_p = Vec::new();

        for &t in &t_vals {
            row_c.push(black_scholes_price(s, k, t, r, sigma, q, true));
            row_p.push(black_scholes_price(s, k, t, r, sigma, q, false));
        }

        z_call.push(row_c);
        z_put.push(row_p);
    }

    (t_vals, sigma_vals, z_call, z_put)
}

pub fn plot_bs_surfaces_side_by_side() {
    let s = 100.0;
    let k = 100.0;
    let r = 0.02;
    let q = 0.0;

    let (t_vals, sigma_vals, z_call, z_put) = bs_grid(s, k, r, q);

    let mut plot = Plot::new();

    // --- Call surface (left) ---
    let call_surface = Surface::new(z_call)
        .x(t_vals.clone())
        .y(sigma_vals.clone())
        .name("Call Price");

    // --- Put surface (right) ---
    let put_surface = Surface::new(z_put)
        .x(t_vals.clone())
        .y(sigma_vals.clone())
        .name("Put Price");
        

    plot.add_trace(call_surface);
    plot.add_trace(put_surface);

    // Left 3D subplot
    let scene1 = LayoutScene::new()
        .x_axis(Axis::new().title("T (years)"))
        .y_axis(Axis::new().title("Volatility σ"))
        .z_axis(Axis::new().title("Call Price"));

    // Right 3D subplot
    let scene2 = LayoutScene::new()
        .x_axis(Axis::new().title("T (years)"))
        .y_axis(Axis::new().title("Volatility σ"))
        .z_axis(Axis::new().title("Put Price"));

    let layout = Layout::new()
        .title("Black–Scholes Call & Put Surfaces (Side-by-Side)")
        .scene(scene1)
        .scene(scene2)
        .width(1600)
        .height(900);

    plot.set_layout(layout);
    plot.show();
}