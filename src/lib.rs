// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::f32::consts::{E, PI};

use rand;
use rand::prelude::*;

use chrono::Local;
use seed::{prelude::*, *};

pub struct Price {
    pub max: f32,
    pub calculate: fn(f32) -> f32,
    pub formula: String,
}

fn prices() -> Vec<Price> {
    vec![
        Price {
            calculate: |_| 0.0,
            formula: "\\(y=0\\)".to_string(),
            max: 1.0,
        },
        Price {
            // 1
            calculate: |t| 50.0 * (t * (20.0 * PI * t).sin() + 1.0),
            formula: "\\(x = 50(t \\sin20\\pi t + 1)\\)".to_string(),
            max: 100.0,
        },
        Price {
            // 2
            calculate: |t| 250.0 * (t * (20.0 * PI * t).sin() + 1.0),
            formula: "\\(x = 250(t \\sin20\\pi t + 1)\\)".to_string(),
            max: 500.0,
        },
        Price {
            // 3
            calculate: |t| 200.0 * t.sqrt() + 300.0,
            formula: "\\(x = 200\\sqrt{t} + 300\\)".to_string(),
            max: 500.0,
        },
        Price {
            // 3'
            calculate: |t| 200.0 * (-t + 1.0).sqrt() + 300.0,
            formula: "\\(x = 200\\sqrt{-t+1} + 300\\)".to_string(),
            max: 500.0,
        },
        Price {
            // 4
            calculate: |t| 2000.0 * (t - 1.0 / 2.0).powf(3.0) + 750.0,
            formula: "\\(x = 2000(t-\\frac{1}{2})^3 + 750\\)".to_string(),
            max: 1000.0,
        },
        Price {
            // 5
            calculate: |t| 50.0 * (10.0 * PI * t).sin() + 400.0 * t + 800.0,
            formula: "\\(x = 50\\sin{10\\pi t} + 400 + 800\\)".to_string(),
            max: 1200.0,
        },
        Price {
            // 6
            calculate: |t| 1400.0 * (-t + 3.0).ln(),
            formula: "\\(x = 1400\\ln{(-t + 3)}\\)".to_string(),
            max: 1600.0,
        },
        Price {
            // 7
            calculate: |t| 100.0 * (10.0 * PI * t).cos() - 1000.0 * t + 3000.0,
            formula: "\\(x = 100\\cos{(10\\pi t) - 1000t + 3000}\\)".to_string(),
            max: 3200.0,
        },
        Price {
            // 8
            calculate: |t| 500.0 * (10.0 * PI * t).sin() + 500.0 * (5.0 * PI * t).cos() + 4000.0,
            formula: "\\(x = 500\\sin{(10\\pi t)} + 500\\cos{(5\\pi t)} + 4000\\)".to_string(),
            max: 5000.0,
        },
        Price {
            // 9
            calculate: |t| {
                150.0
                    * ((128.0 * PI * t).sin()
                        + (64.0 * PI * t).sin()
                        + (32.0 * PI * t).sin()
                        + (16.0 * PI * t).sin()
                        + (8.0 * PI * t).sin()
                        + (4.0 * PI * t).sin()
                        + (2.0 * PI * t).sin())
                    - 1200.0 * t
                    + 6800.0
            },
            formula: "\\(x = 150(\\sum_{k=1}^7 \\sin(2^k\\pi t)) - 1200t + 6800\\)".to_string(),
            max: 7500.0,
        },
        Price {
            calculate: |t| 1500.0 * (2.0 * PI * t).sin() + 15000.0,
            formula: "\\(x = 1500\\sin(2\\pi t) + 15000\\)".to_string(),
            max: 18000.0,
        },
        Price {
            // 11
            calculate: |t| {
                let min: f32 = t * 420.0;
                let sec: f32 = t * 420.0 * 60.0;
                let seed_int: u8 = ((min.round() as u64) % 256) as u8;
                let seed: [u8; 32] = [seed_int; 32];
                let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
                rng.gen::<f32>() * 900.0 + 100.0
            },
            formula: "\\(x\\)は\\(100\\)~\\(1000\\)からランダムな値".to_string(),
            max: 1000.0,
        },
    ]
}

fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        id: url.search().get("id").unwrap()[0].parse().unwrap(),
    }
}
struct Model {
    id: i32,
}

#[derive(Copy, Clone)]
enum Msg {}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {}
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    let id = model.id;
    let formula = prices()[id as usize].formula.clone();

    let sec: i64 = Local::now().timestamp() - 1698541200;
    let t: f32 = (sec as f32) / 60.0 / 420.0;
    // let t = 1.0;
    let price = (prices()[id as usize].calculate)(t);
    vec![
        div!(attrs!(At::Id => "title"), p!("大岡山最終処分場。"), hr!()),
        div!(attrs!(At::Id => "menu")),
        div!(
            attrs!(At::Id => "formula"),
            p!(C!("text"), "価格関数"),
            p!(C!("eq"), formula)
        ),
        div!(
            attrs!(At::Id => "now_time"),
            p!(C!("text"), "現在時刻"),
            p!(C!("time"), Local::now().format("%H:%M:%S").to_string()),
            p!(C!("time-t"), format! {"\\(t = {:.3}\\)",t}),
            p!(C!("complain"), "10:00~17:00 が 0~1 に対応"),
        ),
        div!(
            attrs!(At::Id => "now_price"),
            p!(C!("text"), "現在の価格"),
            p!(C!("complain"), "釣り銭のため、価格は1の位を四捨五入"),
            p!(
                C!("price"),
                span!(attrs!(At::Id => "value"), format!("{:.1}", price)),
                span!(attrs!(At::Id => "yen"), "円"),
            ),
        ),
        div!(
            attrs!(At::Id => "graph"),
            p!(C!("text"), "過去の価格変動"),
            canvas!(attrs!(At::Id => "canvas",At::Width => 1400,At::Height => 800),)
        ),
        p!(attrs!(At::Id => "_id"), id),
        p!(attrs!(At::Id => "_time"), format! {"{:.3}",t}),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

// ##### ここから下はグラフ #####

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn power(canvas_id: &str, id: u32, time: &str) -> Result<Chart, JsValue> {
        // time.parse::<f32>().unwrap()
        let map_coord =
            draw(canvas_id, id, time.parse::<f32>().unwrap()).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

/// Draw power function f(x) = x^power.
fn draw(
    canvas_id: &str,
    id: u32,
    time: f32,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let x_font: FontDesc = ("sans-serif", 60.0).into();
    let y_font: FontDesc = ("sans-serif", 60.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(0u32)
        .x_label_area_size(70u32)
        .y_label_area_size(200u32)
        .build_cartesian_2d(0f32..1.05f32, 0f32..prices()[id as usize].max)?;

    chart
        .configure_mesh()
        .x_labels(3)
        .x_label_style(x_font)
        .y_labels(3)
        .y_label_style(y_font)
        .y_label_formatter(&|x: &f32| format!("{}", *x as u32))
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..=500).map(|t| t as f32 / 420.0).map(|t| {
            let mut t = t;
            if t > time {
                t = time
            }
            (t, (prices()[id as usize].calculate)(t))
        }),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}
