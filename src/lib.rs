mod from_gfaR;
mod helper;
#[macro_use]
extern crate log;

use std::cmp::max;
use std::collections::HashMap;


/// Inplace sorting of a vector which includes a tuple of size two (both usize)
pub fn sort_tuple_vector(vector: & mut Vec<(usize, usize)>){
    info!("Sort the tuple");
    vector.sort_by(|a, b| (a.0.cmp(&b.0).then(a.1.cmp(&b.1))));
}

/// Detect bubbles
/// Returns a list of tuples which span a bubble
/// These numbers are index from the second genome
pub fn bifurcation_analysis(o: & Vec<(usize, usize)>) -> ( HashMap<(usize, usize), Vec<(usize, usize)>>, Option<(usize, usize)>) {
    info!("Running Bifuration analysis");

    let mut bubble2: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    // Mutating vector of starting point of bubbles
    let mut open: Vec<&(usize, usize)> = Vec::new();

    // BUbbles are saved vec((start, end)): Index of second genome
    let mut bubble = Vec::new();


    // TODO
    // Only close bubble if both is bigger
    for x in o.iter(){
        let mut remove = Vec::new();
        let mut trigger = true;
        for (i, x1) in open.iter().enumerate(){
            // If the next entry is just increasing by 1 in both cases --> Remove
            if &(x1.0+1, x1.1+1) == x {
                remove.push(i);
                // If there is difference
            } else if (x1.1 == x.1) | (x1.0 == x.0){
                trigger = false;
                continue;
            } else if (x.0 > x1.0) & (x.1 > x1.1){
                if bubble2.contains_key(x1){
                    bubble2.get_mut(x1).unwrap().push(x.clone())
                } else {
                    let g = vec![x.clone()];
                    bubble2.insert(x1.clone().clone(), vec![x.clone()]);
                }
                bubble.push([x1.0, x1.1,x.0, x.1]);
                remove.push(i);


            }


        }


        for (index, x) in remove.iter().enumerate(){
            open.remove(x-index);
        }
        if trigger{
           open.push(&x);
        }

    }
    eprintln!("{:?}", bubble2);
    eprintln!("keys {}", bubble2.keys().len());
    // let mut j: Vec<(usize, usize)> = bubble2.keys().into_iter().cloned().collect();
    // eprintln!("{}", j.len());
    // sort_tuple_vector(& mut j);
    // eprintln!("{:?}", j[0]);
    let mm = bubble2.keys().into_iter().min().cloned();



    (bubble2, mm)
}




#[cfg(test)]
mod tests {
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


}

