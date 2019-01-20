use crate ::parser::object::Object;

pub struct Usecase{
    pub figur: Object,
    pub text: Object,
    pub extension: bool,
    pub extension_text: Object,
}

impl Usecase{
    //konstruktor für einfachen usecase
    pub fn new(bezeichnung:String)->Usecase{
        let figur = Object::new();
        let extension_text = Object::new();
        let text = Object::new1(bezeichnung);
        Usecase{figur:figur,text:text,extension:false,extension_text}
    }
    //konstruktor für extension usecase
    pub fn new_extension(bezeichnung:String,extension_text:String)->Usecase{
        let figur = Object::new();
        let text = Object::new1(bezeichnung);
        let extension_text = Object::new1(extension_text);
        Usecase{figur:figur,text:text,extension:true,extension_text}
    }
}