trait Game {
    fn name(&self) -> String;
}

#[derive(Debug)]
enum BoardGame {
    Chess,
    Monopoly,
}
impl Game for BoardGame {
    fn name(&self) -> String {
        match self {
            BoardGame::Chess => String::from("Chess"),
            BoardGame::Monopoly => String::from("Monopoly"),
        }
    }
}

#[derive(Debug)]
enum VideoGame {
    PlayStation,
    Xbox,
}
impl Game for VideoGame {
    fn name(&self) -> String {
        match self {
            Self::PlayStation => String::from("PlayStation"),
            Self::Xbox => String::from("Xbox"),
        }
    }
}

struct PlayRoom<T: Game> {
    game: T,
}
impl<T: Game> PlayRoom<T> {
    fn game_info(&self) {
        println!("{}", self.game.name());
    }
}

fn main() {
    let board_game = PlayRoom {
        game: BoardGame::Chess,
    };
    let video_game = PlayRoom {
        game: VideoGame::PlayStation,
    };
    board_game.game_info();
    video_game.game_info();
}
