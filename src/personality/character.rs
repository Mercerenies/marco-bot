
//! Base character archetypes.

use super::base::BasePersonality;

use strum::{VariantArray, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum BaseCharacter {
  #[strum(serialize = "Clint Eastwood")]
  ClintEastwood,
  #[strum(serialize = "Dr. Doofenshmirtz")]
  DrDoofenshmirtz,
  #[strum(serialize = "Dr. Horrible")]
  DrHorrible,
  #[strum(serialize = "Jack Sparrow")]
  JackSparrow,
  #[strum(serialize = "Captain Hook")]
  CaptainHook,
  #[strum(serialize = "Dug (from Up)")]
  Dug,
  #[strum(serialize = "Sharpay (High School Musical)")]
  Sharpay,
  #[strum(serialize = "Paulie Walnuts")]
  PaulieWalnuts,
  #[strum(serialize = "Luca Brasi")]
  LucaBrasi,
  #[strum(serialize = "Gollum")]
  Gollum,
  #[strum(serialize = "Dobby (Harry Potter)")]
  Dobby,
  #[strum(serialize = "Metro-Man (from Megamind)")]
  MetroMan,
  #[strum(serialize = "Superman")]
  Superman,
  #[strum(serialize = "Alfred (from Batman)")]
  Alfred,
  #[strum(serialize = "Doc Brown")]
  DocBrown,
  #[strum(serialize = "Obi-Wan Kenobi")]
  ObiWanKenobi,
  #[strum(serialize = "Yoda")]
  Yoda,
  #[strum(serialize = "Fred Flintstone")]
  FredFlintstone,
  #[strum(serialize = "The Joker")]
  TheJoker,
  #[strum(serialize = "James Bond")]
  JamesBond,
}

impl BaseCharacter {
  pub fn class(&self) -> BasePersonality {
    match self {
      BaseCharacter::ClintEastwood => BasePersonality::Cowboy,
      BaseCharacter::DrDoofenshmirtz => BasePersonality::MadScientist,
      BaseCharacter::DrHorrible => BasePersonality::MadScientist,
      BaseCharacter::JackSparrow => BasePersonality::PirateCaptain,
      BaseCharacter::CaptainHook => BasePersonality::PirateCaptain,
      BaseCharacter::Dug => BasePersonality::Dog,
      BaseCharacter::Sharpay => BasePersonality::SororityGirl,
      BaseCharacter::PaulieWalnuts => BasePersonality::MafiaGoon,
      BaseCharacter::LucaBrasi => BasePersonality::MafiaGoon,
      BaseCharacter::Gollum => BasePersonality::Goblin,
      BaseCharacter::Dobby => BasePersonality::Elf,
      BaseCharacter::MetroMan => BasePersonality::Superhero,
      BaseCharacter::Superman => BasePersonality::Superhero,
      BaseCharacter::Alfred => BasePersonality::Butler,
      BaseCharacter::DocBrown => BasePersonality::Professor,
      BaseCharacter::ObiWanKenobi => BasePersonality::JediMaster,
      BaseCharacter::Yoda => BasePersonality::JediMaster,
      BaseCharacter::FredFlintstone => BasePersonality::Caveman,
      BaseCharacter::TheJoker => BasePersonality::Clown,
      BaseCharacter::JamesBond => BasePersonality::SecretAgent,
    }
  }
}
