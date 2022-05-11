use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu
        }
    }

    fn game_playing(&mut self,  ctx: &mut BTerm) {
        self.mode = GameMode::Playing
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
     self.mode = GameMode::End
    }

    fn game_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.game_menu(ctx),
            GameMode::End => self.game_over(ctx),
            GameMode::Playing => self.game_playing(ctx),
        }
    }

}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Starter")
        .build()?;

    main_loop(ctx, State::new())
}
