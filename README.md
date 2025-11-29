# TMDS Demo

A repo providing a website demonstrating the TMDS algorithm used in DVI and
HDMI.

See the DVI spec section 3.2 for further information. 

## Repo Structure

The logic is written in Rust and compiled to WASM. (Just be thankful I didn't
use Haskell, Tristan.) The crate for this is available in the `alg`
subdirectory. The actual site is stored in `web` and built with Vite.

## TODOs

I'd like to give more introspection into what's actually happening; i.e., have
the `TmdsVal` return more info so we can see more intermediate states. I don't
know if I'll ever actually do this, but it would be nice if we could see 'in
real-time' how the procedure works.

I'd also like to keep track of overall statistics; i.e., number of transitions
and DC bias. It may also be interesting to implement and compare this to
8b/10b.
