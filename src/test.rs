use std::fs::File;
use std::io::{BufRead, BufReader};


/// Creating data
/// Might change for different settings
pub fn data_creation(length: u32, rev_size: u32, freq: u32) -> Vec<[u32; 3]> {
    let mut example_data = Vec::new();

    for x in 0..length{
        if x%freq == 0{
            for x1 in 0..rev_size {
                example_data.push([x+x1, 100000+x-x1, 1]);
            }
            example_data.push([x,x+50000,10])
        } else if x%5 == 0{
            example_data.push([x+3, x+1000,10])
        } else {
            example_data.push([x+1, x+1,10])

        }
    }
    example_data.sort();
    example_data
}

/// Load data from a file
/// Data from BVD
pub fn load_data(filen: &str) -> Vec<[u32; 3]> {
    let mut mm = Vec::new();
    let file = File::open(filen).unwrap();
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line_result in reader.lines(){
        let line = line_result.unwrap();
        let f: Vec<_> = line.split("\t").collect();
        //println!("{:?}", f);
        mm.push([f[1].parse::<u32>().unwrap(), f[2].parse::<u32>().unwrap(), f[3].parse::<u32>().unwrap()]);

    }
    mm
}


/// Running test
#[cfg(test)]
mod tests {
    use std::ops::Sub;
    use crate::{sort_array_vec, bifurcation_analysis_meta, bifurcation_analysis_sort, bifurcation_analysis_btree, bifurcation_analysis_bheap};
    use crate::test::{data_creation, load_data};


    #[test]
    /// Testing sorting
    fn test_check_sorting() {
        let mut vec1 = vec![[1, 2, 3], [1, 3, 4], [1, 2, 4], [2, 10, 11], [3, 20, 13]];
        let mut vec2 = vec1.clone();
        vec1.sort();
        sort_array_vec(&mut vec2);
        assert_eq!(vec1, vec2);
        assert_eq!(vec1, vec![[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 10, 11], [3, 20, 13]])
    }


    #[test]
    /// Esting a simple example
    fn test_easy() {
        let mut vec = vec![[1, 2, 3], [4, 5, 4], [3, 4, 5], [3, 3, 6], [1, 10, 19]];
        //let mut vec = data_creation();
        println!("{}", vec.len());
        vec.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let g = bifurcation_analysis_meta(&vec[..]);
        assert_eq!(vec![(3, 6), (4, 6)], g);
    }

    #[test]
    /// Testing repeat example (simple)
    fn test_repeat() {
        let mut vec = vec![[1, 1, 1], [1, 20, 1], [3, 3, 3], [2, 2, 2], [3, 21, 3]];
        //let mut vec = data_creation();
        println!("{}", vec.len());
        vec.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let g = bifurcation_analysis_meta(&vec[..]);
        let g2 = bifurcation_analysis_sort(&vec[..]);

        assert_eq!(g2, g);
    }


    #[test]
    /// Testing data from data creation (complex)
    fn test_data_creation() {
        let mut data2 = data_creation(500000, 100, 500);
        //let mut data2 = load_data2("data/data.txt");
        sort_array_vec(&mut data2);        //let mut vec = data_creation();
        data2.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let mut g = bifurcation_analysis_meta(&data2[..]);
        let mut g2 = bifurcation_analysis_sort(&data2[..]);
        g.sort();
        g2.sort();
        assert_eq!(g2.len(), g.len());
    }


    #[test]
    /// Testing real data
    fn test_real_data() {
        let mut data2 = load_data("data/test.index.20000.txt");
        //let mut data2 = load_data2("data/data.txt");
        sort_array_vec(&mut data2);        //let mut vec = data_creation();
        data2.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let mut g = bifurcation_analysis_meta(&data2[..]);
        let mut g2 = bifurcation_analysis_sort(&data2[..]);
        g.sort();
        g2.sort();
        assert_eq!(g2.len(), g.len());
    }

    #[test]
    fn test_real_data_btree() {
//        let mut data2 = load_data("data/test.index.20000.txt");
        let mut data2 = data_creation(500000, 100, 500);

        //let mut data2 = load_data2("data/data.txt");
        sort_array_vec(&mut data2);        //let mut vec = data_creation();
        data2.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let mut g = bifurcation_analysis_btree(&data2[..]);
        let mut g2 = bifurcation_analysis_sort(&data2[..]);
        g.sort();
        g2.sort();
        let g_set = g.iter().collect::<std::collections::HashSet<_>>();
        let g2_set = g2.iter().collect::<std::collections::HashSet<_>>();
        let f: Vec<_> = g2_set.difference(&g_set).collect();
        eprintln!("{:?}", f);
        assert_eq!(g.len(), g2.len());

        assert_eq!(f.len(),0)
    }

    #[test]
    fn test_real_data_bheap() {
        let mut data2 = load_data("data/test.index.20000.txt");
        let mut data2 = data_creation(500000, 100, 500);

        //let mut data2 = load_data2("data/data.txt");
        sort_array_vec(&mut data2);        //let mut vec = data_creation();
        data2.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
        let mut g = bifurcation_analysis_sort(&data2[..]);
        let mut g2 = bifurcation_analysis_bheap(&data2[..]);
        g.sort();
        g2.sort();

        let g_set = g.iter().collect::<std::collections::HashSet<_>>();
        let g2_set = g2.iter().collect::<std::collections::HashSet<_>>();
        let f: Vec<_> = g_set.symmetric_difference(&g2_set).collect();
        //eprintln!("{:?}", f);
        assert_eq!(g_set.len(), g2_set.len());

        assert_eq!(f.len(),0);
        assert_eq!(g2.len(), g.len());
    }
}




