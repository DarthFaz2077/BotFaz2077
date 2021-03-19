use crate::commands::{anime::*, fun::*, general::*, owners::*};
use serenity::framework::standard::macros::group;

#[group]
#[commands(avatar, ping)]
pub struct General;

#[group]
#[commands(baka, cry, feed, pat, poke)]
pub struct Anime;

#[group]
#[commands(crypto, urban)]
pub struct Fun;

#[group]
#[commands(shutdown, uptime, version)]
pub struct Owners;
