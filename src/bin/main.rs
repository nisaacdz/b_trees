use b_trees::BTreeMap;
fn main() {
    let keys = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vals = ["src", "avl", "tri", "dec", "inc", "lev", "mod", "ord", "bin", "lib"];
    let mut mp = BTreeMap::new();

    for (key, val) in keys.into_iter().zip(vals) {
        mp.insert(key, val);
    }

    for level in mp.avl().levels() {
        print!("[");
        for val in level {
            print!("{:?}, ", val);
        }
        println!("]")
    }

    println!("");

    let rems = [4, 6, 9];
    for rem in rems {
        mp.remove(&rem);
    }

    for level in mp.avl().levels() {
        print!("[");
        for val in level {
            print!("{:?}, ", val);
        }
        println!("]")
    }
}
