extern crate logos;

use logos::{Lexer, Logos};

use advent_of_rust::load_file;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Tile {
    #[token(".")]
    Open,

    #[token("#")]
    Tree,

    #[token("\n")]
    RowEnd,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
}

impl Map {
    fn parse(tokens: &mut Lexer<Tile>) -> Self {
        let width = tokens
            .clone()
            .take_while(|token| *token != Tile::RowEnd)
            .count();

        let tiles = tokens
            .filter(|token| *token == Tile::Open || *token == Tile::Tree)
            .collect::<Vec<Tile>>();

        let height = tiles.len() / width;

        Self {
            tiles,
            width,
            height,
        }
    }

    /// The Map's origin is at the top left. Zero indexed.
    fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        if y >= self.height {
            None
        } else {
            let idx = y * self.width + x % self.width;
            Some(self.tiles[idx])
        }
    }

    fn toboggan_path(&self, course: &mut impl Iterator<Item = (usize, usize)>) -> Vec<Tile> {
        course
            .map(|(x, y)| self.tile_at(x, y))
            .skip(1)
            .take_while(Option::is_some)
            .flatten()
            .collect()
    }

    #[allow(dead_code)]
    fn view_map(&self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            print!("{:?}({:02}), ", tile, i);
            if (i + 1) % self.width == 0 {
                println!("")
            }
        }
    }
}

fn build_slope(delta_x: usize, delta_y: usize) -> impl Iterator<Item = (usize, usize)> {
    let x = std::iter::successors(Some(0), move |n| Some(n + delta_x));
    let y = std::iter::successors(Some(0), move |n| Some(n + delta_y));

    x.zip(y)
}

fn count_trees(tiles: &[Tile]) -> usize {
    tiles.iter().filter(|tile| **tile == Tile::Tree).count()
}

fn main() {
    println!("Hello from day-03!");

    let file_contents = load_file("assets/day-03-a.input").expect("Could not read puzzle file!");
    let mut lexer = Tile::lexer(&file_contents);
    let map = Map::parse(&mut lexer);

    let mut slopes_to_try = [
        build_slope(1, 1),
        build_slope(3, 1),
        build_slope(5, 1),
        build_slope(7, 1),
        build_slope(1, 2),
    ];

    let total_trees = slopes_to_try
        .iter_mut()
        .map(|slope| map.toboggan_path(slope))
        .map(|path| count_trees(&path))
        .product::<usize>();

    println!("Ouch. Hit {} trees on the way down.", total_trees);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn toboggan_path_test() {
        let mut lex = Tile::lexer(
            "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
",
        );
        let map = Map::parse(&mut lex);

        assert_eq!(map.height, 11);
        assert_eq!(map.width, 11);

        let expected = vec![
            Tile::Open,
            Tile::Tree,
            Tile::Open,
            Tile::Tree,
            Tile::Tree,
            Tile::Open,
            Tile::Tree,
            Tile::Tree,
            Tile::Tree,
            Tile::Tree,
        ];

        let actual = map.toboggan_path(&mut build_slope(3, 1));

        assert_eq!(expected, actual);
    }

    #[test]
    fn map_access() {
        let mut lex = Tile::lexer(".#.\n#.#\n..#");
        let map = Map::parse(&mut lex);

        assert_eq!(Some(Tile::Open), map.tile_at(0, 0));
        assert_eq!(Some(Tile::Tree), map.tile_at(1, 0));
        assert_eq!(Some(Tile::Open), map.tile_at(2, 0));

        assert_eq!(Some(Tile::Tree), map.tile_at(0, 1));
        assert_eq!(Some(Tile::Open), map.tile_at(1, 1));
        assert_eq!(Some(Tile::Tree), map.tile_at(2, 1));

        assert_eq!(Some(Tile::Open), map.tile_at(0, 2));
        assert_eq!(Some(Tile::Open), map.tile_at(1, 2));
        assert_eq!(Some(Tile::Tree), map.tile_at(2, 2));

        // out of bounds beyond the height of the map
        assert_eq!(None, map.tile_at(0, 3));
        assert_eq!(None, map.tile_at(0, 4));

        // out of bounds beyond the width of the map -- should wrap!
        assert_eq!(Some(Tile::Open), map.tile_at(3, 0));
        assert_eq!(Some(Tile::Tree), map.tile_at(4, 0));
        assert_eq!(Some(Tile::Open), map.tile_at(5, 0));

        assert_eq!(Some(Tile::Open), map.tile_at(6, 0));
        assert_eq!(Some(Tile::Tree), map.tile_at(7, 0));
        assert_eq!(Some(Tile::Open), map.tile_at(8, 0));

        assert_eq!(Some(Tile::Open), map.tile_at(9, 0));
        assert_eq!(Some(Tile::Tree), map.tile_at(10, 0));
        assert_eq!(Some(Tile::Open), map.tile_at(11, 0));

        assert_eq!(Some(Tile::Tree), map.tile_at(3, 1));
        assert_eq!(Some(Tile::Open), map.tile_at(4, 1));
        assert_eq!(Some(Tile::Tree), map.tile_at(5, 1));

        assert_eq!(Some(Tile::Open), map.tile_at(3, 2));
        assert_eq!(Some(Tile::Open), map.tile_at(4, 2));
        assert_eq!(Some(Tile::Tree), map.tile_at(5, 2));
    }

    #[test]
    fn map_parsing() {
        let mut lex = Tile::lexer(".#.\n#.#\n..#");
        let map = Map::parse(&mut lex);

        assert_eq!(map.height, 3);
        assert_eq!(map.width, 3);
        assert_eq!(map.tiles.len(), 9);
    }

    #[test]
    fn tile_lexing_test() {
        let mut lex = Tile::lexer("..##..\n.#..");

        assert_eq!(lex.next(), Some(Tile::Open));
        assert_eq!(lex.next(), Some(Tile::Open));
        assert_eq!(lex.next(), Some(Tile::Tree));
        assert_eq!(lex.next(), Some(Tile::Tree));
        assert_eq!(lex.next(), Some(Tile::Open));
        assert_eq!(lex.next(), Some(Tile::Open));
        assert_eq!(lex.next(), Some(Tile::RowEnd));
        assert_eq!(lex.next(), Some(Tile::Open));
        assert_eq!(lex.next(), Some(Tile::Tree));
        assert_eq!(lex.next(), Some(Tile::Open));
        assert_eq!(lex.next(), Some(Tile::Open));
    }
}
