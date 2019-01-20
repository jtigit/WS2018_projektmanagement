extern crate regex;

use self::regex::Regex;
use crate::parser::parser;
use crate::parser::relation::Relation;
use crate::parser::relation;
use crate::layout::graphbuilder;
use crate::image_gen::image_gen;
use image::{Rgb, RgbImage, ImageBuffer};
use crate::parser::useCaseDiagramm::usecase::Usecase;


pub struct Usecasediagramm{
    pub usecases: Vec<Usecase>,
    pub actors: Vec<super::actor::Actor>,
    pub boundarys: Vec<super::boundary::Boundary>,
    pub relations: Vec<super::relation::Relation>
}