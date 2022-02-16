
/// Get chunks
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

/// Get all pairs of vector
pub fn get_all_pairs<T>(vector: Vec<T>) -> Vec<(T,T)>
where T: Clone{

    let mut pairs: Vec<(T, T)> = Vec::new();
    let mut count = 0;
    for path1 in vector.iter(){
        for path2 in vector[count+1..].iter(){
            // Optional for checking
            // println!("{} {}", path1.name, path2.name);
            pairs.push((path1.clone(), path2.clone()));
        }
        count += 1;
    }
    pairs
}