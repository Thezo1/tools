use rng::Random;

fn main()
{
    let mut random = Random::new();
    random.randomize();
    let num = random.get_random_seed();

    println!("{}", num);
}
