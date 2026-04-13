# Issues and Improvements for Guessing on Steroids
## Compilation and Build Issues
- **Cargo.toml edition**: `edition = "2024"` is invalid. Rust editions are 2015, 2018, 2021. Change to `"2021"` or the latest stable edition.
- **No compilation errors**: `cargo check` passes, but the edition issue may cause problems in future Rust versions.

## Code Quality and Idiomatic Rust Issues
- **Error handling**: Functions like `get_user_guess()` return `Result<i32, ()>` with unit error type, which is not idiomatic. Use proper error types like `std::io::Error` or custom enums.
- **Panic usage**: `get_user_guess()` panics after 5 failed attempts instead of returning an error. This is not idiomatic; handle errors gracefully.
- **Unwrap usage**: Multiple `unwrap()` calls (e.g., in `single_play`, `get_input`) can cause panics. Use proper error handling with `?` or `match`.
- **Generic `get_input` function**: The generic `get_input<T>` is misused. In `get_user_guess`, it's used to get a `String` then parsed again, which is redundant. Consider making `get_input` return `String` and parse elsewhere for clarity.
- **Infinite loops**: `single_play` has an infinite loop that only breaks on correct guess. While functional, consider adding a maximum attempt limit to prevent infinite play.
- **Variable naming**: Typos like "atempt" (should be "attempt"), "Wlecome" (should be "Welcome").
- **String handling**: In `main.rs`, `decision.trim() != "y"` is case-sensitive and only checks "y". Make it case-insensitive and accept "yes" or "y".
- **Unnecessary clone**: `range.clone()` in `main.rs` is unnecessary since `Range<i32>` implements `Copy`. Also occurs in `game_modes.rs` line 10: `let system_guess = fastrand::i32(range.clone());`
- **Slice vs Vec**: `create_leaderboard` takes `&Vec<Player>`; idiomatic to take `&[Player]` for more flexibility.
- **Redundant field assignments**: In `structs.rs` line 17, `name: name` uses redundant syntax; could use shorthand `name,`
- **Dead code**: `structs.rs` line 4 has commented-out import `// use std::fmt::Display;` that should be removed.
- **Spacing issue**: `structs.rs` line 24 has non-standard spacing: `table :&Vec<Player>` should be `table: &Vec<Player>`

## Structural and Design Issues
- **Module organization**: Code is well-modularized with `utils/inputs.rs` and `utils/structs.rs`, which is good.
- **Missing features**: README mentions saving scores to file, but this is not implemented yet.
- **Input validation**: `get_range` checks `from >= to`, but should be `from < to` for valid ranges. Additionally, `single_play` doesn't validate that user guesses fall within the specified range.
- **Player input**: In `Player::new`, name is asked after attempts, but logically should be before playing.
- **Debug output**: Commented-out `!(system_guess);` in `single_play` line 10 should be removed.
- **Incomplete function**: `simgle_pay_hard` in `game_modes.rs` (lines 45-50) is incomplete with typos in the name ("simgle" → "single", "pay" → "play") and only returns `Err` immediately without implementation.
- **Function design redundancy**: `get_input<T>` generic is used indirectly in `get_user_guess`; it retrieves a String then parses it separately, which is redundant and unclear.

## Documentation and Typos
- **README.md**: Numerous typos: "steriods" → "steroids", "fr me atleast" → "for me at least", "betther" → "better", "mkaes" → "makes", "tehir" → "their", "smalleer" → "smaller", "teh" → "the", "socores" → "scores".
- **progress.md**: "strctured" → "structured".
- **Code comments**: Limited comments; add doc comments for public functions.
- **Code typos in source**:
  - `main.rs` line 8: "Wlecome" → "Welcome"
  - `main.rs` line 21: Duplicate words "be be wrong" → "be wrong"
  - `game_modes.rs` line 33: Variable name "atempt" → "attempt"
  - `game_modes.rs` line 45: Function name "simgle_pay_hard" → "single_play_hard"
  - `inputs.rs` line 16: "enter you guess" → "enter your guess"
  - `inputs.rs` line 50: Error message formatting "unable to parse ,defaulting" needs space after comma

## Performance and Best Practices
- **No tests**: No unit or integration tests present. Add tests for functions like `get_user_guess`, `single_play`.
- **Dependencies**: Uses `colored`, `fastrand`, `tabled` – appropriate, but ensure versions are up-to-date.
- **Error messages**: Some error messages have typos and formatting issues (see Documentation and Typos section).
- **Error handling robustness**: 
  - `get_user_guess()` returns error after 5 attempts but doesn't provide feedback on which attempt the user is on.
  - `write_to_file()` error messages (lines 82-88) lack context about which specific operation failed.
  - No input timeout mechanism for user interactions.

## Future Improvements
- Implement file saving for leaderboard as described in README.
- Add multiplayer/online features as mentioned.
- Improve UI with more colors and formatting.
- Add configuration for max attempts, range limits.
- Use `anyhow` or `thiserror` for better error handling.