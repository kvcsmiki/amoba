use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct GameState {
    board: Vec<Vec<String>>,
    current_player: String,
    winner: Option<String>,
    game_over: bool,
}

#[derive(Serialize, Deserialize)]
struct Move {
    x: usize,
    y: usize,
}

#[derive(Serialize)]
struct GameStatus {
    board: Vec<Vec<String>>,
    winner: Option<String>,
    game_over: bool,
}

#[tokio::main]
async fn main() {
    let state = GameState {
        board: vec![vec!["".to_string(); 3]; 3],
        current_player: "X".to_string(),
        winner: None,
        game_over: false,
    };

    let game_state = Arc::new(Mutex::new(state));

    let get_status = warp::path("status")
        .and(warp::get())
        .and(with_state(game_state.clone()))
        .and_then(handle_get_status);

    let make_move = warp::path("move")
        .and(warp::post())
        .and(with_state(game_state.clone()))
        .and(warp::body::json())
        .and_then(handle_make_move);

    let restart = warp::path("restart")
        .and(warp::post())
        .and(with_state(game_state.clone()))
        .and_then(handle_restart);

    let static_files = warp::fs::dir("static");

    let routes = get_status.or(make_move).or(restart).or(static_files);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_state(
    state: Arc<Mutex<GameState>>,
) -> impl Filter<Extract = (Arc<Mutex<GameState>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn handle_get_status(
    state: Arc<Mutex<GameState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().unwrap();
    let status = GameStatus {
        board: state.board.clone(),
        winner: state.winner.clone(),
        game_over: state.game_over,
    };
    Ok(warp::reply::json(&status))
}

async fn handle_make_move(
    state: Arc<Mutex<GameState>>,
    mov: Move,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut state = state.lock().unwrap();
    if state.game_over {
        return Ok(warp::reply::with_status(
            "Game Over",
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    if state.board[mov.x][mov.y] == "" {
        state.board[mov.x][mov.y] = state.current_player.clone();
        if let Some(winner) = check_winner(&state.board) {
            state.winner = Some(winner);
            state.game_over = true;
        } else if is_draw(&state.board) {
            state.winner = None;
            state.game_over = true;
        } else {
            state.current_player = if state.current_player == "X" {
                "O".to_string()
            } else {
                "X".to_string()
            };
        }
        Ok(warp::reply::with_status(
            "Valid move",
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Invalid move",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    }
}

async fn handle_restart(
    state: Arc<Mutex<GameState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut state = state.lock().unwrap();
    state.board = vec![vec!["".to_string(); 3]; 3];
    state.current_player = "X".to_string();
    state.winner = None;
    state.game_over = false;
    Ok(warp::reply::with_status(
        "Game restarted",
        warp::http::StatusCode::OK,
    ))
}

fn check_winner(board: &Vec<Vec<String>>) -> Option<String> {
    let lines = [
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],

        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],

        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    for line in &lines {
        let [a, b, c] = *line;
        if board[a.0][a.1] != ""
            && board[a.0][a.1] == board[b.0][b.1]
            && board[a.0][a.1] == board[c.0][c.1]
        {
            return Some(board[a.0][a.1].clone());
        }
    }
    None
}

fn is_draw(board: &Vec<Vec<String>>) -> bool {
    board.iter().all(|row| row.iter().all(|cell| cell != ""))
}