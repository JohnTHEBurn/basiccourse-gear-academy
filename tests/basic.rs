use gstd::{prelude::*, ActorId};
use gtest::{Program, System};
use pebbles_game_io::{DifficultyLevel, GameState, PebblesAction, PebblesEvent, PebblesInit, Player};

#[test]
fn test_initialization() {
    let system = System::new();
    let program = Program::current(&system);

    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send(ActorId::from(1), init_message);

    let state: GameState = program.read_state().expect("Unable to read state");

    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert_eq!(state.pebbles_remaining, 15);
    assert!(state.winner.is_none());
}

#[test]
fn test_easy_level_program_turn() {
    let system = System::new();
    let program = Program::current(&system);

    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send(ActorId::from(1), init_message);

    let state: GameState = program.read_state().expect("Unable to read state");
    if state.first_player == Player::Program {
        let expected_turn = 1;
        assert_eq!(state.pebbles_remaining, 15 - expected_turn);
    }
}

#[test]
fn test_hard_level_program_turn() {
    let system = System::new();
    let program = Program::current(&system);

    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send(ActorId::from(1), init_message);

    let state: GameState = program.read_state().expect("Unable to read state");
    if state.first_player == Player::Program {
        let expected_turn = 1;
        assert_eq!(state.pebbles_remaining, 15 - expected_turn);
    }
}

#[test]
fn test_turn_action() {
    let system = System::new();
    let program = Program::current(&system);

    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send(ActorId::from(1), init_message);

    let state: GameState = program.read_state().expect("Unable to read state");

    if state.first_player == Player::User {
        let user_turn = PebblesAction::Turn(2);
        program.send(ActorId::from(1), user_turn);

        let updated_state: GameState = program.read_state().expect("Unable to read state");
        assert_eq!(updated_state.pebbles_remaining, state.pebbles_remaining - 2);
    }
}

#[test]
fn test_invalid_turn_action() {
    let system = System::new();
    let program = Program::current(&system);

    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send(ActorId::from(1), init_message);

    let state: GameState = program.read_state().expect("Unable to read state");

    if state.first_player == Player::User {
        let invalid_turn = PebblesAction::Turn(4); // Invalid, as max is 3
        program.send(ActorId::from(1), invalid_turn);

        let event: PebblesEvent = program.read_message().expect("Unable to read event");
        if let PebblesEvent::Won(winner) = event {
            assert_eq!(winner, state.current_player);
        } else {
            panic!("Expected PebblesEvent::Won");
        }
    }
}
