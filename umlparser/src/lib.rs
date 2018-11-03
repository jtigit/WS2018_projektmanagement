extern crate regex;

use std::fs;
use std::error::Error;
use regex::Regex;


// Diese Funktion nimmt ein Query entgegen und übrprüft ob es im Content enthalten ist
// Return: alle Zeilen ,wo es zutrifft werden in einem Vektor gespeichert und zrückgegeben
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   let mut results = Vec::new();
   
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
//Diese Methode sucht nach query im Text und gibt die Indizes aller Funde als Vektor zurück
pub fn find_indices<'a>(query: &str, contents: &'a str) -> Vec<usize> {
	let mut results = Vec::new();
	let v: Vec<_> = contents.match_indices(query).collect();
	for &word in &v {
		let (k,b) = word;
		results.push(k);
		println!("{:?}", k);
   }
   results
}

 pub fn baue_klassen<'a>(contents: &'a str){
	baue_klassen_namen(&contents);

} 

pub fn baue_klassen_namen<'a>(contents: &'a str){

	
	let re = Regex::new(r"Klasse\{[^\}]+typ:(?P<typ>[[\w]&&[\S]]+)[^\}]+name:(?P<title>[[\w]&&[\S]]+)[^\}]+impl:(?P<implements>[[\s]?[\w]+[\s]?[,]?]+)[\s]?;[\s]?extends:(?P<extends>[[\s]?[\w]+[\s]?[,]?]+[^\n])[^\}]+\}")
               .unwrap();
	let snipets = re.captures(&contents).unwrap();
	for caps in re.captures_iter(&contents) {
		let name = caps.get(1).unwrap().as_str();
		let typ = caps.get(2).unwrap().as_str();
		let imp= caps.get(3).unwrap().as_str();
		let ext = caps.get(4).unwrap().as_str();

		println!("{}",name);
		println!("{}",typ);
		println!("{}",imp);
		println!("{}",ext);
		println!("Klassename: {:?}, Klassentyp: {:?},
		Interfaces: {:?}, Superklassen: {:?}",
		&caps["title"],&caps["typ"],&caps["implements"],&caps["extends"]);
	}

} 

pub struct Klasse<'a> {
	pub name : &'a str,
	pub typ : &'a str,
	pub atribute : &'a Vec<&'a str>,
	pub methoden : &'a Vec<&'a str>,
	pub interfaces : &'a Vec<&'a str>,
	pub oberklassen : &'a Vec<&'a str>
}
impl<'a> Klasse<'a>{
	fn print_all(&self){
		println!("Klasse.printAll methode {}", self.methoden.get(0).unwrap());
	}


}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
//Schnittstelle nach außen, diese funktion nimmt die Konfiguration entgegen und ruft search auf
// als ausgabe printed sie das ergebniss von search
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
	
	baue_klassen(&contents);
	for line in search(&config.query, &contents) {
        println!("{}", line);
    }
	
	
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}


//Tests-------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}