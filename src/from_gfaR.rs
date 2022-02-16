use gfaR_wrapper::{NGfa, NPath};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use crate::helper::{chunk_inplace, get_all_pairs};
use std::thread;
use crate::bifurcation_analysis;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]

pub struct dir_node{
    pub dir: bool,
    pub id: u32,
}

impl dir_node{
    pub fn new(dir: bool, id: u32)-> Self{
        Self{
            dir: dir,
            id: id,
        }
    }
}


pub fn iterate_test(graph: &NGfa, threads: usize) {
    // Get pairs and
    let pairs = get_all_pairs(&graph.paths);
    let chunks = chunk_inplace(pairs, threads);

    // Resultat
    let mut result = Vec::new();
    let mut handles = Vec::new();
    let mut a = Arc::new(Mutex::new(result));

    // Iterate over chunks
    for chunk in chunks{
        let j = a.clone();
        let handle = thread::spawn(move || {
            for pair in chunk.iter(){
                eprintln!("Working on this pair: {} {}", pair.0.name, pair.1.name);
                let h = get_shared_index(&pair.0, &pair.1);
                let result = bifurcation_analysis(&h);
                let mut rr = j.lock().unwrap();
                rr.push(result);

            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()

    }
}





pub fn get_shared_index(path1: &NPath, path2: &NPath) -> Vec<(usize, usize)> {
    let iter: HashSet<dir_node> = HashSet::from_iter(path1.nodes.iter().cloned().zip(path1.dir.iter().cloned()).map(|x| {dir_node::new(x.1, x.0)}));
    let iter2: HashSet<dir_node> = HashSet::from_iter(path2.nodes.iter().cloned().zip(path2.dir.iter().cloned()).map(|x| {dir_node::new(x.1, x.0)}));

    let g: HashSet<dir_node> = iter.intersection(&iter2).cloned().collect();


    let iterr1: Vec<dir_node> = Vec::from_iter(path1.nodes.iter().cloned().zip(path1.dir.iter().cloned()).map(|x| {dir_node::new(x.1, x.0)}));
    let iterr2: Vec<dir_node> = Vec::from_iter(path1.nodes.iter().cloned().zip(path1.dir.iter().cloned()).map(|x| {dir_node::new(x.1, x.0)}));

    let mut node2pos: HashMap<dir_node, Vec<usize>> = HashMap::new();
    for (index, x) in iterr1.iter().enumerate(){
        if g.contains(x){
            if node2pos.contains_key(&x){
                node2pos.get_mut(&x).unwrap().push(index);
            } else {
                node2pos.insert(x.clone() ,vec![index.clone()]);
            }
        }
    }
    let mut node2pos2: HashMap<dir_node, Vec<usize>> = HashMap::new();

    for (index, x) in iterr2.iter().enumerate(){
        if g.contains(x){
            if node2pos.contains_key(&x){
                node2pos.get_mut(&x).unwrap().push(index);
            } else {
                node2pos.insert(x.clone() ,vec![index.clone()]);
            }
        }
    }

    let mut o = Vec::new();
    for x in g.iter(){
        let k = node2pos.get(x).unwrap().clone();
        let k2 = node2pos2.get_mut(x).unwrap().clone();
        if (k.len() > 1) | (k2.len() > 1){
            o.extend(all_combinations(&k, &k2))
        } else {
            o.push((k[0], k2[0]));
        }
    }
    o
}


///
pub fn all_combinations(a: & Vec<usize>, b: & Vec<usize>) -> Vec<(usize,usize)> {
    let mut p = Vec::new();
    for x in a.iter(){
        for y in b.iter(){
            p.push((x.clone(),y.clone()))
        }
    }
    p
}


#[cfg(test)]
mod form_gfaR {
    use crate::{sort_tuple_vector, bifurcation_analysis};
    use crate::from_gfaR::{all_combinations, get_shared_index};
    use gfaR_wrapper::NGfa;

    #[test]
    fn test_combinations() {
        let mut vec = vec![1,2,3];
        let mut vec2 = vec![30,40];
        let j = all_combinations(&vec, &vec2);
        assert_eq!(vec![(1,30), (1,40), (2,30), (2,40), (3,30), (3,40)], j);
    }

    #[test]
    fn shared_index(){
        let mut graph: NGfa = NGfa::new();

        graph.from_graph("test.gfa");

    }
}