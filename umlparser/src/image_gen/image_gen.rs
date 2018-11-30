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

static SCALE: f32 = 1.0;
pub fn draw_klassendiagramm(klassendiagramm: &Klassendiagramm) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = RgbImage::new(2500, 2500);
    let (witdth, height) = image.dimensions();
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(witdth, height), Rgb([0xFF, 0xFF, 0xFF]));


    for i in 0..klassendiagramm._get_relationen().len() {
        draw_relation(&klassendiagramm._get_relationen()[i], &mut image);
    }
    for i in 0..klassendiagramm.get_klassen().len() {
        draw_klasse(&klassendiagramm.get_klassen()[i], &mut image);
    }

    return image;
}

fn draw_klasse(klasse: &Klasse, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {

    let font_normal = FontCollection::from_bytes(Vec::from(include_bytes!("VeraMono.ttf") as &[u8]))
        .unwrap().into_font().unwrap();
    let font_bold = FontCollection::from_bytes(Vec::from(include_bytes!("VeraMono-Bold.ttf") as &[u8]))
        .unwrap().into_font().unwrap();
    let font_italic_bold = FontCollection::from_bytes(Vec::from(include_bytes!("VeraMono-Bold-Italic.ttf") as &[u8]))
        .unwrap().into_font().unwrap();
    let scale= 40.0;
    let font_scale = Scale { x: scale, y: scale };

    let color_black = Rgb([0x00, 0x00, 0x00]);
    let text_height = scale;
    let text_x = klasse.get_x() * SCALE + 2.0;
    let mut text_y = klasse.get_y() * SCALE + 2.0;

    let rect_x = klasse.get_x() * SCALE + 0.0;
    let mut rect_y = klasse.get_y() * SCALE + 0.0;
    let rect_width = rect_width(klasse);
    let mut rect_height;

    //Text Zentrierung
    let ch_len0 = klasse.get_id()[0].chars().count() as f32 *(-10.0);
    let ch_len1 = klasse.get_id()[1].chars().count() as f32 *(-10.0);
    let rect_len =(rect_width/2) as f32;
    let d0 =(rect_len +(ch_len0 ))as u32;
    let d1 =(rect_len +(ch_len1 ))as u32;
    //println!("d0:{}",rect_len);
    //println!("d1:{}",d1);


    // Klasse.id
    for i in 0..klasse.get_id().len() {
        //Zeichne Klassentyp  bsp abstract interface etc
        if i ==0 {
            draw_text_mut(image, color_black, text_x as u32 +d1,
                          text_y as u32, font_scale, &font_normal, &klasse.get_id()[1]);
        }else if i == 1{
            let mut font = &font_italic_bold;
            if klasse.get_id()[1].eq("") {
                font = &font_bold;
            }
            draw_text_mut(image, color_black, text_x as u32+d0, text_y as u32, font_scale, &font, &klasse.get_id()[0]);
        }else{
            draw_text_mut(image, color_black, text_x as u32, text_y as u32, font_scale, &font_normal, &klasse.get_id()[i]);
        }
        text_y += text_height;
    }
    rect_height = klasse.get_id().len() as u32 * scale as u32 + (scale / 2.0) as u32;
    draw_hollow_rect_mut(image, Rect::at(rect_x as i32, rect_y as i32).of_size(rect_width, rect_height), color_black);
    text_y += text_height / 2.0;

    // Klasse.attribute
    for i in 0..(klasse._get_atr().len()) {
        draw_text_mut(image, color_black, text_x as u32, text_y as u32, font_scale, &font_normal, &klasse._get_atr()[i]);
        text_y += text_height;
    }
    rect_y += (rect_height - 1) as f32;
    rect_height = klasse._get_atr().len() as u32 * scale as u32 + (scale / 2.0) as u32;
    draw_hollow_rect_mut(image, Rect::at(rect_x as i32, rect_y as i32).of_size(rect_width, rect_height), color_black);
    text_y += text_height / 2.0;

    // Klasse.methoden
    for i in 0..klasse._get_meth().len() {
        draw_text_mut(image, color_black, text_x as u32, text_y as u32, font_scale, &font_normal, &klasse._get_meth()[i]);
        text_y += text_height;
    }
    rect_y += (rect_height - 1) as f32;
    rect_height = klasse._get_meth().len() as u32 * scale as u32 + (scale / 2.0) as u32;
    draw_hollow_rect_mut(image, Rect::at(rect_x as i32, rect_y as i32).of_size(rect_width, rect_height), color_black);
}

fn draw_relation(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let color_white =  Rgb([0xFF, 0xFF, 0xFF]);
    //for i in 0..relation.get_koord().len() - 1 {
       // let mut start = (relation.get_koord()[i].0 as f32 * SCALE, relation.get_koord()[i].1 as f32 * SCALE);
      //  let mut end = (relation.get_koord()[i + 1].0 as f32 * SCALE, relation.get_koord()[i + 1].1 as f32 * SCALE);
    //}
    //match relation.get_typ() {
       // "Komposition" => draw_komp(relation, image),
       // "Aggregation" => draw_aggr(relation, image),
       // "Implementierung" => draw_inher(relation, image),
       // "Vererbung" => draw_inher(relation, image),
      //  _ => draw_einfach(relation, image)
    //}
    draw_einfach(relation,image);
}

fn draw_dashed_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start: (f32, f32), end: (f32, f32), color: Rgb<u8>) {
    let dir = get_dir(start.0, start.1, end.0, end.1);
    let mut start_tmp = start;
    let mut add = (0.0, 0.0);
    let mut end_tmp = start_tmp;
    if dir == Direction::Up {
        add.1 = 10.0;
        end_tmp.1 += 5.0;
    } else if dir == Direction::Right {
        add.0 = 10.0;
        end_tmp.0 += 5.0;
    } else if dir == Direction::Down {
        add.1 = -10.0;
        end_tmp.1 += -5.0;
    } else {
        add.0 = -10.0;
        end_tmp.0 += -5.0;
    }
    while start_tmp < end {
        draw_line_segment_mut(image, start_tmp, end_tmp, color);
        start_tmp.0 += add.0;
        start_tmp.1 += add.1;
        end_tmp.0 += add.0;
        end_tmp.1 += add.1;
    }
}

fn draw_aggr(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let mut start_point = (relation.get_koord()[0].0 as i32 * SCALE as i32, relation.get_koord()[0].1 as i32 * SCALE as i32);
    let point: &[Point<i32>] = &[
        Point::new(start_point.0, start_point.1),
        Point::new(start_point.0 + 10, start_point.1 + 10),
        Point::new(start_point.0, start_point.1 + 20),
        Point::new(start_point.0 - 10, start_point.1 + 10)
    ];
    if get_start_dir(relation) == Direction::Up {
        //start_point = (start_point.0 + 20, start_point.1);
    } else if get_start_dir(relation) == Direction::Right {
        start_point = (start_point.0 - 10, start_point.1 - 10);
    } else if get_start_dir(relation) == Direction::Down {
        start_point = (start_point.0 - 20, start_point.1);
    } else {
        start_point = (start_point.0 + 10, start_point.1 + 10);
    }
    draw_convex_polygon_mut(image, &point, color_black);
}

fn draw_komp(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_white =  Rgb([0xFF, 0xFF, 0xFF]);
    let mut start_point= (relation.get_koord()[0].0 as i32 * SCALE as i32, relation.get_koord()[0].1 as i32 * SCALE as i32);
    let point: &[Point<i32>] = &[
        Point::new(start_point.0, start_point.1),
        Point::new(start_point.0 + 10, start_point.1 + 10),
        Point::new(start_point.0, start_point.1 + 20),
        Point::new(start_point.0 - 10, start_point.1 + 10)
    ];
    if get_start_dir(relation) == Direction::Up {

    } else if get_start_dir(relation) == Direction::Right {
        start_point = (start_point.1 - 8, start_point.1 - 8);
    } else if get_start_dir(relation) == Direction::Down {
        start_point = (start_point.1 - 18, start_point.1);
    } else {
        start_point = (start_point.1 + 8, start_point.1 + 8);
    }
    draw_convex_polygon_mut(image, &point, color_white);
}

fn draw_inher(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let color_white = Rgb([0xFF, 0xFF, 0xFF]);
    let start = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE);
    let mut end1 = (0.0, 0.0);
    let mut end2 = (0.0, 0.0);
    if get_end_dir(relation) == Direction::Up {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE + 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE + 5.0);
        draw_line_segment_mut(image, start, (start.0, start.1 + 4.0), color_white);
    } else if get_end_dir(relation) == Direction::Right {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE + 5.0);
        draw_line_segment_mut(image, start, (start.0 - 4.0, start.1), color_white);
    } else if get_end_dir(relation) == Direction::Down {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE - 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE - 5.0);
        draw_line_segment_mut(image, start, (start.0, start.1 - 4.0), color_white);
    } else {
        end1 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE - 5.0);
        end2 = (relation.get_koord()[relation.get_koord().len() - 1].0 as f32 * SCALE + 5.0, relation.get_koord()[relation.get_koord().len() - 1].1 as f32 * SCALE + 5.0);
        draw_line_segment_mut(image, start, (start.0 + 4.0, start.1), color_white);
    }
    draw_line_segment_mut(image, start, end1, color_black);
    draw_line_segment_mut(image, start, end2, color_black);
    draw_line_segment_mut(image, end1, end2, color_black);

}
fn draw_einfach_neu(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let scale:f32 = 40.0;
    let pf :f32 = 35.0;
    let  x1 = relation.get_x_startknoten().clone()as f32;
    let  y1 = relation.get_y_startknoten().clone()as f32;
    let  x2 = relation.get_x_endknoten().clone()as f32;
    let  y2 = relation.get_y_endknoten().clone()as f32;

}
fn draw_einfach(relation: &Relation, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let color_black = Rgb([0x00, 0x00, 0x00]);
    let scale:f32 = 40.0;
    let pf :f32 = 35.0;
    let  x1 = relation.get_x_startknoten().clone()as f32;
    let  y1 = relation.get_y_startknoten().clone()as f32;
    let  x2 = relation.get_x_endknoten().clone()as f32;
    let  y2 = relation.get_y_endknoten().clone()as f32;
    let (h1,w1) = relation.get_startklasse().get_laenge_breite();
    let (h2,w2) = relation.get_endklasse().get_laenge_breite();
    println!("Draw einfache Relation.........{},{}                {},{}.....",&h1,&w1,&x2,&y2);
    //draw_line_segment_mut(image, (x1,y1), (x2,y2), color_black);
    let dx =(x1-x2).abs();
    let dy =(y1-y2).abs();
    let height1 = (h1 as f32 +1.5)*scale -2.0;
    let height2 = (h2 as f32 +1.5)*scale -2.0;
    let widht1 = rect_width(&relation.get_startklasse()) as f32;
    let widht2 = rect_width(&relation.get_endklasse())as f32;
    let t:f32=0.5;
    println!("Draw einfache Relation.........{},{}                {},{}.....",&height1,&height2,&x2,&y2);
    //Start ist oben rechts von Endknoten
    if y1+height1<y2 {
        let ax = x1 + widht1 *0.5;
        let ay = y1 + height1;
        let a = (ax, ay);
        let dif_y = (ay - y2).abs();
        let bx = ax;
        let by = ay + dif_y * t;
        let b = (bx, by);
        let cx = x2 + widht2 * 0.5;
        let cy = by;
        let c = (cx, cy);
        let dx = cx;
        let dy = y2;
        let d = (dx, dy);
        let dif_x = (x1 + widht1 - x2 + widht2).abs();

        draw_line_segment_mut(image, a, b, color_black);
        draw_line_segment_mut(image, b, c, color_black);
        draw_line_segment_mut(image, c, d, color_black);

        draw_line_segment_mut(image, (dx+pf,dy-pf), d, color_black);
        draw_line_segment_mut(image, (dx-pf,dy-pf), d, color_black);

    }else if y2+height2<y1{
        let ax =x2+widht2*0.5;
        let ay =y2+height2; let a =(ax,ay);
        let dif_y =(ay-y1).abs();
        let bx =ax; let by=ay+dif_y*t; let b=(bx,by);
        let cx =x1 +widht1*0.5; let cy=by; let c =(cx,cy);
        let dx =cx; let dy =y1; let d =(dx,dy);
        let dif_x =(x1+widht1-x2+widht2).abs();

        draw_line_segment_mut(image, a, b, color_black);
        draw_line_segment_mut(image, b, c, color_black);
        draw_line_segment_mut(image, c, d, color_black);

        draw_line_segment_mut(image, (ax+pf,ay+pf), a, color_black);
        draw_line_segment_mut(image, (ax-pf,ay+pf), a, color_black);
    }else if x1==x2&&y1>y2{
        let ax =x1+widht1*0.5;
        let ay =y1+height1; let a =(ax,ay);
        let bx =x1; let by=y2; let b=(bx,by);
        draw_line_segment_mut(image, a, b, color_black);
    }else if x1==x2&&y2>y1 {
        let ax = x2 + widht2 * 0.5;
        let ay = y2 + height2;
        let a = (ax, ay);
        let bx = x1;
        let by = y1;
        let b = (bx, by);
        draw_line_segment_mut(image, a, b, color_black);

    }else if y1<=y2 && y1+height1>=y2  && x1<x2{
        let ax =x1+widht1;
        let ay =y1+height1*0.5; let a =(ax,ay);
        let dif_x =(ax-x2).abs();
        let bx =ax+dif_x*t; let by=ay; let b=(bx,by);
        let cx =bx; let cy=y2+height2*0.5; let c =(cx,cy);
        let dx =x2; let dy =cy; let d =(dx,dy);


        draw_line_segment_mut(image, a, b, color_black);
        draw_line_segment_mut(image, b, c, color_black);
        draw_line_segment_mut(image, c, d, color_black);

        draw_line_segment_mut(image, (dx-pf,dy-pf), d, color_black);
        draw_line_segment_mut(image, (dx-pf,dy+pf), d, color_black);
    }
        else if y2<=y1 && y2+height2>=y1  && x2<x1{
            let ax =x2+widht2;
            let ay =y2+height2*0.5; let a =(ax,ay);
            let dif_x =(ax-x1).abs();
            let bx =ax+dif_x*t; let by=ay; let b=(bx,by);
            let cx =bx; let cy=y1+height1*0.5; let c =(cx,cy);
            let dx =x1; let dy =cy; let d =(dx,dy);


            draw_line_segment_mut(image, a, b, color_black);
            draw_line_segment_mut(image, b, c, color_black);
            draw_line_segment_mut(image, c, d, color_black);

            draw_line_segment_mut(image, (dx-pf,dy-pf), d, color_black);
            draw_line_segment_mut(image, (dx-pf,dy+pf), d, color_black);
        }else if y2>=y1 && y2+height2>=y1  && x2<x1{
            let ax =x2+widht2;
            let ay =y2+height2*0.5; let a =(ax,ay);
            let dif_x =(ax-x1).abs();
            let bx =ax+dif_x*t; let by=ay; let b=(bx,by);
            let cx =bx; let cy=y1+height1*0.5; let c =(cx,cy);
            let dx =x1; let dy =cy; let d =(dx,dy);


            draw_line_segment_mut(image, a, b, color_black);
            draw_line_segment_mut(image, b, c, color_black);
            draw_line_segment_mut(image, c, d, color_black);

            draw_line_segment_mut(image, (ax+pf,ay-pf), a, color_black);
            draw_line_segment_mut(image, (ax+pf,ay+pf), a, color_black);

        }else if y2<=y1 && y2+height2>=y1  && x2>x1 {
            let ax = x1 + widht1;
            let ay = y1 + height1 * 0.5;
            let a = (ax, ay);
            let dif_x = (ax - x2).abs();
            let bx = ax + dif_x * t;
            let by = ay;
            let b = (bx, by);
            let cx = bx;
            let cy = y2 + height2 * 0.5;
            let c = (cx, cy);
            let dx = x2;
            let dy = cy;
            let d = (dx, dy);


            draw_line_segment_mut(image, a, b, color_black);
            draw_line_segment_mut(image, b, c, color_black);
            draw_line_segment_mut(image, c, d, color_black);

            draw_line_segment_mut(image, (dx-pf,dy-pf), d, color_black);
            draw_line_segment_mut(image, (dx-pf,dy+pf), d, color_black);

        }

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
    return (max_width * 25) as u32;
}