pub mod helper;
extern crate log;

use std::cmp::{max, min};
use log::{debug};


/// **Inplace sorting of a vector of arrays>**
/// Sort by first entry, then by second
///
/// Comment: I think this is also happening when sorting normal
pub fn sort_array_vec(vector: & mut Vec<[u32; 3]>){
    debug!("Sort the tuple");
    vector.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
}


/// Check if a vector is sorted
pub fn is_sorted(vector: & Vec<[u32; 3]>) -> bool{
    for x in 1..vector.len(){
        if ! (((vector[x-1] == vector[x]) && (vector[x-1] <= vector[x])) ||  (vector[x-1] < vector[x]) ){
            return false
        }
    }

    return true
}




/// Detect bubbles but only return the start and the end nodes
///
/// Input:
/// - shared index: [path1_index, path2_index, node]
///
/// Return:
/// - Vector of shared index
/// Return vector of (node1, node2) that span a bubble
///
///
/// Example:
///  let mut vec = vec![[1, 2,3], [4, 5,4], [3, 4,5], [3, 3,6]];
///  vec.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
///  let g = bifurcation_analysis_meta(&vec);
///
pub fn bifurcation_analysis_meta(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");
    if shared_index.len() < 2{
        return Vec::new()
    }

    // Mutating vector of starting point of bubbles
    let mut open_index = vec![shared_index[0]];

    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());

    // Only close bubble if both is bigger
    for [istart, iend, ibub] in shared_index.iter().skip(1){

        for [ostart, oend, obub] in open_index.iter(){

            if ((istart > ostart) && (iend > oend)) && !((ostart + 1 == *istart) && (oend + 1 == *iend)){
                bubble.push((*min(ibub,  obub), *max(ibub, obub)));
            }
        }

        // I want those which are bigger than the new one
        open_index.retain(|&[start, end, _bub_id]| istart <= &start || iend<= &end);



        open_index.push([*istart, *iend, *ibub]);

    }
    bubble
}




#[cfg(test)]
mod tests {
    use crate::{sort_array_vec, bifurcation_analysis_meta, is_sorted};

    #[allow(dead_code)]
    pub fn data_creation() -> Vec<[u32; 3]> {
        let mut mm = Vec::new();
        for x in 0..50{
            mm.push([x, 1000000-x, 1]);
        }

        for x in 100..6000000{
            if x%20 == 0{
                mm.push([x,x+500,10])
            } else if x%5 == 0{
                mm.push([x+3, x+10000,10])
            } else {
                mm.push([x+1, x+1,10])

            }
        }
        mm.sort();
        mm
    }
    #[test]
    /// General test the sorting
    fn test_check_sorting(){
        let mut vec1 = vec![[1,2,3], [1,3,4], [1,2,4], [2,10,11], [3,20, 13]];
        let mut vec2 = vec1.clone();
        vec1.sort();
        sort_array_vec(&mut vec2);
        assert_eq!(vec1, vec2);
        assert_eq!(vec1, vec![[1,2,3], [1,2,4], [1,3,4], [2,10,11], [3,20, 13]])
    }

    #[test]
    /// Check "is_sorted" function.
    fn test_is_sorted(){
        let mut vec1 = vec![[1,2,3], [1,3,4], [1,2,4], [2,10,11], [3,20, 13]];
        let vec2 = vec1.clone();
        vec1.sort();
        assert_eq!(true, is_sorted(&vec1));
        assert_eq!(false, is_sorted(&vec2));
    }

    #[test]
    /// Check a simple example
    fn test_simple(){
        let mut vec = vec![[1, 2,3], [4, 5,4], [3, 4,5], [3, 3,6], [1,10,19]];
        //let mut vec = data_creation();
        println!("{}", vec.len());
        vec.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let g = bifurcation_analysis_meta(&vec[..]);
        assert_eq!(vec![(3,6), (4,6)], g);
    }


}

