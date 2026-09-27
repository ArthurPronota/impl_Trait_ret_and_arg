fn iter_evens(v: &[i32]) ->impl Iterator<Item = &i32>{
    v
        .iter()
        .filter(|x| *x%2 == 0)
}

fn log(v: impl std::fmt::Display) {
    println!("{}", v) ;
}

fn main() {
    let v = vec![1, 2, 3, 4, 5] ;

    for item in iter_evens(&v) {
        println!("item: {}", item) ;
    } /* Out:
item: 2
item: 4    
     */

    log("abc"); // Out: abc
}
