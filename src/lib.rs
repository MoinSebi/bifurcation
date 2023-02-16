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
pub fn bifurcation_analysis_meta(shared_index: & Vec<[u32; 3]>) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index: Vec<&[u32; 3]> = Vec::new();

    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());



    // Only close bubble if both is bigger
    for index_tuple in shared_index.iter(){
        // Dummy list what "open" bubble to remove

        // Trigger if the same entry is already there - we always index to open_index
        let mut trigger = true;

        // Index to delete entries on the fly
        let mut index = 0;
        while index < open_index.len(){
            let start = open_index[index];
            // If the next entry is just increasing by 1 in both cases --> remove and update new entry
            if  (start[0] + 1 == index_tuple[0]) && start[1] + 1 == index_tuple[1] {
                open_index.remove(index);

                // If one index is same - nothing happens
            } else if (&index_tuple[0] > &start[0]) & (&index_tuple[1] > &start[1]){

                bubble.push((min(index_tuple[2], start[2]), max(index_tuple[2], start[2])));
                open_index.remove(index);}

                // If one index is same - nothing happens (don't add to new open bubbles?)
            else if (start[0] == index_tuple[0]) | (start[1] == index_tuple[1]) {
                trigger = false;
                index  += 1;
                // This happens when the second entry is smaller for example
            } else {
                index +=1;
            }

        }
        // This is only relevant for the first entry
        if trigger{
            open_index.push(index_tuple);
        }

    }
    bubble
}




#[cfg(test)]
mod tests {
    use log::info;
    use crate::{sort_array_vec, bifurcation_analysis_meta, is_sorted};

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
        let mut vec2 = vec1.clone();
        vec1.sort();
        assert_eq!(true, is_sorted(&vec1));
        assert_eq!(false, is_sorted(&vec2));
    }

    #[test]
    /// Check a simple example
    fn test_simple(){
        let mut vec = vec![[1, 2,3], [4, 5,4], [3, 4,5], [3, 3,6]];
        vec.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let g = bifurcation_analysis_meta(&vec);
        assert_eq!(vec![(3,6), (4,6)], g);
    }


}

