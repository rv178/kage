use crate::*;

pub fn get_material_count(game_status: &GameStatus, colour: Colour) -> u16 {
    let mut total_material: u16 = 0;

    for piece in game_status.pieces.into_iter().flatten() {
        if piece.colour == colour {
            total_material += match piece.kind {
                Kind::Pawn => 1,
                Kind::Knight => 3,
                Kind::Bishop => 3,
                Kind::Rook => 5,
                Kind::Queen => 9,
                Kind::King => 0,
            };
        }
    }

    total_material
}

pub fn match_u32_to_sq(sq: u32) -> Square {
    match sq {
        0 => Square::A8,
        1 => Square::B8,
        2 => Square::C8,
        3 => Square::D8,
        4 => Square::E8,
        5 => Square::F8,
        6 => Square::G8,
        7 => Square::H8,
        8 => Square::A7,
        9 => Square::B7,
        10 => Square::C7,
        11 => Square::D7,
        12 => Square::E7,
        13 => Square::F7,
        14 => Square::G7,
        15 => Square::H7,
        16 => Square::A6,
        17 => Square::B6,
        18 => Square::C6,
        19 => Square::D6,
        20 => Square::E6,
        21 => Square::F6,
        22 => Square::G6,
        23 => Square::H6,
        24 => Square::A5,
        25 => Square::B5,
        26 => Square::C5,
        27 => Square::D5,
        28 => Square::E5,
        29 => Square::F5,
        30 => Square::G5,
        31 => Square::H5,
        32 => Square::A4,
        33 => Square::B4,
        34 => Square::C4,
        35 => Square::D4,
        36 => Square::E4,
        37 => Square::F4,
        38 => Square::G4,
        39 => Square::H4,
        40 => Square::A3,
        41 => Square::B3,
        42 => Square::C3,
        43 => Square::D3,
        44 => Square::E3,
        45 => Square::F3,
        46 => Square::G3,
        47 => Square::H3,
        48 => Square::A2,
        49 => Square::B2,
        50 => Square::C2,
        51 => Square::D2,
        52 => Square::E2,
        53 => Square::F2,
        54 => Square::G2,
        55 => Square::H2,
        56 => Square::A1,
        57 => Square::B1,
        58 => Square::C1,
        59 => Square::D1,
        60 => Square::E1,
        61 => Square::F1,
        62 => Square::G1,
        63 => Square::H1,
        _ => panic!("Invalid square: {}", sq),
    }
}
