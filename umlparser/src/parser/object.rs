
pub struct Object {
    x : f32,
    y: f32,
    w: f32,
    h: f32,
    name: String,
}


impl Object {
    pub fn new()-> Object {
        Object {x:0.0,y:0.0,w:0.0,h:0.0,name:"".to_string(),}
    }
    pub fn new1(name:String)-> Object {
        Object {x:0.0,y:0.0,w:0.0,h:0.0,name,}
    }
    pub fn new3(x:f32,y:f32,name:String)-> Object {
        Object {x,y,w:0.0,h:0.0,name,}
    }

}
