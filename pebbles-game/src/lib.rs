use gstd::{exec, msg, prelude::*};
use pebbles_game_io::{DifficultyLevel, GameState, PebblesAction, PebblesEvent, PebblesInit, Player};

static mut GAME_STATE: Option<GameState> = None;

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to decode PebblesAction");

    let mut state = unsafe { GAME_STATE.clone().expect("Game state not initialized") };

    match action {
        PebblesAction::Turn(n) => {
            if n < 1 || n > state.max_pebbles_per_turn {
                msg::reply(PebblesEvent::Won(state.current_player.clone()), 0)
                    .expect("Unable to reply");
                return;
            }

            if n >= state.pebbles_remaining {
                state.pebbles_remaining = 0;
                state.winner = Some(state.current_player.clone());
                msg::reply(PebblesEvent::Won(state.current_player.clone()), 0).expect("Unable to reply");
            } else {
                state.pebbles_remaining -= n;
                state.current_player = next_player(&state.current_player);
                let program_turn = program_move(&state);
                if program_turn >= state.pebbles_remaining {
                    state.pebbles_remaining = 0;
                    state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
                } else {
                    state.pebbles_remaining -= program_turn;
                    state.current_player = next_player(&state.current_player);
                    msg::reply(PebblesEvent::CounterTurn(program_turn), 0)
                        .expect("Unable to reply");
                }
            }
        }
        PebblesAction::GiveUp => {
            state.winner = Some(next_player(&state.current_player));
            msg::reply(PebblesEvent::Won(next_player(&state.current_player)), 0)
                .expect("Unable to reply");
        }
        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            let first_player = if get_random_u32() % 2 == 0 {
                Player::User
            } else {
                Player::Program
            };

            let new_state = GameState {
                pebbles_count,
                max_pebbles_per_turn,
                pebbles_remaining: pebbles_count,
                difficulty,
                first_player: first_player.clone(),
                current_player: first_player,
                winner: None,
            };

            unsafe {
                GAME_STATE = Some(new_state.clone());
            }
            msg::reply(PebblesEvent::Won(new_state.first_player.clone()), 0)
                .expect("Unable to reply");
        }
    }

    unsafe { GAME_STATE = Some(state) };
}

fn next_player(current: &Player) -> Player {
    match current {
        Player::User => Player::Program,
        Player::Program => Player::User,
    }
}

#[cfg(not(test))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[cfg(test)]
fn get_random_u32() -> u32 {
    1 // Fixed value for testing
}

fn program_move(state: &GameState) -> u32 {
    let max_take = state.max_pebbles_per_turn;
    match state.difficulty {
        DifficultyLevel::Easy => (get_random_u32() % max_take) + 1,
        DifficultyLevel::Hard => find_best_move(state.pebbles_remaining, max_take),
    }
}

fn find_best_move(pebbles_remaining: u32, max_take: u32) -> u32 {
    for take in 1..=max_take {
        if (pebbles_remaining - take) % (max_take + 1) == 0 {
            return take;
        }
    }
    1 // Default move if no winning move found
}

#[no_mangle]
extern "C" fn init() {
    let init_params: PebblesInit = msg::load().expect("Unable to decode PebblesInit");

    if init_params.pebbles_count == 0 || init_params.max_pebbles_per_turn == 0 {
        panic!("Invalid initialization parameters");
    }

    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    let mut state = GameState {
        pebbles_count: init_params.pebbles_count,
        max_pebbles_per_turn: init_params.max_pebbles_per_turn,
        pebbles_remaining: init_params.pebbles_count,
        difficulty: init_params.difficulty,
        first_player: first_player.clone(),
        current_player: first_player.clone(),
        winner: None,
    };

    if first_player == Player::Program {
        let program_turn = program_move(&state);
        state.pebbles_remaining -= program_turn;
        state.current_player = next_player(&state.current_player);
        msg::reply(PebblesEvent::CounterTurn(program_turn), 0)
            .expect("Unable to reply");
    } else {
        msg::reply(PebblesEvent::Won(state.current_player.clone()), 0)
            .expect("Unable to reply");
    }

    unsafe {
        GAME_STATE = Some(state);
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = unsafe { GAME_STATE.clone().expect("Game state not initialized") };
    msg::reply(state, 0).expect("Unable to reply");
}
