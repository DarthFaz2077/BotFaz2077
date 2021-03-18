use serenity::framework::standard::macros::group;

use crate::commands::{fun::*, general::*, owner::*};

#[group]
#[commands(poke, feed, pat)]
struct Fun;

#[group]
#[commands(ping, urban, crypto, avatar)]
struct General;

#[group]
#[commands(shutdown, uptime, version)]
struct Owner;
