pub mod test;
extern crate log;
use std::cmp::{max, min, Ordering, Reverse};

use std::collections::BTreeSet;
use std::collections::{BinaryHeap};
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
pub fn bifurcation_meta(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

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


pub fn bifurcation_vec_sorted(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

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



pub fn bifurcation_sort_hold(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index = Vec::new();
    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());
    let mut trig = false;
    let mut index = 0;
    let mut hold = Vec::new();
    let mut next = shared_index[1][0];
    // Only close bubble if both is bigger
    for index_tuple in 0..shared_index.len()-1{
        // Dummy list what "open" bubble to remove

        let [istart, iend, ibub] = shared_index[index_tuple];
        // Trigger if the same entry is already there - we always index to open_index
        // Index to delete entries on the fly
        index = open_index.len();
        trig = false;
        next = shared_index[index_tuple+1][0];
        let m = next == istart;

        if next == istart {
            hold.push([istart, iend, ibub]);
            trig = true;

        }


        while index > 0{

            let [ostart, oend, obub]: _ = open_index[index-1];


            if (istart > ostart) && (iend > oend) {
                if !(ostart + 1 == istart && oend + 1 == iend) {
                    bubble.push((min(ibub, obub), max(ibub, obub)));
                }
                open_index.pop();
                index -= 1;
                continue
            } else {
                if !m{
                    if hold.len() == 0 {
                        if index == open_index.len() {
                            open_index.push([istart, iend, ibub]);
                        } else {
                            open_index.insert(index, [istart, iend, ibub]);
                        }
                    } else {
                        // Add all index at one
                        hold.push([istart, iend, ibub]);
                        insert_sorted_vector(&mut open_index, &hold);
                        hold.clear();
                    }
                    trig = true;
                    break

                }
                else {
                    break
                }


            }

        }



        if !trig && hold.len() == 0{
            open_index.insert(0, [istart, iend, ibub]);
        } else if !trig{
            hold.push([istart, iend, ibub]);
            insert_sorted_vector(&mut open_index, &hold);
            hold.clear();
        }

    }
    let [istart, iend, ibub] = *shared_index.last().unwrap();

    index = open_index.len();
    while index > 0{

        let [ostart, oend, obub] = open_index[index-1];


        if (istart > ostart) && (iend > oend) {
            if !(ostart + 1 == istart && oend + 1 == iend) {
                bubble.push((min(ibub, obub), max(ibub, obub)));
            }
            open_index.pop();
        }
        index -= 1;

    }
    bubble
}



pub fn bifurcation_sort_inplace(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index = Vec::new();
    open_index.push(shared_index[0]);
    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());
    let mut trig = false;
    let mut index = 0;

    // Only close bubble if both is bigger
    for index_tuple in 1..shared_index.len(){
        // Dummy list what "open" bubble to remove

        let [istart, iend, ibub] = shared_index[index_tuple];
        // Trigger if the same entry is already there - we always index to open_index
        // Index to delete entries on the fly
        index = open_index.len();
        trig = false;

        while index > 0{
            let [ostart, oend, obub] = open_index[index-1];
            if (istart > ostart) && (iend > oend) {
                if !(ostart + 1 == istart && oend + 1 == iend) {
                    bubble.push((min(ibub, obub), max(ibub, obub)));
                }
                open_index.remove(index-1);
                index -= 1;
                continue
            }
            if oend > iend {
                if index == open_index.len(){
                    open_index.push([istart, iend, ibub]);
                } else {
                    open_index.insert(index, [istart, iend, ibub]);
                }
                //open_index.insert(index, [istart, iend, ibub]);
                trig = true;
                break
            }
            index -= 1;
        }
        if !trig{
            open_index.insert(0, [istart, iend, ibub]);
        }
    }
    bubble
}







pub fn bifurcation_btreeset(shared_index: &[[u32; 3]]) ->  Vec<(u32, u32)> {

    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index: BTreeSet<SumOrdering> = BTreeSet::new();
    open_index.insert(SumOrdering(shared_index[0]));
    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::with_capacity(shared_index.len());



    // Only close bubble if both is bigger
    for index_tuple in 1..shared_index.len(){
        // Dummy list what "open" bubble to remove

        let [istart, iend, ibub] = shared_index[index_tuple];
        // Trigger if the same entry is already there - we always index to open_index
        // Index to delete entries on the fly

        for x in open_index.iter(){
            let [ostart, oend, obub] = &x.0;
            if (istart > *ostart) && (iend > *oend) {
                if !(ostart + 1 == istart && oend + 1 == iend) {
                    bubble.push((min(ibub, *obub), max(ibub, *obub)));
                }
            }

            if oend > &iend {
                break
            }
        }


        open_index.retain(|a| ((istart <= a.0[0]) || (iend <= a.0[1])));
        open_index.insert(SumOrdering([istart, iend, ibub]));

        // This is only relevant for the first entry
    }
    bubble
}



/// Insert a sorted vector into another sorted vector
pub fn insert_sorted_vector(l1: &mut Vec<[u32;3]>, l2: &Vec<[u32;3]>){
    let mut index = l1.len();
    let mut vec_iter = l2.iter();
    let mut next_val = vec_iter.next();
    while index >0{
        if next_val.unwrap()[1] < l1[index-1][1]{
            l1.insert(index, *next_val.unwrap());
            next_val = vec_iter.next();
        } else {
            index -= 1;
        }
        if next_val.is_none(){
            break
        }
    }
    if next_val.is_some(){
        while next_val.is_some(){
            l1.insert(0, *next_val.unwrap());
            next_val = vec_iter.next();
        }
    }

}

#[derive(Eq, PartialEq)]
struct SumOrdering([u32; 3]);

impl Ord for SumOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0[1].cmp(&other.0[1]).then(self.0[0].cmp(&other.0[0]))
    }
}

impl PartialOrd for SumOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}




