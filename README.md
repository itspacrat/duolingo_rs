# duolingo_rs
## about
a minimal (as it can be) wrapper for the duolingo api.

## current features
duolingo_rs *can* currently:
- do nothing. the api changes require this library be refactored; this being the case i've decided to put this library and itspacrat/da-oxide on an indefinite hiatus.
<s>
- log in with a reqwest::Client and set session cookies,
- fetch userdata for a Vec of users (only streaks implimented thus far)
  </s>

## planned features
- single-user manifest fetching
- more datapoint fetching
  - current language
  - current xp
- clientside data crunching & comparison

## documentation
[docs.rs](https://docs.rs/duolingo_rs/latest/duolingo_rs/fn.fetch_streak_map.html)
