use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

//fn simulated_expensive_calculation(intensity: i32) -> i32 {
//    println!("Calculating slowly ...");
//    thread::sleep(Duration::from_secs(2));
//
//    intensity
//}

fn generate_workout(intensity: i32, random_number: i32) {
//    let expensive_closure = |num| {
//        println!("Calculating slowly ...");
//        thread::sleep(Duration::from_secs(2));
//
//        num
//    };

    let mut expensive_result = Cache::new(|num| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));

        num
    });


    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cache<T> where T:Fn(i32) -> i32 {
    calculation: T,
    value: Option<i32>,
}

impl<T> Cache<T> where T:Fn(i32) -> i32 {
    fn new(calculation: T) -> Cache<T> {
        Cache{
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}