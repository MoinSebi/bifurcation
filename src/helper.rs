
/// **Get chunks of a Vector**
///
/// Takes full vector and get new vector
pub fn chunk_inplace<T>(it: Vec<T>, numb: usize) -> Vec<Vec<T>>{
    let mut vec_new: Vec<Vec<T>> = Vec::new();
    for _x in 0..numb{
        vec_new.push(Vec::new());
    }
    let each_size = (it.len() as f64 /numb as f64).ceil() as usize;

    let mut count = 0;
    for x in it{

        vec_new[count/each_size].push(x);
        count += 1;

    }
    vec_new

}

/// **Get all pairs of a vector**
///
/// - Only upper "triangle"
/// - Clones the items
pub fn get_all_pairs<T>(vector: &Vec<T>) -> Vec<(T,T)>
where T: Clone{
    let mut pairs: Vec<(T, T)> = Vec::new();
    let mut count = 0;
    for item1 in vector.iter(){
        for item2 in vector[count+1..].iter(){
            pairs.push((item1.clone(), item2.clone()));
        }
        count += 1;
    }
    pairs
}