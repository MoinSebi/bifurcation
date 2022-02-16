#[cfg(test)]
mod tests {
    use crate::{test_cat, open_close};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut k = vec![(1,2),(3,3),(3,4),(4,5)];
        test_cat(& mut k);
        open_close(&k);
        eprintln!("{:?}", k);
    }

}

pub fn test_cat(o: & mut Vec<(u32, u32)>){
    o.sort_by(|a, b| (a.0.cmp(&b.0).then(a.1.cmp(&b.1))));
}

/// this is the thing
pub fn open_close(o: & Vec<(u32, u32)>){
    let mut j: (u32, u32) = (0,0);
    let k = o[0].clone();
    let mut open: Vec<&(u32, u32)> = Vec::new();
    let mut bubble = Vec::new();
    for (i, x) in o.iter().enumerate(){
        let mut update = Vec::new();
        for (i, x1) in open.iter().enumerate(){
            eprintln!("{:?}", (x1.0+1, x1.1+1));
            eprintln!("{:?}", x);

            if &(x1.0+1, x1.1+1) == x{
                update.push(i);
            } else {
                eprintln!("hit");
                bubble.push((x1.1, x.1));
                update.push(i);

            }
            eprintln!("{}", x1.1);


        }
        for (index, x) in update.iter().enumerate(){
            open.remove(x-index);
        }
        open.push(&x);
    }
    eprintln!("bubbles {:?}", bubble);
}
