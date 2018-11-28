extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::{Rgb, RgbImage, ImageBuffer};
use imageproc::{rect::Rect, drawing::*};
use rusttype::{FontCollection, Scale};

use crate::parser::klassendiagramm::klassendiagramm::*;
use crate::parser::relation::*;

#[derive(PartialEq,PartialOrd)]
enum Direction {Up, Right, Down, Left}

pub fn draw_klassendiagramm(klassendiagramm: &Klassendiagramm) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = RgbImage::new(800, 800);
    let (witdth, height) = image.dimensions();
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(witdth, height), Rgb([0xFF, 0xFF, 0xFF]));

    for i in 0..klassendiagramm.get_klassen().len() {
        draw_klasse(&klassendiagramm.get_klassen()[i], &mut image);
    }
    for i in 0..klassendiagramm._get_relationen().len() {
        draw_relation(&klassendiagramm._get_relationen()[i], &mut image);
    }

    return image;
}

fn draw_klasse(klasse: &Klasse, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let scale= 30.0;
    let font_scale = Scale { x: scale, y: scale };

    let color_black = Rgb([0x00, 0x00, 0x00]);
    let text_height = scale;
    let text_x = klasse.get_x() + 2.0;
    let mut text_y = klasse.get_y() + 2.0;

    let rect_x = klasse.get_x() + 0.0;
    let mut rect_y = klasse.get_y() + 0.0;
    let rect_width = rect_width(klasse);
    let mut rect_height;

    // Klasse.id
    for i in 0..klasse.get_id().len() {
        draw_text_mut(image, color_black, text_x as u32, text_y as u32, font_scale, &font, &klasse.get_id()[i]);
        text_y += text_height;
    }
    rect_height = klasse.get_id().len() as u32 * scale as u32 + (scale / 2.0) as u32;
    draw_hollow_rect_mut(image, Rect::at(rect_x as i32, rect_y as i32).of_size(rect_width, rect_height), color_black);
    text_y += text_height / 2.0;

    // Klasse.attribute
    for i in 0..(klasse._get_atr().len()) {
        draw_text_mut(image, color_black, text_x as u32, text_y as u32, font_scale, &font, &klasse._get_atr()[i]);
        text_y += text_height;
    }
    rect_y += (rect_height - 1) as f32;
    rect_height = klasse._get_atr().len() as u32 * scale as u32 + (scale / 2.0) as u32;
    draw_hollow_rect_mut(image, Rect::at(rect_x as i32, rect_y as i32).of_size(rect_width, rect_height), color_black);
    text_y += text_height / 2.0;

    // Klasse.methoden
    for i in 0..klasse._get_meth().len() {
        draw_text_mut(image, color_black, text_x as u32, text_y as u32, font_scale, &font, &klasse._get_meth()[i]);
        text_y += text_height;
    }
    rect_y += (rect_height - 1) as f32;
    rect_height = klasse._get_meth().len() as u32 * scale as u32 + (scale / 2.0) as u32;
    draw_hollow_rect_mut(image, Rect::at(rect_x as i32, rect_y as i32).of_size(rect_width, rect_height), color_black);
}

fn draw_relation(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let color_white =  Rgb([0xFF, 0xFF, 0xFF]);
    for i in 0..relation.get_koord().len() - 1 {
        let mut start = (relation.get_koord()[i].0 as f32, relation.get_koord()[i].1 as f32);
        let mut end = (relation.get_koord()[i + 1].0 as f32, relation.get_koord()[i + 1].1 as f32);
        draw_line_segment_mut(image, start, end, color_black);
    }
        draw_inher(&relation, image);
}

fn draw_aggr(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let point: &[Point<i32>] = &[
        Point::new(relation.get_koord()[0].0 as i32, relation.get_koord()[0].1 as i32),
        Point::new(relation.get_koord()[0].0 as i32 + 10, relation.get_koord()[0].1 as i32 + 10),
        Point::new(relation.get_koord()[0].0 as i32, relation.get_koord()[0].1 as i32 + 20),
        Point::new(relation.get_koord()[0].0 as i32 - 10, relation.get_koord()[0].1 as i32 + 10)
    ];
    draw_convex_polygon_mut(image, &point, color_black);
}

fn draw_komp(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_white =  Rgb([0xFF, 0xFF, 0xFF]);
    draw_aggr(relation, image);
    let mut point: &[Point<i32>] = &[
        Point::new(relation.get_koord()[0].0 as i32, relation.get_koord()[0].1 as i32 + 2),
        Point::new(relation.get_koord()[0].0 as i32 + 8, relation.get_koord()[0].1 as i32 + 10),
        Point::new(relation.get_koord()[0].0 as i32, relation.get_koord()[0].1 as i32 + 18),
        Point::new(relation.get_koord()[0].0 as i32 - 8, relation.get_koord()[0].1 as i32 + 10)
    ];
    draw_convex_polygon_mut(image, &point, color_white);
}

fn draw_impl(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
}

fn draw_inher(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let color_white = Rgb([0xFF, 0xFF, 0xFF]);
    let start = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32, relation.get_koord()[relation.get_koord().len() - 1].1 as f32);
    let mut end1 = (0.0, 0.0);
    let mut end2 = (0.0, 0.0);
    if get_end_dir(relation) == Direction::Up {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
        draw_line_segment_mut(image, start, (start.0, start.1 + 4.0), color_white);
    } else if get_end_dir(relation) == Direction::Right {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
        draw_line_segment_mut(image, start, (start.0 - 4.0, start.1), color_white);
    } else if get_end_dir(relation) == Direction::Down {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        draw_line_segment_mut(image, start, (start.0, start.1 - 4.0), color_white);
    } else {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
        draw_line_segment_mut(image, start, (start.0 + 4.0, start.1), color_white);
    }
    draw_line_segment_mut(image, start, end1, color_black);
    draw_line_segment_mut(image, start, end2, color_black);
    draw_line_segment_mut(image, end1, end2, color_black);

}

fn draw_abh(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);

}

fn draw_einfach(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let start = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32, relation.get_koord()[relation.get_koord().len() - 1].1 as f32);
    let mut end1 = (0.0, 0.0);
    let mut end2 = (0.0, 0.0);
    if get_end_dir(relation) == Direction::Up {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
    } else if get_end_dir(relation) == Direction::Right {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
    } else if get_end_dir(relation) == Direction::Down {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
    } else {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 + 5.0);
    }
    draw_line_segment_mut(image, start, end1, color_black);
    draw_line_segment_mut(image, start, end2, color_black);
}

fn get_end_dir(relation: &Relation) -> Direction {
    return get_dir(relation.get_koord()[relation.get_koord().len() - 2].0 as f32, relation.get_koord()[relation.get_koord().len() - 2].1 as f32, relation.get_koord()[relation.get_koord().len() - 1].0 as f32, relation.get_koord()[relation.get_koord().len() - 1].1 as f32);
}

fn get_start_dir(relation: &Relation) -> Direction {
    return get_dir(relation.get_koord()[0].0 as f32, relation.get_koord()[0].1 as f32, relation.get_koord()[1].0 as f32, relation.get_koord()[1].1 as f32);
}

fn get_dir(x1: f32, y1: f32, x2: f32, y2: f32) -> Direction {
    if y1 - y2 > 0.0  && x1 - x2 == 0.0 {
        return Direction::Up;
    } else if x1 - x2 < 0.0 && y1 - y2 == 0.0 {
        return Direction::Right;
    } else if y1 - y2 < 0.0  && x1 - x2 == 0.0 {
        return Direction::Down;
    } else {
        return Direction::Left;
    }
}

fn rect_width(klasse: &Klasse) -> u32 {
    let mut max_width = 0;
    for i in 0..(klasse.get_id().len()) {
        if klasse.get_id()[i].len() > max_width {
            max_width = klasse.get_id()[i].len();
        }
    }
    for i in 0..(klasse._get_atr().len()) {
        if klasse._get_atr()[i].len() > max_width {
            max_width = klasse._get_atr()[i].len();
        }
    }
    for i in 0..(klasse._get_meth().len()) {
        if klasse._get_meth()[i].len() > max_width {
            max_width = klasse._get_meth()[i].len();
        }
    }
    return (max_width * 17) as u32;
}