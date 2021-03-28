use crate::commands::{anime::*, economy::*, fun::*, general::*, owners::*};
use serenity::framework::standard::macros::group;

#[group]
#[description = "Various commands."]
#[summary = "Various commands."]
#[commands(avatar, ping, profile)]
pub struct General;

#[group]
#[description = "Anime commands."]
#[summary = "Anime commands."]
#[commands(
    baka, cry, cuddle, feed, hug, kiss, laugh, lick, pat, poke, slap, smug, tickle
)]
pub struct Anime;

#[group]
#[description = "Economy commands."]
#[summary = "Economy commands."]
#[commands(daily)]
pub struct Economy;

#[group]
#[description = "Fun commands."]
#[summary = "Fun commands."]
#[commands(crypto, urban)]
pub struct Fun;

#[group]
#[description = "Bot administration commands."]
#[summary = "Bot administration commands."]
#[owners_only]
#[commands(shutdown, uptime, version)]
pub struct Owners;
