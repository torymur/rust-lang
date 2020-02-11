use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, A, R>
    where T: Fn(A) -> R
{
    calculation: T,
    cache: HashMap<A, R>,
}

impl<T, A, R> Cacher<T, A, R>
    where T: Fn(A) -> R,
          A: std::cmp::Eq + std::hash::Hash + Copy,
          R: Copy
{
    fn new(calculation: T) -> Cacher<T, A, R> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }
    fn value(&mut self, arg: A) -> R {
        match self.cache.get(&arg) {
            Some(v) => *v,
            None => {
				let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
            );
        println!(
            "Next, do {} situps",
            expensive_result.value(intensity)
            );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
                );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number)
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn generic_cacher() {
    let mut c1 = Cacher::new(|x: &str| {
        if x.contains("zz") {
            "Zzz"
        } else {
            "Awake!"
        }
    });
    assert_eq!(c1.value("Lazzzzzy"), "Zzz");

    let mut c2 = Cacher::new(|x| {
        if x == 5 {
            "Zzz"
        } else {
            "Awake!"
        }
    });
    assert_eq!(c2.value(10), "Awake!");
}
