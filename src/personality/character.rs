
//! Base character archetypes.

use super::base::BasePersonality;

use strum::{VariantArray, EnumString, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray, EnumString)]
pub enum BaseCharacter {
  #[strum(to_string = "Clint Eastwood", serialize = "eastwood")]
  ClintEastwood,
  #[strum(to_string = "Dr. Doofenshmirtz", serialize = "doof")]
  DrDoofenshmirtz,
  #[strum(to_string = "Dr. Horrible", serialize = "horrible")]
  DrHorrible,
  #[strum(to_string = "Jack Sparrow", serialize = "sparrow")]
  JackSparrow,
  #[strum(to_string = "Captain Hook", serialize = "hook")]
  CaptainHook,
  #[strum(to_string = "Dug (from Up)", serialize = "dug")]
  Dug,
  #[strum(to_string = "Scooby Doo", serialize = "scooby")]
  ScoobyDoo,
  #[strum(to_string = "Sharpay (High School Musical)", serialize = "sharpay")]
  Sharpay,
  #[strum(to_string = "Paulie Walnuts", serialize = "walnuts")]
  PaulieWalnuts,
  #[strum(to_string = "Luca Brasi", serialize = "brasi")]
  LucaBrasi,
  #[strum(to_string = "Gollum", serialize = "gollum")]
  Gollum,
  #[strum(to_string = "Dobby (Harry Potter)", serialize = "dobby")]
  Dobby,
  #[strum(to_string = "Metro-Man (from Megamind)", serialize = "metroman")]
  MetroMan,
  #[strum(to_string = "Superman", serialize = "superman")]
  Superman,
  #[strum(to_string = "Alfred (from Batman)", serialize = "alfred")]
  Alfred,
  #[strum(to_string = "Doc Brown", serialize = "docbrown")]
  DocBrown,
  #[strum(to_string = "Obi-Wan Kenobi", serialize = "obiwan")]
  ObiWanKenobi,
  #[strum(to_string = "Yoda", serialize = "yoda")]
  Yoda,
  #[strum(to_string = "Fred Flintstone", serialize = "fred")]
  FredFlintstone,
  #[strum(to_string = "The Joker", serialize = "joker")]
  TheJoker,
  #[strum(to_string = "James Bond", serialize = "jamesbond")]
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
      BaseCharacter::ScoobyDoo => BasePersonality::Dog,
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
