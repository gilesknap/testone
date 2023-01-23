trait Quack {
    fn quack(&self);
}

struct Duck();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

struct RandomBird {
    is_a_parrot: bool,
}

impl Quack for RandomBird {
    fn quack(&self) {
        if !self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

fn quack<Q>(q: &Q)
where
    Q: Quack,
{
    q.quack();
}

fn quack_everyone<I>(iter: I)
where
    I: Iterator<Item = Box<dyn Quack>>,
{
    for d in iter {
        d.quack();
    }
}

pub fn test_duck() {
    let d = Duck();

    quack(&d);

    let duck1 = Duck();
    let duck2 = RandomBird { is_a_parrot: false };
    let parrot = RandomBird { is_a_parrot: true };

    let ducks: Vec<Box<dyn Quack>> = vec![Box::new(duck1), Box::new(duck2), Box::new(parrot)];

    quack_everyone(ducks.into_iter());
}
