use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }

    let clone = list.clone();
    println!("{}", list);
    println!("is equal before: {}", list == clone);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    println!("{}", clone.to_string()); // ToString impl for anything impl Display
    println!("is equal after: {}", list == clone);


    let a = 1;
    let b = a;
    println!("a: {}, b: {}", a, b);
    let item = &list;
    // If you implement iterator trait:
    for val in item {
       println!("{}", val);
    }
    println!("list len: {}", item.get_size());
}
