
//Starte UmL Parser

use crate::umlparser::klassendiagramm::klasse;

    pub fn starte_umlparser(input: &String) {
        parse_klassendiagramm(&input);
        parse_aktivitaetendiagramm(&input);
    }

    pub fn parse_klassendiagramm(input: &String) {
        klasse::baue_klassen(input);
    }

    pub fn parse_aktivitaetendiagramm(input: &String) {}
