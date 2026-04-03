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
- **Unnecessary clone**: `range.clone()` in `main.rs` is unnecessary since `Range<i32>` implements `Copy`.
- **Slice vs Vec**: `print_table` takes `&Vec<Player>`; idiomatic to take `&[Player]` for more flexibility.

## Structural and Design Issues
- **Module organization**: Code is well-modularized with `utils/inputs.rs` and `utils/player.rs`, which is good.
- **Missing features**: README mentions saving scores to file, but this is not implemented yet.
- **Input validation**: `get_range` checks `from >= to`, but should be `from < to` for valid ranges.
- **Player input**: In `Player::new`, name is asked after attempts, but logically should be before playing.
- **Debug output**: `dbg!(system_guess);` in `single_play` should be removed for production.

## Documentation and Typos
- **README.md**: Numerous typos: "steriods" → "steroids", "fr me atleast" → "for me at least", "betther" → "better", "mkaes" → "makes", "tehir" → "their", "smalleer" → "smaller", "teh" → "the", "socores" → "scores".
- **progress.md**: "strctured" → "structured".
- **Code comments**: Limited comments; add doc comments for public functions.

## Performance and Best Practices
- **No tests**: No unit or integration tests present. Add tests for functions like `get_user_guess`, `single_play`.
- **Dependencies**: Uses `colored`, `fastrand`, `tabled` – appropriate, but ensure versions are up-to-date.
- **Error messages**: Some error messages have typos, e.g., "teh number correctly" → "the number correctly", "enter you guess" → "enter your guess".

## Future Improvements
- Implement file saving for leaderboard as described in README.
- Add multiplayer/online features as mentioned.
- Improve UI with more colors and formatting.
- Add configuration for max attempts, range limits.
- Use `anyhow` or `thiserror` for better error handling.