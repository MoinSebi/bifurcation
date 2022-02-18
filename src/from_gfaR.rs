use gfaR_wrapper::{NGfa, NPath};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use crate::helper::{chunk_inplace, get_all_pairs};
use std::thread;
use crate::bifurcation_analysis;
use std::panic::resume_unwind;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]

pub struct DirNode{
    pub dir: bool,
    pub id: u32,
}

impl DirNode{
    pub fn new(dir: bool, id: u32)-> Self{
        Self{
            dir: dir,
            id: id,
        }
    }
}


pub fn iterate_test(graph: &NGfa, threads: usize) -> Vec<(usize, usize)>{
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

    let mut result_final: Vec<(usize, usize)> = Vec::new();
    let ro = a.lock().unwrap();
    for x in ro.iter(){
        for y in x.iter(){
            result_final.push(y.clone());
        }
    }
    return result_final
}

pub fn path2hashset_dirnode(path: &NPath) -> HashSet<DirNode>{
    let hs_dirnode: HashSet<DirNode> = HashSet::from_iter(path.nodes.iter().cloned().zip(path.dir.iter().cloned()).map(|x| {DirNode::new(x.1, x.0)}));
    return hs_dirnode
}

pub fn path2vec_dirnode(path: &NPath) -> Vec<DirNode>{
    let iter1: Vec<DirNode> = Vec::from_iter(path.nodes.iter().cloned().zip(path.dir.iter().cloned()).map(|x| {DirNode::new(x.1, x.0)}));
    iter1
}

pub fn vec2hashmap(vec: &Vec<DirNode>, intersection: &HashSet<DirNode>) -> HashMap<DirNode, Vec<usize>>{
    let mut node2pos: HashMap<DirNode, Vec<usize>> = HashMap::new();
    for (index, dir_node) in vec.iter().enumerate(){
        if intersection.contains(dir_node){
            if node2pos.contains_key(&dir_node){
                node2pos.get_mut(&dir_node).unwrap().push(index);
            } else {
                node2pos.insert(dir_node.clone(), vec![index.clone()]);
            }
        }
    }
    node2pos
}




pub fn get_shared_index(path1: &NPath, path2: &NPath) -> Vec<(usize, usize)> {
    let iter1 = path2hashset_dirnode(path1);
    let iter2 = path2hashset_dirnode(path2);

    let g: HashSet<DirNode> = iter1.intersection(&iter2).cloned().collect();


    let iterr1 = path2vec_dirnode(path1);
    let iterr2 = path2vec_dirnode(path2);

    let mut node2pos: HashMap<DirNode, Vec<usize>> = vec2hashmap(&iterr1, &g);
    let mut node2pos2: HashMap<DirNode, Vec<usize>> = vec2hashmap(&iterr2, &g);

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
    use crate::from_gfaR::{all_combinations, get_shared_index, iterate_test};
    use gfaR_wrapper::NGfa;

    #[test]
    fn test_combinations() {
        let vec = vec![1,2,3];
        let vec2 = vec![30,40];
        let j = all_combinations(&vec, &vec2);
        assert_eq!(vec![(1,30), (1,40), (2,30), (2,40), (3,30), (3,40)], j);
    }

    #[test]
    fn shared_index(){
        let mut graph: NGfa = NGfa::new();

        graph.from_graph("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
        let g = iterate_test(&graph, 1);
        eprintln!("{:?}", g.len());
    }
}