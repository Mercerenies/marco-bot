
//! Base personality type.

use strum::VariantArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantArray)]
pub enum BasePersonality {
  Cowboy,
  MadScientist,
  PirateCaptain,
  Snake,
  Dog,
  Cat,
  Witch,
  MovieNarrator,
  AncientWizard,
  ConspiracyTheorist,
  FrenchPoet,
  FraternityBoy,
  SororityGirl,
  MafiaGoon,
  RadioHost,
  Goblin,
  Superhero,
  Butler,
  Professor,
  JediMaster,
}

impl BasePersonality {
  pub fn long_name(self) -> &'static str {
    match self {
      BasePersonality::Cowboy => "Wild West Cowboy",
      BasePersonality::MadScientist => "Mad Scientist",
      BasePersonality::PirateCaptain => "Pirate Captain",
      BasePersonality::Snake => "Talking Snake",
      BasePersonality::Dog => "Talking Dog",
      BasePersonality::Cat => "Talking Cat",
      BasePersonality::Witch => "Evil Witch",
      BasePersonality::MovieNarrator => "Movie Narrator",
      BasePersonality::AncientWizard => "Ancient Wizard",
      BasePersonality::ConspiracyTheorist => "Conspiracy Theorist",
      BasePersonality::FrenchPoet => "French Poet",
      BasePersonality::FraternityBoy => "Fraternity Boy",
      BasePersonality::SororityGirl => "Sorority Girl",
      BasePersonality::MafiaGoon => "Mafia Goon",
      BasePersonality::RadioHost => "Radio Host",
      BasePersonality::Goblin => "Greedy Goblin",
      BasePersonality::Superhero => "All-American Superhero",
      BasePersonality::Butler => "Traditional British Butler",
      BasePersonality::Professor => "College Professor",
      BasePersonality::JediMaster => "Jedi Master",
    }
  }

  pub fn marco_name(self) -> &'static str {
    match self {
      BasePersonality::Cowboy => "Cowboy Marco",
      BasePersonality::MadScientist => "Mad Scientist Marco",
      BasePersonality::PirateCaptain => "Pirate Captain Marco",
      BasePersonality::Snake => "Snake Marco",
      BasePersonality::Dog => "Dog Marco",
      BasePersonality::Cat => "Cat Marco",
      BasePersonality::Witch => "Witch Marco",
      BasePersonality::MovieNarrator => "Narrator Marco",
      BasePersonality::AncientWizard => "Wizard Marco",
      BasePersonality::ConspiracyTheorist => "Conspiracy Theorist Marco",
      BasePersonality::FrenchPoet => "Poet Marco",
      BasePersonality::FraternityBoy => "Fraternity Boy Marco",
      BasePersonality::SororityGirl => "Sorority Girl Marco",
      BasePersonality::MafiaGoon => "Goon Marco",
      BasePersonality::RadioHost => "Radio Host Marco",
      BasePersonality::Goblin => "Goblin Marco",
      BasePersonality::Superhero => "Superhero Marco",
      BasePersonality::Butler => "Butler Marco",
      BasePersonality::Professor => "Professor Marco",
      BasePersonality::JediMaster => "Jedi Marco",
    }
  }
}
