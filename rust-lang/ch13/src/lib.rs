#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|shoe| { shoe.size == shoe_size })
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn iterator_demo() {
        let v = vec![1, 2];

        let mut it = v.iter();

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];
        let it = v.iter();
        let sum: i32 = it.sum();

        assert_eq!(sum, 6);
    }

    #[test]
    fn iterator_map() {
        let v = vec![1, 2, 3];
        let it = v.iter();
        let ans: Vec<_> = it.map(|x| { x + 1 }).collect();

        assert_eq!(ans, vec![2, 3, 4]);
    }

    use super::*;

    #[test]
    fn filter_shoe_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let shoe_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            shoe_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") }
            ]
        );
    }
    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum :u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a,b)| {a*b})
            .filter(|x|{ x % 3 == 0})
            .sum();

        assert_eq!(18, sum);
    }
}