pub mod helper;
extern crate log;

use core::fmt::Debug;
use std::cmp::{max, min};
use std::ops::{Add, Index, RangeBounds, Rem};
use std::os::unix::io::AsFd;
use std::slice::SliceIndex;
use log::{debug};


/// **Inplace sorting of a vector Vec<(a,b)>**
/// By a then by b
/// Check later of own sorting is faster
pub fn sort_tuple_vector(vector: & mut Vec<[usize; 2]>){
    debug!("Sort the tuple");
    vector.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
}

pub fn bifuraction_analysis_index<T>(o: & Vec<T>) ->  Vec<Vec<[usize; 2]>>
    where T: IntoIterator<Item = usize> + Index<usize, Output = T::Item> + std::fmt::Debug + ,<T as IntoIterator>::Item: Debug{
    bifurcation_analysis(o)
}

/// Detect bubbles
/// Returns a list of tuples which span a bubble
/// These numbers are index from the second genome
///
/// T can be any iterator - mainly this way so it can be used by array and vector at the same time
///
/// Returns the stretching indices
///
///
/// Example:
/// let mut vec = vec![[1, 2], [4, 5], [3, 4], [3, 3]];
/// sort_tuple_vector(&mut vec);
/// let f = bifurcation_analysis(&vec);
pub fn bifurcation_analysis<T>(o: & Vec<T>) ->  Vec<Vec<[usize; 2]>>
    where T: IntoIterator<Item = usize> + Index<usize, Output = T::Item> + std::fmt::Debug + ,<T as IntoIterator>::Item: Debug {
    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index: Vec<&T> = Vec::new();

    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = vec![vec![]; 2];


    // TODO
    // Only close bubble if both is bigger
    for shared_index in o.iter(){
        let g = [shared_index[0], shared_index[1]];
        // Dummy list what "open" bubble to remove
        let mut remove = Vec::new();

        // Trigger if the same entry is already there - we always index to open_index
        let mut trigger = true;
        for (index, start) in open_index.iter().enumerate(){

            // If the next entry is just increasing by 1 in both cases --> remove and update new entry
            if [start[0] as usize +1, start[1] as usize+1] == g {
                remove.push(index);


                // If one index is same - nothing happens
            } else if (start[0] == shared_index[0]) | (start[1] == shared_index[1]){
                trigger = false;
                continue;

                // If both things are bigger -> add bubble
            } else if (&shared_index[0] > &start[0]) & (&shared_index[1] > &start[1]){
                bubble[0].push([start[0], shared_index[0]]);
                bubble[1].push([start[1], shared_index[1]]);
                remove.push(index);
            }


        }

        // Remove all open bubbles
        for (index, x) in remove.iter().enumerate(){
            open_index.remove(x-index);
        }
        // This is only relevant for the first entry
        if trigger{
           open_index.push(shared_index);
        }

    }
    bubble
}


/// Detect bubbles but only return the start and end node id
///
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
            else if (start[0] == index_tuple[0]) | (start[1] == index_tuple[1]) {
                trigger = false;
                index  += 1;
                continue;
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
    use crate::{sort_tuple_vector, bifurcation_analysis, bifurcation_analysis_meta};
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    /// Check if the sorting works.
    fn sort_test() {
        init();
        info!("S");
        let mut vec = vec![[1, 2], [4, 5], [3, 4], [3, 3]];
        sort_tuple_vector(&mut vec);
    }

    #[test]
    ///
    fn detect_bubbles1(){
        let mut vec = vec![[1, 2], [4, 5], [3, 4], [3, 3]];
        sort_tuple_vector(&mut vec);
        let f = bifurcation_analysis(&vec);


    }


    #[test]
    fn detect_bubbles2(){
        let mut vec = vec![[1, 2], [4, 5], [3, 4], [3, 3]];
        sort_tuple_vector(&mut vec);
        let f = bifurcation_analysis(&vec);
        println!("hellno: {:?}", f);

    }

    #[test]
    fn detect_bubbles3(){
        let mut vec = vec![[1, 2], [4, 5], [3, 4], [3, 3]];
        sort_tuple_vector(&mut vec);
        bifurcation_analysis(&vec);

    }


    #[test]
    fn run_all(){
        let mut vec = vec![[1, 2], [4, 5], [3, 4], [3, 3]];
        sort_tuple_vector(&mut vec);
        let g = bifurcation_analysis(&vec);
        eprintln!("{:?}", g);

    }

    #[test]
    fn run_meta(){
        let mut vec = vec![[1, 2,3], [4, 5,4], [3, 4,5], [3, 3,6]];
        vec.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let g = bifurcation_analysis_meta(&vec);
        assert_eq!(vec![(3,6), (4,6)], g);
        eprintln!("okay {:?}", g);
    }


}

