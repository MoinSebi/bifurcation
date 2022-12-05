pub mod from_gfaR;
pub mod helper;
extern crate log;
use std::collections::HashMap;
use log::{debug, info};


/// Inplace sorting of a vector which includes a tuple of size two (both usize)
pub fn sort_tuple_vector(vector: & mut Vec<(usize, usize)>){
    debug!("Sort the tuple");
    vector.sort_by(|a, b| (a.0.cmp(&b.0).then(a.1.cmp(&b.1))));
}

/// Detect bubbles
/// Returns a list of tuples which span a bubble
/// These numbers are index from the second genome
pub fn bifurcation_analysis(o: & Vec<(usize, usize)>) -> ( HashMap<(usize, usize), Vec<(usize, usize)>>, Option<(usize, usize)>) {
    debug!("Running bifuration analysis");

    let mut bubble2: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    // Mutating vector of starting point of bubbles
    let mut open: Vec<&(usize, usize)> = Vec::new();

    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::new();


    // TODO
    // Only close bubble if both is bigger
    for shared_index in o.iter(){
        let mut remove = Vec::new();
        // trigger - do not update cycles
        let mut trigger = true;
        for (i, open_index) in open.iter().enumerate(){
            // If the next entry is just increasing by 1 in both cases --> remove and update new entry
            if &(open_index.0+1, open_index.1+1) == shared_index {
                remove.push(i);
                // If one index is same
            } else if (open_index.1 == shared_index.1) | (open_index.0 == shared_index.0){
                trigger = false;
                continue;
                // If both things are bigger -> add bubble
            } else if (shared_index.0 > open_index.0) & (shared_index.1 > open_index.1){
                if bubble2.contains_key(open_index){
                    bubble2.get_mut(open_index).unwrap().push(shared_index.clone())
                } else {
                    bubble2.insert(open_index.clone().clone(), vec![shared_index.clone()]);
                }
                bubble.push([open_index.0, open_index.1, shared_index.0, shared_index.1]);
                remove.push(i);


            }


        }


        for (index, x) in remove.iter().enumerate(){
            open.remove(x-index);
        }
        if trigger{
           open.push(&shared_index);
        }

    }
    let mm = bubble2.keys().into_iter().min().cloned();



    (bubble2, mm)
}




#[cfg(test)]
mod tests {
    use log::info;
    use crate::{sort_tuple_vector, bifurcation_analysis};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn sort_test() {
        init();
        info!("S");
        let mut vec = vec![(1, 2), (4, 5), (3, 4), (3, 3)];
        sort_tuple_vector(&mut vec);
        assert_eq!(vec![(1, 2), (3, 3), (3, 4), (4, 5)], vec);
    }

    #[test]
    fn detect_bubbles1(){
        let mut vec = vec![(1, 1), (3, 3), (4, 5)];
        sort_tuple_vector(&mut vec);
        bifurcation_analysis(&vec);

    }


    #[test]
    fn detect_bubbles2(){
        let mut vec = vec![(1, 1), (3, 3), (3, 4), (3, 5), (4,6)];
        sort_tuple_vector(&mut vec);
        bifurcation_analysis(&vec);

    }

    #[test]
    fn detect_bubbles3(){
        let mut vec = vec![(1, 1), (3, 3), (4, 3), (5, 3), (6,4)];
        sort_tuple_vector(&mut vec);
        bifurcation_analysis(&vec);

    }


    #[test]
    fn run_all(){
        let mut vec = vec![(1, 1), (2,3), (3,2), (4,4)];
        sort_tuple_vector(&mut vec);
        let g = bifurcation_analysis(&vec);
        eprintln!("{:?}", g);

    }


}

