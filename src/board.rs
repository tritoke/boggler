use std::{iter::FusedIterator, ops::Index};

#[allow(unused)]
pub enum Tile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    QU,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Tile {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Tile::A => "A",
            Tile::B => "B",
            Tile::C => "C",
            Tile::D => "D",
            Tile::E => "E",
            Tile::F => "F",
            Tile::G => "G",
            Tile::H => "H",
            Tile::I => "I",
            Tile::J => "J",
            Tile::K => "K",
            Tile::L => "L",
            Tile::M => "M",
            Tile::N => "N",
            Tile::O => "O",
            Tile::P => "P",
            Tile::QU => "QU",
            Tile::R => "R",
            Tile::S => "S",
            Tile::T => "T",
            Tile::U => "U",
            Tile::V => "V",
            Tile::W => "W",
            Tile::X => "X",
            Tile::Y => "Y",
            Tile::Z => "Z",
        }
    }
}

impl std::fmt::Display for Tile {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // don't worry I hate myself too
        f.write_str(match self {
            Tile::A  => "A ",
            Tile::B  => "B ",
            Tile::C  => "C ",
            Tile::D  => "D ",
            Tile::E  => "E ",
            Tile::F  => "F ",
            Tile::G  => "G ",
            Tile::H  => "H ",
            Tile::I  => "I ",
            Tile::J  => "J ",
            Tile::K  => "K ",
            Tile::L  => "L ",
            Tile::M  => "M ",
            Tile::N  => "N ",
            Tile::O  => "O ",
            Tile::P  => "P ",
            Tile::QU => "QU",
            Tile::R  => "R ",
            Tile::S  => "S ",
            Tile::T  => "T ",
            Tile::U  => "U ",
            Tile::V  => "V ",
            Tile::W  => "W ",
            Tile::X  => "X ",
            Tile::Y  => "Y ",
            Tile::Z  => "Z ",
        })
    }
}

pub struct Boggle {
    pub tiles: [[Tile; 4]; 4],
}

impl std::fmt::Display for Boggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("┌──┬──┬──┬──┐\n")?;
        for (i, [a, b, c, d]) in self.tiles.iter().enumerate() {
            write!(f, "│{a}│{b}│{c}│{d}│\n",)?;
            if i < 3 {
                f.write_str("├──┼──┼──┼──┤\n")?;
            }
        }
        f.write_str("└──┴──┴──┴──┘")?;

        Ok(())
    }
}

impl Index<TileLocation> for Boggle {
    type Output = Tile;

    fn index(&self, index: TileLocation) -> &Self::Output {
        &self.tiles[index.y][index.x]
    }
}

/// Opaque Representation of the location of a tile on a boggle board
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileLocation {
    // type invariant: both must be between 0 and 3
    x: usize,
    y: usize,
}

impl From<TileLocation> for u8 {
    fn from(TileLocation { x, y }: TileLocation) -> Self {
        x as u8 | ((y << 2) as u8)
    }
}

impl TryFrom<(usize, usize)> for TileLocation {
    type Error = &'static str;

    fn try_from((x, y): (usize, usize)) -> Result<Self, Self::Error> {
        if x > 3 || y > 3 {
            return Err("Number too big, smaller number please :)");
        }

        Ok(Self { x, y })
    }
}

/// Packed representation of a path around a boggle board
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BoardPath {
    /// This is treated as effectively [u4; 16]
    /// the lowest u4 is the number of packed locations
    locations_packed: u64,
}

impl BoardPath {
    pub fn starting_at(tile_loc: TileLocation) -> Self {
        let mut board_path = Self {
            locations_packed: 0,
        };

        board_path.push(tile_loc);

        board_path
    }

    pub fn push(&mut self, index: TileLocation) {
        let shift = self.locations_packed & 0b1111;
        assert!(shift != 0b1111, "Cannot push to a full BoardPath");

        self.locations_packed |= (u8::from(index) as u64) << ((shift + 1) * 4);
        self.locations_packed += 1;
    }

    pub fn len(&self) -> usize {
        (self.locations_packed & 0b1111) as usize
    }

    const fn tile_position_to_neighbour_bitmask(packed_tile_loc: u64) -> u16 {
        assert!(packed_tile_loc <= 0b1111);

        let x = packed_tile_loc & 0b11;
        let y = packed_tile_loc >> 2;

        let mut mask = 0;

        let xmin = x.saturating_sub(1);
        let ymin = y.saturating_sub(1);
        let xmax = if x == 3 { 3 } else { x + 1 };
        let ymax = if y == 3 { 3 } else { y + 1 };

        let mut xx = xmin;
        while xx <= xmax {
            let mut yy = ymin;
            while yy <= ymax {
                debug_assert!(xx <= 3);
                debug_assert!(yy <= 3);

                if xx == x && yy == y {
                    yy += 1;
                    continue;
                }

                let tilepos_packed = xx | (yy << 2);
                mask |= 1 << tilepos_packed;

                yy += 1;
            }

            xx += 1;
        }

        mask
    }

    const TILELOC_NEIGHBOURS: [u16; 16] = [
        Self::tile_position_to_neighbour_bitmask(0b0000),
        Self::tile_position_to_neighbour_bitmask(0b0001),
        Self::tile_position_to_neighbour_bitmask(0b0010),
        Self::tile_position_to_neighbour_bitmask(0b0011),
        Self::tile_position_to_neighbour_bitmask(0b0100),
        Self::tile_position_to_neighbour_bitmask(0b0101),
        Self::tile_position_to_neighbour_bitmask(0b0110),
        Self::tile_position_to_neighbour_bitmask(0b0111),
        Self::tile_position_to_neighbour_bitmask(0b1000),
        Self::tile_position_to_neighbour_bitmask(0b1001),
        Self::tile_position_to_neighbour_bitmask(0b1010),
        Self::tile_position_to_neighbour_bitmask(0b1011),
        Self::tile_position_to_neighbour_bitmask(0b1100),
        Self::tile_position_to_neighbour_bitmask(0b1101),
        Self::tile_position_to_neighbour_bitmask(0b1110),
        Self::tile_position_to_neighbour_bitmask(0b1111),
    ];

    pub fn valid_next_tiles(&self) -> NextTileIterator {
        let no_locs = self.locations_packed & 0b1111;
        let last_tile_loc = (self.locations_packed >> (no_locs * 4)) & 0b1111;

        let mut mask = Self::TILELOC_NEIGHBOURS[last_tile_loc as usize];

        for i in 1..no_locs {
            let tile_loc = (self.locations_packed >> (i * 4)) & 0b1111;
            mask &= !(1 << tile_loc);
        }

        let mut packed_tile_locs: u32 = 0;
        let mut count = 0;
        for i in 0..16 {
            // if a position is not in the mask then skip it
            if (mask >> i) & 1 != 1 {
                continue;
            }

            packed_tile_locs <<= 4;
            packed_tile_locs |= i;
            count += 1;
        }

        NextTileIterator {
            packed: ((packed_tile_locs as u64) << 8) | (count << 4),
        }
    }
}

impl IntoIterator for BoardPath {
    type Item = TileLocation;

    type IntoIter = BoardPathIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardPathIterator {
           path: self,
           i: 0, 
        }
    }
}

/// An iterator over the TileLocation's that make up the BoardPath
pub struct BoardPathIterator {
    path: BoardPath,
    i: usize,
}

impl Iterator for BoardPathIterator {
    type Item = TileLocation;

    fn next(&mut self) -> Option<Self::Item> {
        let no_locs = (self.path.locations_packed & 0b1111) as usize;

        if self.i == no_locs {
            return None;
        }

        let next_packed_tileloc = self.path.locations_packed >> (4 * (self.i + 1)) & 0b1111;
        let x = (next_packed_tileloc & 0b11) as usize;
        let y = (next_packed_tileloc >> 2) as usize;

        debug_assert!(x <= 3);
        debug_assert!(y <= 3);

        self.i += 1;

        Some(TileLocation { x, y })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let no_locs = (self.path.locations_packed & 0b1111) as usize;

        debug_assert!(self.i <= no_locs);

        let remaining = no_locs - self.i;
        (remaining, Some(remaining))
    }
}

impl FusedIterator for BoardPathIterator {}
impl ExactSizeIterator for BoardPathIterator {}

/// An iterator over the TileLocation's that can advance a BoardPath
#[repr(transparent)]
pub struct NextTileIterator {
    // This is treated as [u4; 16] again
    // the lowest u4 is which element of the iterator we are on
    // the next u4 is now many positions this iterator will yield
    // the remaining u4s are packed TileLocations
    packed: u64,
}

impl Iterator for NextTileIterator {
    type Item = TileLocation;

    fn next(&mut self) -> Option<Self::Item> {
        let tile_no = (self.packed & 0b1111) as usize;
        let no_locs = ((self.packed >> 4) & 0b1111) as usize;

        debug_assert!(tile_no <= no_locs);

        if tile_no == no_locs {
            return None;
        }

        let tile_loc_packed = self.packed >> ((tile_no + 2) * 4);
        let x = (tile_loc_packed & 0b11) as usize;
        let y = ((tile_loc_packed >> 2) & 0b11) as usize;

        self.packed += 1;

        // invariant is upheld as both coordinates are masked with 0b11
        Some(TileLocation { x, y })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let tile_no = (self.packed & 0b1111) as usize;
        let no_locs = ((self.packed >> 4) & 0b1111) as usize;

        debug_assert!(tile_no <= no_locs);

        let remaining = no_locs - tile_no;
        (remaining, Some(remaining))
    }
}

// idk lets the compiler optimise maybe?? lol
impl FusedIterator for NextTileIterator {}
impl ExactSizeIterator for NextTileIterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use Tile::*;

    macro_rules! tile {
        ($x:expr, $y:expr) => {
            TileLocation::try_from(($x, $y)).unwrap()
        };
    }

    #[test]
    fn test_tilelocation_try_from_accepts_valid() {
        for x in 0..=3 {
            for y in 0..=3 {
                tile!(x, y);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_tilelocation_try_rejects_invalid_x() {
        tile!(4, 3);
    }

    #[test]
    #[should_panic]
    fn test_tilelocation_try_rejects_invalid_y() {
        tile!(3, 4);
    }

    #[test]
    fn test_next_tile_iterator_top_left() {
        let path = BoardPath::starting_at(tile!(0, 0));
        let mut tiles: Vec<_> = path.valid_next_tiles().collect();
        tiles.sort();
        assert_eq!(tiles, vec![tile!(0, 1), tile!(1, 0), tile!(1, 1)]);
    }

    #[test]
    fn test_next_tile_iterator_middle() {
        let path = BoardPath::starting_at(tile!(1, 1));
        let mut tiles: Vec<_> = path.valid_next_tiles().collect();
        tiles.sort();
        #[rustfmt::skip]
        assert_eq!(tiles, vec![
            tile!(0, 0), tile!(0, 1), tile!(0, 2), 
            tile!(1, 0),              tile!(1, 2), 
            tile!(2, 0), tile!(2, 1), tile!(2, 2), 
        ]);
    }

    #[test]
    fn test_next_tile_iterator_middle_obscured() {
        let mut path = BoardPath::starting_at(tile!(1, 1));
        path.push(tile!(0, 1));

        let mut tiles: Vec<_> = path.valid_next_tiles().collect();
        tiles.sort();
        #[rustfmt::skip]
        assert_eq!(tiles, vec![
            tile!(0, 0),              tile!(0, 2), 
            tile!(1, 0),              tile!(1, 2), 
        ]);

        path.push(tile!(0, 0));

        tiles = path.valid_next_tiles().collect();
        tiles.sort();
        assert_eq!(tiles, vec![tile!(1, 0),]);

        path.push(tile!(1, 0));
        tiles = path.valid_next_tiles().collect();
        tiles.sort();
        assert_eq!(tiles, vec![tile!(2, 0), tile!(2, 1)]);
    }
}
