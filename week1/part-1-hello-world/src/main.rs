fn main() {
    let mut n = 1;
    n += 1;
    
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{} number: {}", s, n);

    let mut v: Vec<_> = Vec::new();
    v.push(1);
    println!("vec: {:?}", &v);

    let s1 = String::from("hello");
    let v1: Vec<char> = s1.chars().collect();
    let s2: Vec<String> = v1.iter().map(|x| x.to_string()).collect();
    println!("{}", s2.join(" "));
}
