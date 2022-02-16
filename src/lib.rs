mod from_gfaR;
mod helper;

/// Inplace sorting of a vector which includes a tuple of size two (both usize)
pub fn sort_tuple_vector(vector: & mut Vec<(usize, usize)>){

    vector.sort_by(|a, b| (a.0.cmp(&b.0).then(a.1.cmp(&b.1))));
}

/// Detect bubbles
/// Returns a list of tuples which span a bubble
/// These numbers are index from the second genome
pub fn bifurcation_analysis(o: & Vec<(usize, usize)>) -> Vec<(usize, usize)>{

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
            if (&(x1.0+1, x1.1+1) == x) {
                remove.push(i);
                // If there is difference
            } else if (x1.1 == x.1) | (x1.0 == x.0){
                trigger = false;
                continue;
            } else if ((x.0 > x1.0) & (x.1 > x1.1)){
                bubble.push((x1.1, x.1));
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
    eprintln!("bubbles {:?}", bubble);
    bubble
}




#[cfg(test)]
mod tests {
    use crate::{sort_tuple_vector, bifurcation_analysis};

    #[test]
    fn sort_test() {
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

