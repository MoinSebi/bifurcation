pub mod helper;
extern crate log;
use log::{debug};


/// **Inplace sorting of a vector Vec<(a,b)>**
/// By a then by b
/// Check later of own sorting is faster
pub fn sort_tuple_vector(vector: & mut Vec<[usize; 2]>){
    debug!("Sort the tuple");
    vector.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
}

/// Detect bubbles
/// Returns a list of tuples which span a bubble
/// These numbers are index from the second genome
pub fn bifurcation_analysis(o: & Vec<[usize; 2]>) ->  Vec<Vec<[usize; 2]>> {
    debug!("Running bifuration analysis");

    // Mutating vector of starting point of bubbles
    let mut open_index: Vec<&[usize; 2]> = Vec::new();

    // Bubbles -> dict (from -> Vec[to])
    let mut bubble = vec![vec![]; 2];


    // TODO
    // Only close bubble if both is bigger
    for shared_index in o.iter(){
        // Dummy list what "open" bubble to remove
        let mut remove = Vec::new();

        // Trigger if the same entry is already there - we always index to open_index
        let mut trigger = true;
        for (index, start) in open_index.iter().enumerate(){
            // If the next entry is just increasing by 1 in both cases --> remove and update new entry
            if &[start[0] +1, start[1] +1] == shared_index {
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


}

