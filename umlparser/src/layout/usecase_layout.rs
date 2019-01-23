use crate::parser::useCaseDiagramm::usecasediagramm::Usecasediagramm;
use crate::parser::useCaseDiagramm::usecase::Usecase;
use crate::parser::useCaseDiagramm::actor::Actor;
use crate::parser::useCaseDiagramm::boundary::Boundary;
use crate::parser::useCaseDiagramm::relation::Relation;




pub fn create_layout(usecases: &mut Vec<Usecase>, relationen: &mut Vec<Relation>,boundarys:&mut Vec<Boundary>,
actors:&mut Vec<Actor>) {

    for boundary in boundarys.iter_mut(){
        boundary.figur.x = 500.0;
        boundary.figur.y = 100.0;
        boundary.figur.w = 1600.0;
        boundary.figur.h = 1600.0;
        boundary.text.x= 550.0;
        boundary.text.y= 150.0;
    }

}