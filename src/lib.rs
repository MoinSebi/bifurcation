pub mod test;
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
        open_index.retain(|&[start, end, _bub_id]| ((istart <= &start) || (iend <= &end)));




        open_index.push([*istart, *iend, *ibub]);

    }
    bubble
}


pub fn bifurcation_analysis_sort(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index = Vec::new();
    open_index.push(shared_index[0]);
    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());



    // Only close bubble if both is bigger
    for index_tuple in 1..shared_index.len(){
        // Dummy list what "open" bubble to remove

        let [istart, iend, ibub] = shared_index[index_tuple];
        // Trigger if the same entry is already there - we always index to open_index
        // Index to delete entries on the fly

        for x in open_index.iter(){
            let [ostart, oend, obub] = x;
            if (istart > *ostart) && (iend > *oend) {
                if !(ostart + 1 == istart && oend + 1 == iend) {
                    bubble.push((min(ibub, *obub), max(ibub, *obub)));
                }
            }
            if oend > &iend {
                break
            }
        }

        open_index.retain(|[start, end, _bubb]| ((istart <= *start) || (iend <= *end)));
        open_index.push([istart, iend, ibub]);

        open_index.sort_by(|a, b| a[1].cmp(&b[1]));

        // This is only relevant for the first entry




    }
    bubble
}




pub fn bifurcation_analysis_bheap(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index = Vec::new();
    open_index.push(shared_index[0]);
    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());



    // Only close bubble if both is bigger
    for index_tuple in 1..shared_index.len(){
        // Dummy list what "open" bubble to remove

        let [istart, iend, ibub] = shared_index[index_tuple];
        // Trigger if the same entry is already there - we always index to open_index
        // Index to delete entries on the fly

        for x in open_index.iter() {
            let [ostart, oend, obub] = x;
            if (istart > *ostart) && (iend > *oend) {
                if !(ostart + 1 == istart && oend + 1 == iend) {
                    bubble.push((min(ibub, *obub), max(ibub, *obub)));
                }
            }

            if oend > &iend {
                break
            }
        }

        open_index.retain(|[start, end, _bubb]| ((istart <= *start) || (iend <= *end)));
        open_index.push([istart, iend, ibub]);

        if open_index.len() != 0{
            open_index.sort_by(|a, b| a[1].cmp(&b[1]));
        }

        // This is only relevant for the first entry




    }
    bubble
}



