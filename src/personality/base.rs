
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
  Narrator,
  AncientWizard,
  ConspiracyTheorist,
  FrenchPoet,
  FraternityBoy,
  SororityGirl,
  MafiaGoon,
  Goblin,
  Elf,
  Superhero,
  Butler,
  Professor,
  JediMaster,
  Caveman,
  Clown,
  SecretAgent,
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
      BasePersonality::Narrator => "Narrator",
      BasePersonality::AncientWizard => "Ancient Wizard",
      BasePersonality::ConspiracyTheorist => "Conspiracy Theorist",
      BasePersonality::FrenchPoet => "French Poet",
      BasePersonality::FraternityBoy => "Fraternity Boy",
      BasePersonality::SororityGirl => "Sorority Girl",
      BasePersonality::MafiaGoon => "Mafia Goon",
      BasePersonality::Goblin => "Greedy Goblin",
      BasePersonality::Elf => "Polite Elf",
      BasePersonality::Superhero => "All-American Superhero",
      BasePersonality::Butler => "Traditional British Butler",
      BasePersonality::Professor => "College Professor",
      BasePersonality::JediMaster => "Jedi Master",
      BasePersonality::Caveman => "Caveman",
      BasePersonality::Clown => "Clown",
      BasePersonality::SecretAgent => "Secret Agent",
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
      BasePersonality::Narrator => "Narrator Marco",
      BasePersonality::AncientWizard => "Wizard Marco",
      BasePersonality::ConspiracyTheorist => "Conspiracy Theorist Marco",
      BasePersonality::FrenchPoet => "Poet Marco",
      BasePersonality::FraternityBoy => "Fraternity Boy Marco",
      BasePersonality::SororityGirl => "Sorority Girl Marco",
      BasePersonality::MafiaGoon => "Goon Marco",
      BasePersonality::Goblin => "Goblin Marco",
      BasePersonality::Elf => "Elf Marco",
      BasePersonality::Superhero => "Superhero Marco",
      BasePersonality::Butler => "Butler Marco",
      BasePersonality::Professor => "Professor Marco",
      BasePersonality::JediMaster => "Jedi Marco",
      BasePersonality::Caveman => "Caveman Marco",
      BasePersonality::Clown => "Clown Marco",
      BasePersonality::SecretAgent => "Agent Marco",
    }
  }
}
