
use crate ::parser::object::Object;

pub struct Actor{
    figur: Object,
    art: Object,
}

impl Actor{
    pub fn new(bezeichnung:String)->Actor{
        let figur = Object::new();
        let art = Object::new1(bezeichnung);
        Actor{figur:figur,art:art}
    }
}