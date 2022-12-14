pub mod helper;
extern crate log;
use hashbrown::HashMap;
use log::{debug};


/// **Inplace sorting of a vector Vec<(a,b)>**
/// By a then by b
pub fn sort_tuple_vector(vector: & mut Vec<(usize, usize)>){
    debug!("Sort the tuple");
    vector.sort_by(|a, b| (a.0.cmp(&b.0).then(a.1.cmp(&b.1))));
}

/// Detect bubbles
/// Returns a list of tuples which span a bubble
/// These numbers are index from the second genome
pub fn bifurcation_analysis(o: & Vec<(usize, usize)>) ->  Vec<(usize, usize)> {
    debug!("Running bifuration analysis");

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
        for (i, (open_start, open_end)) in open.iter().enumerate(){
            // If the next entry is just increasing by 1 in both cases --> remove and update new entry
            if &(open_start+1, open_end+1) == shared_index {
                remove.push(i);


                // If one index is same - nothing happens
            } else if (*open_start == shared_index.1) | (*open_end == shared_index.0){
                trigger = false;
                continue;

                // If both things are bigger -> add bubble
            } else if (shared_index.0 > *open_start) & (shared_index.1 > *open_end){
                bubble.push((*open_start, shared_index.0));
                bubble.push((*open_end, shared_index.1));
                remove.push(i);


            }


        }

        // Remove all open bubbles
        for (index, x) in remove.iter().enumerate(){
            open.remove(x-index);
        }
        // This is only relevant for the first entry
        if trigger{
           open.push(&shared_index);
        }

    }



    bubble
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

