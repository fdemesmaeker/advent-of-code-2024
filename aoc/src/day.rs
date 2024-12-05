pub trait Challenge {
    fn part_1(&self) -> i32;
    fn part_2(&self) -> i32;
    fn run(&self) -> ();
}