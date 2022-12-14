pub mod helper;
extern crate log;
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
    let mut open_index: Vec<&(usize, usize)> = Vec::new();

    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = Vec::new();


    // TODO
    // Only close bubble if both is bigger
    for shared_index in o.iter(){
        // Dummy list what "open" bubble to remove
        let mut remove = Vec::new();

        // Trigger if the same entry is already there - we always index to open_index
        let mut trigger = true;
        for (index, (pair1_start, pair2_start)) in open_index.iter().enumerate(){
            // If the next entry is just increasing by 1 in both cases --> remove and update new entry
            if &(pair1_start +1, pair2_start +1) == shared_index {
                remove.push(index);


                // If one index is same - nothing happens
            } else if (pair1_start == &shared_index.1) | (pair2_start == &shared_index.0){
                trigger = false;
                continue;

                // If both things are bigger -> add bubble
            } else if (&shared_index.0 > pair1_start) & (&shared_index.1 > pair2_start){
                bubble.push((*pair1_start, shared_index.0));
                bubble.push((*pair2_start, shared_index.1));
                remove.push(index);
            }


        }

        // Remove all open bubbles
        for (index, x) in remove.iter().enumerate(){
            open_index.remove(x-index);
        }
        // This is only relevant for the first entry
        if trigger{
           open_index.push(&shared_index);
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

