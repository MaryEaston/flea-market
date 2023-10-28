// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        head_view(&model),
        div!(
            attrs!(At::Id => "container"),
            div!(
                attrs!(At::Id => "formula"),
                p!(C!("text"), "価格関数"),
                p!(C!("eq"), "\\(y= \\left\\lceil 3\\sin \\left( 10\\pi t \\right)+1 \\right\\rceil \\times 10\\)")
            ),
            div!(
                attrs!(At::Id => "now_time"),
                p!(C!("text"), "現在時刻"),
                p!(C!("text"), "18:58:13"),
                p!(C!("text"), "\\(t = 1242.42\\) [\\(\\times\\) 420 分]"),
            ),
            div!(
                attrs!(At::Id => "now_price"),
                p!(C!("text"), "現在の価格"),
                p!(
                    C!("price"),
                    span!(attrs!(At::Id => "value"), "40"),
                    span!(attrs!(At::Id => "yes"), "円"),
                )
            ),
            div!(attrs!(At::Id => "graph"), p!(C!("text"), "過去の価格変動"),canvas!(attrs!(At::Id => "canvas",At::Width => 600,At::Height => 400),))
        ),
    ]
}

fn head_view(_model: &Model) -> Node<Msg> {
    div!(attrs!(At::Id => "title"), p!("処分場"), hr!())
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

// ##### ここから下はグラフ #####

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod func_plot;

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
    pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
        let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
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
