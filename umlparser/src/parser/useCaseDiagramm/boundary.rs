use crate ::parser::object::Object;
use crate ::parser::useCaseDiagramm::usecase::Usecase;

pub struct Boundary{
    pub figur: Object,
    pub text: Object,
    pub usecaseliste:Vec<Usecase>,
}

impl Boundary{
    pub fn new(bezeichnung:String)->Boundary{
        let figur:Object = Object::new();
        let text:Object = Object::new1(bezeichnung);
        let usecaseliste = vec![];
        Boundary{figur,text,usecaseliste}
    }
}