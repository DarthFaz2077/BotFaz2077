use crate::commands::{anime::*, general::*, owner::*};
use serenity::framework::standard::macros::group;

#[group]
#[commands(poke, feed, pat, baka)]
struct Anime;

#[group]
#[commands(ping, urban, crypto, avatar)]
struct General;

#[group]
#[commands(shutdown, uptime, version)]
struct Owner;
