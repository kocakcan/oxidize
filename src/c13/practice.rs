fn main() {
    let v: Vec<i32> = vec![10, -5, 4, 9, 7, 16, 11];
    let result: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{result:?}");

    let result: Vec<i32> = v.iter().map(|x| *x + 1).collect();
    println!("{result:?}");

    let kocaks: Vec<&str> = vec!["Seyfi", "Leyli", "Dilan", "Medet", "Can"];
    let result: Vec<&&str> = kocaks.iter().filter(|x| x.contains('a')).collect();
    println!("{result:?}");
}
