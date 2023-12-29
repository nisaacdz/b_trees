use b_trees::AVL;
fn main() {
    let nums = [2, 3, 4, 5, 6, 7, 8];
    let avl = nums.into_iter().collect::<AVL<_>>();
    println!("{:?}", avl.less_than(&5).collect::<Vec<_>>());
}
