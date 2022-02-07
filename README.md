# duolingo_rs
## about
a minimal (as it can be) wrapper for the duolingo api.

## current features
duolingo_rs *can* currently:
- log in with a reqwest::Client and set session cookies,
- fetch userdata for a Vec of users (only streaks implimented thus far)

## planned features
- single-user manifest fetching
- more datapoint fetching
  - current language
  - current xp
- <s>clientside data crunching & comparison</s> *out of scope for the lib at the time of writing*

## documentation
[docs.rs](https://docs.rs/duolingo_rs/latest/duolingo_rs)