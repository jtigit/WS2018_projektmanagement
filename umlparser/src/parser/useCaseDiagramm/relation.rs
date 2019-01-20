use crate ::parser::object::Object;

pub struct Relation{
    pub start: Object,
    pub ende: Object,
    pub text: Object,
}

impl Relation{
    pub fn new()->Relation{
        let start:Object = Object::new();
        let ende:Object = Object::new();
        let text:Object = Object::new();
        Relation{start,ende,text}
    }
    pub fn new_include()->Relation{
        let bezeichnung:String = "<<include>>".to_string();
        let start:Object = Object::new();
        let ende:Object = Object::new();
        let text:Object = Object::new1(bezeichnung);
        Relation{start,ende,text}
    }
    pub fn new_extend()->Relation{
        let bezeichnung:String = "<<extend>>".to_string();
        let start:Object = Object::new();
        let ende:Object = Object::new();
        let text:Object = Object::new1(bezeichnung);
        Relation{start,ende,text}
    }
}