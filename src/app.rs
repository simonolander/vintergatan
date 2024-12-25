use crate::model::board_error::BoardError;
use crate::model::border::Border;
use crate::model::position::Position;
use crate::model::state::State;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, Element, Event};

const VIEW_BOX_SIZE: f64 = 100.0;
const WALL_CELL_RATIO: f64 = 0.1;
const SIZE: i32 = 10;
const CELL_SIZE: f64 = VIEW_BOX_SIZE / (SIZE as f64 + (SIZE as f64 + 1.0) * WALL_CELL_RATIO);
const WALL_SIZE: f64 = CELL_SIZE * WALL_CELL_RATIO;
const SVG_NAMESPACE: Option<&str> = Some("http://www.w3.org/2000/svg");
const WALL_COLOR: &str = "#5a5a5a";

pub struct App {
    state: State,
    border_elements: HashMap<Border, Element>,
    galaxy_center_elements: HashMap<Position, Element>,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        let app = Rc::new(RefCell::new(App {
            state: State::generate(SIZE as usize),
            border_elements: HashMap::new(),
            galaxy_center_elements: HashMap::new(),
        }));

        {
            let svg = document.create_element_ns(SVG_NAMESPACE, "svg")?;
            svg.set_attribute("viewBox", &format!("0 0 {VIEW_BOX_SIZE} {VIEW_BOX_SIZE}"))?;
            svg.set_id("board");
            body.append_child(&svg)?;

            {
                // Add border rectangle
                let rect = document.create_element_ns(SVG_NAMESPACE, "rect")?;
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
                                app.on_border_click(border).unwrap();
                            });
                            wall_svg.add_event_listener_with_callback(
                                "click",
                                closure.as_ref().unchecked_ref(),
                            )?;
                            closure.forget();
                        }
                        app.borrow_mut().border_elements.insert(border, wall_svg);
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
                                app.on_border_click(border).unwrap();
                            });
                            wall_svg.add_event_listener_with_callback(
                                "click",
                                closure.as_ref().unchecked_ref(),
                            )?;
                            closure.forget();
                        }
                        app.borrow_mut().border_elements.insert(border, wall_svg);
                    }
                }
            }

            {
                let mut app = app.borrow_mut();
                for center in app
                    .state
                    .objective
                    .centers
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                {
                    let cx = WALL_SIZE / 2.0
                        + (WALL_SIZE + CELL_SIZE) / 2.0 * (center.position.column + 1) as f64;
                    let cy = WALL_SIZE / 2.0
                        + (WALL_SIZE + CELL_SIZE) / 2.0 * (center.position.row + 1) as f64;
                    let r = CELL_SIZE / 3.0 - WALL_SIZE;
                    let circle = document.create_element_ns(SVG_NAMESPACE, "circle")?;
                    circle.set_attribute("cx", &cx.to_string())?;
                    circle.set_attribute("cy", &cy.to_string())?;
                    circle.set_attribute("r", &r.to_string())?;
                    circle.set_attribute("class", "galaxy-center")?;
                    svg.append_child(&circle)?;
                    app.galaxy_center_elements.insert(center.position, circle);
                }
            }

            let pre = document.create_element("pre")?;
            pre.set_text_content(Some(&app.borrow().state.universe.to_string()));
            pre.set_attribute("style", "display:none")?;
            body.append_child(&pre)?;
        }

        {
            let div = document.create_element("div")?;
            div.set_attribute("class", "controls")?;
            body.append_child(&div)?;

            {
                let check_button = &document.create_element("button")?;
                div.append_child(&check_button)?;
                check_button.set_text_content(Some("Check"));
                let app = Rc::clone(&app);
                let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
                    app.borrow_mut().on_check_click().unwrap();
                });
                check_button
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }
        }

        Ok(app)
    }

    fn on_border_click(&mut self, border: Border) -> Result<(), JsValue> {
        let p1 = border.p1();
        let p2 = border.p2();
        self.state.board.toggle_wall(p1, p2);
        self.state.error = BoardError::none();
        self.render()
    }

    fn on_check_click(&mut self) -> Result<(), JsValue> {
        self.state.error = self.state.board.compute_error(&self.state.objective);
        self.render()
    }

    fn render(&self) -> Result<(), JsValue> {
        // render_cells();
        self.render_borders()?;
        self.render_centers()?;

        Ok(())
    }

    fn render_borders(&self) -> Result<(), JsValue> {
        for (border, element) in &self.border_elements {
            let mut classes = vec!["wall-group"];
            if self.state.error.dangling_segments.contains(&border) {
                classes.push("dangling");
            }
            if self.state.board.is_wall(border.p1(), border.p2()) {
                classes.push("active");
            }
            element.set_attribute("class", &classes.join(" "))?;
        }

        Ok(())
    }

    fn render_centers(&self) -> Result<(), JsValue> {
        for gc in &self.state.objective.centers {
            if let Some(element) = self.galaxy_center_elements.get(&gc.position) {
                let mut classes = vec!["galaxy-center"];
                if self.state.error.cut_centers.contains(&gc.position) {
                    classes.push("cut");
                }
                element.set_attribute("class", &classes.join(" "))?;
            }
        }

        Ok(())
    }
}

fn create_wall_svg(document: &Document, border: Border) -> Result<Element, JsValue> {
    let group = document.create_element_ns(SVG_NAMESPACE, "g")?;
    group.set_attribute("class", "wall-group")?;

    let p1 = border.p1();
    let p2 = border.p2();
    let x_min = WALL_SIZE / 2.0 + (WALL_SIZE + CELL_SIZE) * (p1.column + p2.column) as f64 / 2.0;
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
