fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("[1] The largest number is {}", largest_general(&number_list));
    println!("[1] The largest char is {}", largest_general(&char_list));
    println!("[2] The largest number is {}", largest_general2(&number_list));
    println!("[2] The largest char is {}", largest_general2(&char_list));
    println!("[3] The largest number is {}", largest_general3(&number_list));
    println!("[3] The largest char is {}", largest_general3(&char_list));
}

#[allow(dead_code)]
// as an example of not general function
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_general<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Using the clone function means we’re potentially making more heap
// allocations in the case of types that own heap data like String,
// and heap allocations can be slow if we’re working with large amounts of data
fn largest_general2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        let new_item = item.clone();
        if new_item > largest {
            largest = new_item;
        }
    }
    largest
}

// If we change the return type to &T instead of T, thereby changing the
// body of the function to return a reference, we wouldn’t need the Clone
// or Copy trait bounds and we could avoid heap allocation
fn largest_general3<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = &item;
        }
    }
    largest
}
