pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let players = vec!['X','O'];
    for player in players {
        if diagonals(player, table) || horizontal(player, table) || vertical(player, table) {
            return format!("player {} won",player);
        }
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {

    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true
    }

    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    false
}


pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }

    false
}