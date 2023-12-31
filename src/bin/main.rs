use b_trees::BTreeMap;
fn main() {
    let keys = [0, 1, 2, 3, 0, 1, 2, 3, 8, 9];
    let vals = ["src", "avl", "tri", "dec", "inc", "lev", "mod", "ord", "bin", "lib"];
    let mut mp = BTreeMap::new();

    for (key, val) in keys.into_iter().zip(vals) {
        mp.insert(key, val);
    }

    while !mp.is_empty() {
        let last_idx = mp.len() as i32 - 1;
        println!("{:?}", mp.delete(&last_idx).unwrap());
        println!("{:?}", mp);
        println!("")
    }

}
