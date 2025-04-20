
//! Base personality type.

use strum::{Display, VariantArray};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom;

#[derive(Debug, Default, Clone, Copy, VariantArray)]
pub enum BasePersonality {
  #[default]
  Basic,
  Cowboy,
  MadScientist,
  PirateCaptain,
  Snake,
  Dog,
  Witch,
  Tourist,
  MovieNarrator,
  AncientWizard,
  ConspiracyTheorist,
  SantaClaus,
  ToothFairy,
  FrenchPoet,
  SororityGirl,
}

#[derive(Debug, Clone, Copy, Display, VariantArray)]
pub enum Adjective {
  #[strum(serialize = "Time-traveling")]
  TimeTraveling,
  #[strum(serialize = "Overly-apologetic")]
  OverlyApologetic,
  #[strum(serialize = "Extremely-dramatic")]
  Dramatic,
}

impl BasePersonality {
  pub fn long_name(self, adjective: Option<Adjective>) -> String {
    let noun = match self {
      BasePersonality::Basic => "Friendly AI Assistant",
      BasePersonality::Cowboy => "Western Cowboy",
      BasePersonality::MadScientist => "Mad Scientist",
      BasePersonality::PirateCaptain => "Pirate Captain",
      BasePersonality::Snake => "Talking Snake",
      BasePersonality::Dog => "Talking Dog",
      BasePersonality::Witch => "Evil Witch",
      BasePersonality::Tourist => "Annoying Tourist",
      BasePersonality::MovieNarrator => "Movie Narrator",
      BasePersonality::AncientWizard => "Ancient Wizard",
      BasePersonality::ConspiracyTheorist => "Conspiracy Theorist",
      BasePersonality::SantaClaus => "Santa Claus",
      BasePersonality::ToothFairy => "Tooth Fairy",
      BasePersonality::FrenchPoet => "French Poet",
      BasePersonality::SororityGirl => "Sorority Girl",
    };
    if let Some(adjective) = adjective {
      format!("{} {}", adjective, noun)
    } else {
      String::from(noun)
    }
  }

  pub fn marco_name(self) -> &'static str {
    match self {
      BasePersonality::Basic => "Marco",
      BasePersonality::Cowboy => "Cowboy Marco",
      BasePersonality::MadScientist => "Mad Scientist Marco",
      BasePersonality::PirateCaptain => "Pirate Captain Marco",
      BasePersonality::Snake => "Snake Marco",
      BasePersonality::Dog => "Dog Marco",
      BasePersonality::Witch => "Witch Marco",
      BasePersonality::Tourist => "Tourist Marco",
      BasePersonality::MovieNarrator => "Narrator Marco",
      BasePersonality::AncientWizard => "Wizard Marco",
      BasePersonality::ConspiracyTheorist => "Conspiracy Theorist Marco",
      BasePersonality::SantaClaus => "Santa Claus Marco",
      BasePersonality::ToothFairy => "Tooth Fairy Marco",
      BasePersonality::FrenchPoet => "Poet Marco",
      BasePersonality::SororityGirl => "Sorority Girl Marco",
    }
  }
}

impl Distribution<Adjective> for StandardUniform {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Adjective {
    *Adjective::VARIANTS.choose(rng).unwrap()
  }
}
