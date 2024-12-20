use crate::model::border::Border;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, Element, Event};
use crate::model::position::Position;

const VIEW_BOX_SIZE: f64 = 100.0;
const WALL_CELL_RATIO: f64 = 0.1;
const SIZE: i32 = 10;
const CELL_SIZE: f64 = VIEW_BOX_SIZE / (SIZE as f64 + (SIZE as f64 + 1.0) * WALL_CELL_RATIO);
const WALL_SIZE: f64 = CELL_SIZE * WALL_CELL_RATIO;
const SVG_NAMESPACE: Option<&str> = Some("http://www.w3.org/2000/svg");
const WALL_COLOR: &str = "#5a5a5a";

pub struct App {
    walls: HashMap<Border, Element>,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        let app = Rc::new(RefCell::new(App {
            walls: HashMap::new(),
        }));

        {
            let svg = document.create_element_ns(SVG_NAMESPACE, "svg")?;
            svg.set_attribute("viewBox", &format!("0 0 {VIEW_BOX_SIZE} {VIEW_BOX_SIZE}"))?;
            svg.set_id("board");
            body.append_child(&svg)?;

            {
                // Add border rectangle
                let mut rect = document.create_element_ns(SVG_NAMESPACE, "rect")?;
                rect.set_attribute("x", &(WALL_SIZE / 2.0).to_string())?;
                rect.set_attribute("y", &(WALL_SIZE / 2.0).to_string())?;
                rect.set_attribute("width", &(VIEW_BOX_SIZE - WALL_SIZE).to_string())?;
                rect.set_attribute("height", &(VIEW_BOX_SIZE - WALL_SIZE).to_string())?;
                rect.set_attribute("stroke", WALL_COLOR)?;
                rect.set_attribute("stroke-width", &WALL_SIZE.to_string())?;
                rect.set_attribute("fill", "none")?;
                svg.append_child(&rect)?;
            }

            {
                // Add vertical walls
                for row in 0..SIZE {
                    for col in 0..SIZE - 1 {
                        let p1 = Position::new(row, col);
                        let p2 = p1.right();
                        let border = Border::new(p1, p2);
                        let wall_svg = create_wall_svg(&document, border)?;
                        svg.append_child(&wall_svg)?;
                        {
                            let app = Rc::clone(&app);
                            let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
                                let mut app = app.borrow_mut();
                                app.on_click(border);
                            });
                            wall_svg.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                            closure.forget();
                        }
                        app.borrow_mut().walls.insert(border, wall_svg);
                    }
                }

                // Horizontal walls
                for row in 0..SIZE - 1 {
                    for col in 0..SIZE {
                        let p1 = Position::new(row, col);
                        let p2 = p1.down();
                        let border = Border::new(p1, p2);
                        let wall_svg = create_wall_svg(&document, border)?;
                        svg.append_child(&wall_svg)?;
                        {
                            let app = Rc::clone(&app);
                            let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
                                let mut app = app.borrow_mut();
                                app.on_click(border);
                            });
                            wall_svg.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                            closure.forget();
                        }
                        app.borrow_mut().walls.insert(border, wall_svg);
                    }
                }
            }
        }

        Ok(app)
    }

    fn on_click(&mut self, border: Border) {}
}

fn create_wall_svg(
    document: &Document,
    border: Border,
) -> Result<Element, JsValue> {
    let group = document.create_element_ns(SVG_NAMESPACE, "g")?;
    group.set_attribute("class", "wall-group")?;

    let p1 = border.p1();
    let p2 = border.p2();
    let x_min =
        WALL_SIZE / 2.0 + (WALL_SIZE + CELL_SIZE) * (p1.column + p2.column) as f64 / 2.0;
    let x_max = x_min + CELL_SIZE + WALL_SIZE;
    let x_mid = (x_min + x_max) / 2.0;
    let y_min = WALL_SIZE / 2.0 + (WALL_SIZE + CELL_SIZE) * (p1.row + p2.row) as f64 / 2.0;
    let y_max = y_min + CELL_SIZE + WALL_SIZE;
    let y_mid = (y_min + y_max) / 2.0;

    {
        let polygon = document.create_element_ns(SVG_NAMESPACE, "polygon")?;
        polygon.set_attribute(
            "points",
            &format!(
                "{},{} {},{} {},{} {},{}",
                x_mid, y_min, x_max, y_mid, x_mid, y_max, x_min, y_mid
            ),
        )?;
        polygon.set_attribute("class", "wall-touch")?;
        group.append_child(&polygon)?;
    }

    {
        let line = document.create_element_ns(SVG_NAMESPACE, "line")?;
        line.set_attribute("class", "wall-line")?;
        if p1.row == p2.row {
            // Vertical
            line.set_attribute("x1", &x_mid.to_string())?;
            line.set_attribute("y1", &y_min.to_string())?;
            line.set_attribute("x2", &x_mid.to_string())?;
            line.set_attribute("y2", &y_max.to_string())?;
        } else {
            // Horizontal
            line.set_attribute("x1", &x_min.to_string())?;
            line.set_attribute("y1", &y_mid.to_string())?;
            line.set_attribute("x2", &x_max.to_string())?;
            line.set_attribute("y2", &y_mid.to_string())?;
        }
        line.set_attribute("stroke-width", &WALL_SIZE.to_string())?;
        group.append_child(&line)?;
    }

    Ok(group)
}