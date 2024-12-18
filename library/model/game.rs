use crate::model::board::Board;
use crate::model::position::Position;
use std::cmp::min;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    window, Document, Element, Event, SvgElement, SvgLineElement, SvgPolygonElement,
    SvgRectElement, SvggElement,
};

pub struct Game {
    board: Board,
    document: Document,
    walls: HashMap<(Position, Position), Element>,
}

impl Game {
    const VIEW_BOX_SIZE: f64 = 100.0;
    const WALL_CELL_RATIO: f64 = 0.1;
    const SVG_NS: &'static str = "http://www.w3.org/2000/svg";

    pub fn new() -> Game {
        Game {
            board: Board::new(10, 10),
            document: window().unwrap().document().unwrap(),
            walls: HashMap::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        let body = self.document.body().unwrap();
        body.set_inner_html("");
        self.walls.clear();

        let svg = self.create_board_svg()?;
        body.append_child(&svg)?;

        let border_rect = self.create_boarder_rect()?;
        svg.append_child(&border_rect)?;

        // Vertical walls
        for row in 0..self.board.get_height() as i32 {
            for col in 0..self.board.get_width() as i32 - 1 {
                let p1 = Position::new(row, col);
                let p2 = p1.right();
                let wall_element = self.create_wall_svg(p1, p2)?;
                svg.append_child(wall_element.as_ref())?;
                self.walls.insert((p1, p2), wall_element);
            }
        }

        // Horizontal walls
        for col in 0..self.board.get_height() as i32 {
            for row in 0..self.board.get_width() as i32 - 1 {
                let p1 = Position::new(row, col);
                let p2 = p1.down();
                let wall_element = self.create_wall_svg(p1, p2)?;
                svg.append_child(wall_element.as_ref())?;
                self.walls.insert((p1, p2), wall_element);
            }
        }

        Ok(())
    }

    fn create_boarder_rect(&self) -> Result<SvgRectElement, JsValue> {
        let size = self.board.get_width() as f64;
        let cell_size = Self::VIEW_BOX_SIZE / (size + (size + 1.0) * Self::WALL_CELL_RATIO);
        let wall_size = cell_size * Self::WALL_CELL_RATIO;
        let mut rect: SvgRectElement = self
            .document
            .create_element_ns(Some(Self::SVG_NS), "rect")?
            .dyn_into()?;
        rect.set_attribute("x", &(wall_size / 2.0).to_string())?;
        rect.set_attribute("y", &(wall_size / 2.0).to_string())?;
        rect.set_attribute("width", &(Self::VIEW_BOX_SIZE - wall_size).to_string())?;
        rect.set_attribute("height", &(Self::VIEW_BOX_SIZE - wall_size).to_string())?;
        rect.set_attribute("stroke", "#5a5a5a")?;
        rect.set_attribute("stroke-width", &(wall_size).to_string())?;
        rect.set_attribute("fill", "none")?;
        Ok(rect)
    }

    fn create_board_svg(&self) -> Result<SvgElement, JsValue> {
        let svg: SvgElement = self
            .document
            .create_element_ns(Some(Self::SVG_NS), "svg")?
            .dyn_into()?;
        svg.set_attribute(
            "viewBox",
            &format!("0 0 {} {}", Self::VIEW_BOX_SIZE, Self::VIEW_BOX_SIZE),
        )?;
        svg.set_id("board");
        Ok(svg)
    }

    fn create_wall_svg(&mut self, p1: Position, p2: Position) -> Result<Element, JsValue> {
        let size = self.board.get_width() as f64;
        let cell_size = Self::VIEW_BOX_SIZE / (size + (size + 1.0) * Self::WALL_CELL_RATIO);
        let wall_size = cell_size * Self::WALL_CELL_RATIO;

        let group = self.document.create_element_ns(Some(Self::SVG_NS), "g")?;
        group.set_attribute("class", "wall-group")?;

        let x_min = wall_size / 2.0 + (wall_size + cell_size) * (p1.column + p2.column) as f64 / 2.0;
        let x_max = x_min + cell_size + wall_size;
        let x_mid = (x_min + x_max) / 2.0;
        let y_min = wall_size / 2.0 + (wall_size + cell_size) * (p1.row + p2.row) as f64 / 2.0;
        let y_max = y_min + cell_size + wall_size;
        let y_mid = (y_min + y_max) / 2.0;
        {
            let polygon: SvgPolygonElement = self
                .document
                .create_element_ns(Some(Self::SVG_NS), "polygon")?
                .dyn_into()?;
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
            let line: SvgLineElement = self
                .document
                .create_element_ns(Some(Self::SVG_NS), "line")?
                .dyn_into()?;
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
            line.set_attribute("stroke-width", &wall_size.to_string())?;
            group.append_child(&line)?;
        }

        let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
            self.on_wall_click(p1, p2);
        });
        group.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(group)
    }

    fn get_wall_element(&self, p1: Position, p2: Position) -> Option<&Element> {
        self.walls.get(&(p1, p2))
    }

    fn on_wall_click(&mut self, p1: Position, p2: Position) {
        self.board.toggle_wall(p1, p2);
        if let Some(element) = self.get_wall_element(p1, p2) {
            if self.board.is_wall(p1, p2) {
                element.set_attribute("class", "wall-line active").unwrap();
            } else {
                element.set_attribute("class", "wall-line").unwrap();
            }
        }
    }

    fn clear_errors(&mut self) {}
}
