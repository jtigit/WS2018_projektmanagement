use crate ::parser::object::Object;
use crate ::parser::useCaseDiagramm::usecase::Usecase;

pub struct Boundary{
    figur: Object,
    text: Object,
    usecaseliste:Vec<Usecase>,
}

impl Boundary{
    pub fn new(bezeichnung:String)->Boundary{
        let figur:Object = Object::new();
        let text:Object = Object::new();
        let usecaseliste = vec![];
        Boundary{figur,text,usecaseliste}
    }
}