mod part1;
mod part2;

fn main() {
    part1::part1();
    part2::part2();
}

#[macro_export]
macro_rules! dbg {
    ($($expr:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($expr),+)
            }
        }
    };
}
