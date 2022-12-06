use gfaR_wrapper::{NGfa, NPath};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use crate::helper::{chunk_inplace, get_all_pairs};
use std::thread;
use log::{debug, info};
use crate::{bifurcation_analysis, sort_tuple_vector};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]


/// **Directed nodes**
/// Holds identifier and direction
/// This is 2 bytes
pub struct DirNode{
    pub dir: bool,
    pub id: u32,
}

impl DirNode{
    /// Constructor
    pub fn new(dir: bool, id: u32)-> Self{
        Self{
            dir: dir,
            id: id,
        }
    }
}

/// **Wrapper function for genome graphs**
///
/// TODO:
/// - Change the multithreading
pub fn iterate_test(graph: &NGfa, threads: usize) -> Vec<((String, String),  (HashMap<(usize, usize), Vec<(usize, usize)>>, Option<(usize, usize)>))>{


    // Get all pairs of paths - (n*n-1)/2
    let pairs = get_all_pairs(&graph.paths);
    info!("Number of pairs: {}", pairs.len());
    // Chunk the pairs
    let chunks = chunk_inplace(pairs, threads);

    // Results
    let result = Vec::new();
    let result_arc = Arc::new(Mutex::new(result));

    // Handles
    let mut handles = Vec::new();

    // Iterate over chunks
    for chunk in chunks{
        let j = result_arc.clone();
        let handle = thread::spawn(move || {
            for pair in chunk.iter(){
                debug!("Pair: {} {}", pair.0.name, pair.1.name);

                // Get the shared index
                let mut shared_index = get_shared_index(&pair.0, &pair.1, true);
                let result = bifurcation_analysis(&shared_index);
                let mut rr = j.lock().unwrap();
                rr.push(((pair.0.name.clone(), pair.1.name.clone()), result));

            }
        });
        handles.push(handle);
    }

    // Wait for handles
    for handle in handles {
        handle.join().unwrap()

    }

    let mut result_final = Vec::new();
    let ro = result_arc.lock().unwrap();
    for x in ro.iter(){
        if x.1.1.is_some(){
            result_final.push(x.clone());
        }
    }
    return result_final
}

/// Creates HashSet of DirNode from Path
pub fn path2hashset_dirnode(path: &NPath) -> HashSet<DirNode>{
    let hs_dirnode: HashSet<DirNode> = HashSet::from_iter(path.nodes.iter().cloned().zip(path.dir.iter().cloned()).map(|x| {DirNode::new(x.1, x.0)}));
    hs_dirnode
}

/// Creates Vector of DirNode from Path
pub fn path2vec_dirnode(path: &NPath) -> Vec<DirNode>{
    let vec_dirnode: Vec<DirNode> = Vec::from_iter(path.nodes.iter().cloned().zip(path.dir.iter().cloned()).map(|x| {DirNode::new(x.1, x.0)}));
    vec_dirnode
}

/// **Convert vector to HashMap - {DirNode -> [index]}**
///
///
/// Iterate over the path
/// Check if node is contained in the "shared" index
///
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



/// Get all positions [x1, x2] of the same shared nodes
pub fn get_shared_index(path1: &NPath, path2: &NPath, sort: bool) -> Vec<(usize, usize)> {
    // Get all directed nodes found in both paths
    let iter1 = path2hashset_dirnode(path1);
    let iter2 = path2hashset_dirnode(path2);

    let shared_nodes: HashSet<DirNode> = iter1.intersection(&iter2).cloned().collect();


    // Makes path if dir_nodes
    let iterr1 = path2vec_dirnode(path1);
    let iterr2 = path2vec_dirnode(path2);
    // ---- This upper part can be static (or precomputed somehow)

    // {DirNode -> [index]}
    let node2pos: HashMap<DirNode, Vec<usize>> = vec2hashmap(&iterr1, &shared_nodes);
    let node2pos2: HashMap<DirNode, Vec<usize>> = vec2hashmap(&iterr2, &shared_nodes);

    // Iterate over shared nodes
    // --> Get all index from this node
    // --> Add all possible combinations to the vector
    let mut result = Vec::new();
    for x in shared_nodes.iter(){
        let k = node2pos.get(x).unwrap().clone();
        let k2 = node2pos2.get(x).unwrap().clone();
        if (k.len() > 1) | (k2.len() > 1){
            result.extend(all_combinations(&k, &k2))
        } else {
            result.push((k[0], k2[0]));
        }
    }
    // Sort it afterwards
    if sort{
        sort_tuple_vector(&mut result)
    }
    result
}


/// **Get all combinations of two vectors**
///
pub fn all_combinations<T>(a: & Vec<T>, b: & Vec<T>) -> Vec<(T,T)>
    where T: Clone{
    {
        let mut p = Vec::new();
        for x in a.iter(){
            for y in b.iter(){
                p.push((x.clone(),y.clone()))
            }
        }
        p
    }
}


#[cfg(test)]
mod form_gfaR {
    // cargo test -- --nocapture --test-threads=1
    use crate::from_gfaR::{all_combinations, iterate_test};
    use gfaR_wrapper::NGfa;
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_combinations() {
        init();
        info!("Testing combinations");
        let vec = vec![1,2,3];
        let vec2 = vec![30,40];
        let j = all_combinations(&vec, &vec2);
        assert_eq!(vec![(1,30), (1,40), (2,30), (2,40), (3,30), (3,40)], j);
    }


    fn shared_index(){
        info!("Testing shared_index function");
        let mut graph: NGfa = NGfa::new();

        graph.from_graph("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa");
        graph.from_graph("/home/svorbrugg_local/Rust/data/chr1.wfmash.n20.a90.s10000.p1,19,39,3,81,1.seqwish.sort.smooth.sort.noC.gfa"); 
        let g = iterate_test(&graph, 1);
        for x in g.iter(){
            if (x.0.0 == "a_Chr1".to_owned()) & (x.0.1 == "b_Chr".to_owned()){
                assert_eq!(x.1.0.contains_key(&(2,2)), true);

            }
        }
    }
}