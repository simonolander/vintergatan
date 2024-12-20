// use crate::model::border::Border;
// use std::collections::HashMap;
// use web_sys::wasm_bindgen::prelude::*;
// use web_sys::{Document, Element, Event};
//
// const VIEW_BOX_SIZE: f64 = 100.0;
// const WALL_CELL_RATIO: f64 = 0.1;
// const SVG_NS: &str = "http://www.w3.org/2000/svg";
//
// struct UI {
//     board: BoardUI,
// }
//
// struct BoardUI {
//     svg_element: Element,
//     wall_elements: HashMap<Border, Element>,
// }
//
// impl BoardUI {
//     pub fn new(document: &Document, size: usize) -> Result<BoardUI, JsValue> {
//         let cell_size = Self::get_cell_size(size);
//         let wall_size = Self::get_wall_size(cell_size);
//
//         let svg_element = document.create_element_ns(Some(SVG_NS), "svg")?;
//         svg_element.set_attribute(
//             "viewBox",
//             &format!("0 0 {} {}", VIEW_BOX_SIZE, VIEW_BOX_SIZE),
//         )?;
//         svg_element.set_id("board");
//
//         {
//             let mut rect = document.create_element_ns(Some(SVG_NS), "rect")?;
//             rect.set_attribute("x", &(wall_size / 2.0).to_string())?;
//             rect.set_attribute("y", &(wall_size / 2.0).to_string())?;
//             rect.set_attribute("width", &(VIEW_BOX_SIZE - wall_size).to_string())?;
//             rect.set_attribute("height", &(VIEW_BOX_SIZE - wall_size).to_string())?;
//             rect.set_attribute("stroke", "#5a5a5a")?;
//             rect.set_attribute("stroke-width", &(wall_size).to_string())?;
//             rect.set_attribute("fill", "none")?;
//             svg_element.append_child(&rect)?;
//         }
//
//         Ok(rect)
//     }
//
//     fn get_cell_size(size: usize) -> f64 {
//         VIEW_BOX_SIZE / (size as f64 + (size as f64 + 1.0) * WALL_CELL_RATIO)
//     }
//
//     fn get_wall_size(cell_size: f64) -> f64 {
//         cell_size * WALL_CELL_RATIO
//     }
//
//     fn create_wall_svg(
//         &mut self,
//         document: &Document,
//         border: Border,
//         cell_size: f64,
//         wall_size: f64,
//     ) -> Result<Element, JsValue> {
//         let group = document.create_element_ns(Some(SVG_NS), "g")?;
//         group.set_attribute("class", "wall-group")?;
//
//         let p1 = border.p1();
//         let p2 = border.p2();
//         let x_min =
//             wall_size / 2.0 + (wall_size + cell_size) * (p1.column + p2.column) as f64 / 2.0;
//         let x_max = x_min + cell_size + wall_size;
//         let x_mid = (x_min + x_max) / 2.0;
//         let y_min = wall_size / 2.0 + (wall_size + cell_size) * (p1.row + p2.row) as f64 / 2.0;
//         let y_max = y_min + cell_size + wall_size;
//         let y_mid = (y_min + y_max) / 2.0;
//
//         {
//             let polygon = document.create_element_ns(Some(SVG_NS), "polygon")?;
//             polygon.set_attribute(
//                 "points",
//                 &format!(
//                     "{},{} {},{} {},{} {},{}",
//                     x_mid, y_min, x_max, y_mid, x_mid, y_max, x_min, y_mid
//                 ),
//             )?;
//             polygon.set_attribute("class", "wall-touch")?;
//             group.append_child(&polygon)?;
//         }
//
//         {
//             let line = document.create_element_ns(Some(SVG_NS), "line")?;
//             line.set_attribute("class", "wall-line")?;
//             if p1.row == p2.row {
//                 // Vertical
//                 line.set_attribute("x1", &x_mid.to_string())?;
//                 line.set_attribute("y1", &y_min.to_string())?;
//                 line.set_attribute("x2", &x_mid.to_string())?;
//                 line.set_attribute("y2", &y_max.to_string())?;
//             } else {
//                 // Horizontal
//                 line.set_attribute("x1", &x_min.to_string())?;
//                 line.set_attribute("y1", &y_mid.to_string())?;
//                 line.set_attribute("x2", &x_max.to_string())?;
//                 line.set_attribute("y2", &y_mid.to_string())?;
//             }
//             line.set_attribute("stroke-width", &wall_size.to_string())?;
//             group.append_child(&line)?;
//         }
//
//         let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
//             self.on_wall_click(p1, p2);
//         });
//         group.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
//         closure.forget();
//
//         Ok(group)
//     }
// }
