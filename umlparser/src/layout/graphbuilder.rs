use crate::parser::klassendiagramm::klassendiagramm::Klasse;
use crate::layout::graph::Graph;
use crate::parser::relation::Relation;
use crate::layout::p2d::P2d;
use crate::layout::fruchterman_reingold;

use rand::Closed01;
use rand::random;


pub fn create_graph(klassen: &mut Vec<Klasse>, relationen: &mut Vec<Relation>) {
    if klassen.len() > 0 {
        let mut graph = Graph::new();
        for klasse in klassen.iter_mut() {
            klasse.set_pk(graph.add_node());
        }
        for relation in relationen.iter_mut() {
            let s = relation.get_name_startknoten();
            let e = relation.get_name_endknoten();
            let mut n1: usize = 0;
            let mut n2: usize = 0;
            for klasse in klassen.iter_mut() {
                if klasse.get_id().first().unwrap() == &s.to_string() {
                    n1 = *klasse.get_pk();
                }
                if klasse.get_id().first().unwrap() == &e.to_string() {
                    n2 = *klasse.get_pk();
                }
            }
            graph.add_edge((n1, n2));
        }
        println!("knoten {}   edges {}", graph.node_count(), graph.edge_count());

        layout_graph(klassen, graph, None);
        //Kolisionsbehandlung
        let mut klassen2 = klassen.clone();
        for klasse_a in klassen.iter_mut() {
            for klasse_b in klassen2.iter_mut() {

                let scale = 40.0;
                let ax = klasse_a.get_pos_x().clone();
                let ay = klasse_a.get_pos_y().clone();

                let bx = klasse_b.get_pos_x().clone();
                let by = klasse_b.get_pos_y().clone();
                let (h1,w1) = klasse_a.get_laenge_breite();
                let (h2,w2) = klasse_b.get_laenge_breite();
                let widht1 = w1 as f32 * 25.0;
                let widht2 = w2 as f32 * 25.0;
                let height1 = (h1 as f32 +1.5)*scale -2.0;
                let height2 = (h2 as f32 +1.5)*scale -2.0;
                let r:f32 = 50.0;//rahmengröße
                if ax!=bx && ay != by { //Vergleiche nur nicht gleiche Klassen
                    //Wenn es mit Rahmen kolidiert dann..
                    //Obere linke ecke
                    if bx>=ax-r && bx<= ax+widht1+r && by>=ay-r && by<= ay+height1+r {
                        klasse_a.set_pos_y(ay-(ay+height1+r-by).abs());
                        //Obere rechte ecke
                    } else if bx+widht2>=ax-r && bx +widht2<= ax+widht1+r && by>=ay-r && by<= ay+height1+r{
                        klasse_a.set_pos_y(ay-(ay+height1+r-by).abs());
                        //untere linke ecke
                    }else if bx>=ax-r && bx<= ax+widht1+r && by+height2>=ay-r && by+height2<= ay+height1+r{
                        klasse_a.set_pos_y(ay-(ay+height1+r-by).abs());
                        //untere rechte ecke
                    }else if bx+widht2>=ax-r && bx+widht2<= ax+widht1+r && by+height2>=ay-r && by+height2<= ay+height1+r{
                        klasse_a.set_pos_y(ay-(ay+height1+r-by).abs());
                    }  
                }

            }
        }

        for relation in relationen.iter_mut() {
            let s = relation.get_name_startknoten().clone();
            let e = relation.get_name_endknoten().clone();

            for klasse in klassen.iter_mut() {
                if klasse.get_id().first().unwrap() == &s.to_string() {
                    let x = *klasse.get_pos_x()as u32;
                    let y =*klasse.get_pos_y()as u32;
                    relation.set_startknoten(x,y);
                }
                else if klasse.get_id().first().unwrap() == &e.to_string() {
                    let x = *klasse.get_pos_x()as u32;
                    let y =*klasse.get_pos_y()as u32;
                    relation.set_endknoten(x,y);
                }
            }
        }
    }
}

fn layout_graph(klassen: &mut Vec<Klasse>, g: Graph, l: Option<f32>) {
    let mut node_positions: Vec<P2d> = g.nodes
        .iter()
        .map(|_| {
            P2d(random::<Closed01<f32>>().0,
                random::<Closed01<f32>>().0)
        })
        .collect();
    let mut node_neighbors: Vec<Vec<usize>> = g.nodes.iter().map(|_| Vec::new()).collect();
    for &(src, dst) in g.edges.iter() {
        node_neighbors[src].push(dst);
    }
    fruchterman_reingold::layout_typical_2d(l, &mut node_positions, &node_neighbors, 0);
    for (index, pos1) in node_positions.iter().enumerate() {
        println!("Positionen test:  Koordinaten({}/{})",pos1.0,pos1.1);
        for klasse in klassen.iter_mut() {
            if *klasse.get_pk() == index {
                klasse.set_pos_x(pos1.0 );
                klasse.set_pos_y(pos1.1 );
                println!("Klassenname:{:?}  Koordinaten({}/{})",klasse.get_id().first(),klasse.get_pos_x(),klasse.get_pos_y());
                break;
            }
        }
    }
}