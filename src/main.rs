extern crate rand;
extern crate itertools;

mod board;
use rand::{thread_rng, Rng};

#[cfg(test)]
mod slide_test {
    use super::slide;
    #[test]
    fn test_slide_with_one_element() {
        assert_eq!([0, 0, 0, 1], slide([0, 1, 0, 0]));
    }
    #[test]
    fn test_slide_with_two_different_elements() {
        assert_eq!([0, 0, 2, 1], slide([1, 0, 2, 0]));
    }
    #[test]
    fn test_slide_with_two_same_elements() {
        assert_eq!([0, 0, 0, 2], slide([1, 0, 1, 0]));
    }
    #[test]
    fn test_slide_with_three_same_elements() {
        assert_eq!([0, 0, 1, 2], slide([1, 0, 1, 1]));
    }
    #[test]
    fn test_slide_with_three_different_elements() {
        assert_eq!([0, 0, 2, 2], slide([1, 0, 1, 2]));
        assert_eq!([0, 2, 1, 2], slide([2, 0, 1, 2]));
        assert_eq!([0, 0, 2, 2], slide([0, 1, 1, 2]));
    }
    #[test]
    fn test_slide_with_four_same_elements() {
        assert_eq!([0, 0, 2, 2], slide([1, 1, 1, 1]));
    }
    #[test]
    fn test_slide_with_four_different_elements() {
        assert_eq!([1, 2, 1, 2], slide([1, 2, 1, 2]));
    }
}

fn merge(slice: &mut [i32]) {
    if slice[0] == slice[1] && slice[1] != 0 {
        slice[0] += 1;
        slice[1] = 0;
    }
}

fn slide(data: [i32; 4]) -> [i32; 4] {
    use itertools::partition;
    let mut ret = data.clone();
    partition(&mut ret, |x| *x == 0);
    ret.reverse();
    let mut index = 0;
    while (index < 3) {
        merge(&mut ret[index..index + 2]);
        index += 1;
    }
    ret.reverse();
    partition(&mut ret, |x| *x == 0);
    ret
}

enum Move {
    up,
    down,
    left,
    right,
}

enum GameStatus {
    ongoing,
    won,
    lost,
    interrupted,
}

struct Game {
    status: GameStatus,
    score: i32,
    data: [i32; 16],
}

impl Game {
    pub fn new() -> Game {
        let mut rng = thread_rng();
        let mut data = [0; 16];
        data[0] = 1;
        data[1] = 1;
        rng.shuffle(&mut data);
        Game {
            status: GameStatus::ongoing,
            score: 0,
            data: data,
        }
    }
    fn data(self) -> [i32; 16] {
        self.data
    }
}

fn main() {
    let board = board::Board::new();
    let game = Game::new();
    board.print(game.data());
}
