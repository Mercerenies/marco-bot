
//! Base personality type.

use strum::{Display, VariantArray};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom;

pub const RANDOM_TRIGGER_WORDS: &[&str] = &[
  "random", "randoms", "chaos", "chaotic", "unpredictable", "unknown",
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, VariantArray)]
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
  FraternityBoy,
  SororityGirl,
  MafiaGoon,
  OldLibrarian,
  RadioHost,
  Goblin,
  Superhero,
  Butler,
  Professor,
  Yoda,
}

#[derive(Debug, Clone, Copy, Display, VariantArray)]
pub enum Adjective {
  #[strum(serialize = "Time-traveling")]
  TimeTraveling,
  #[strum(serialize = "Overly-apologetic")]
  OverlyApologetic,
  #[strum(serialize = "Extremely-dramatic")]
  Dramatic,
  #[strum(serialize = "Sleepy")]
  Sleepy,
  #[strum(serialize = "Soft-spoken")]
  SoftSpoken,
  #[strum(serialize = "Optimistic")]
  Optimistic,
  #[strum(serialize = "Tea-obsessed")]
  TeaObsessed,
}

impl BasePersonality {
  pub fn long_name(self, adjective: Option<Adjective>) -> String {
    let noun = match self {
      BasePersonality::Basic => "Friendly AI Assistant",
      BasePersonality::Cowboy => "Western Cowboy",
      BasePersonality::MadScientist => "Insane Mad Scientist",
      BasePersonality::PirateCaptain => "Pirate Captain",
      BasePersonality::Snake => "Manipulative Talking Snake",
      BasePersonality::Dog => "Hyperactive Talking Dog",
      BasePersonality::Witch => "Evil Witch",
      BasePersonality::Tourist => "Annoying Foreign Tourist",
      BasePersonality::MovieNarrator => "Twilight Zone Narrator",
      BasePersonality::AncientWizard => "Ancient, Wise Wizard",
      BasePersonality::ConspiracyTheorist => "Crazy Conspiracy Theorist",
      BasePersonality::SantaClaus => "Santa Claus",
      BasePersonality::ToothFairy => "Tooth Fairy",
      BasePersonality::FrenchPoet => "Arrogant French Poet",
      BasePersonality::FraternityBoy => "Drunk Fraternity Boy",
      BasePersonality::SororityGirl => "Excitable Sorority Girl",
      BasePersonality::MafiaGoon => "Sleazy Mafia Goon",
      BasePersonality::OldLibrarian => "Old Maid Librarian",
      BasePersonality::RadioHost => "Over-the-top Radio Host",
      BasePersonality::Goblin => "Greedy, Unintelligent Goblin",
      BasePersonality::Superhero => "All-American Superhero",
      BasePersonality::Butler => "Traditional British Butler",
      BasePersonality::Professor => "Out-of-Touch College Professor",
      BasePersonality::Yoda => "Jedi Master Yoda",
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
      BasePersonality::FraternityBoy => "Fraternity Boy Marco",
      BasePersonality::SororityGirl => "Sorority Girl Marco",
      BasePersonality::MafiaGoon => "Goon Marco",
      BasePersonality::OldLibrarian => "Librarian Marco",
      BasePersonality::RadioHost => "Radio Host Marco",
      BasePersonality::Goblin => "Goblin Marco",
      BasePersonality::Superhero => "Superhero Marco",
      BasePersonality::Butler => "Butler Marco",
      BasePersonality::Professor => "Professor Marco",
      BasePersonality::Yoda => "Yoda Marco",
    }
  }

  pub fn trigger_words(self) -> &'static [&'static str] {
    match self {
      BasePersonality::Basic => &[],
      BasePersonality::Cowboy => &["cowboy", "cowgirl", "western"],
      BasePersonality::MadScientist => &["science", "scientist", "supervillain", "chemist",
                                         "chemistry", "biologist", "biology", "physics",
                                         "villains", "supervillains", "scientists", "biologists",
                                         "chemists"],
      BasePersonality::PirateCaptain => &["captain", "pirate", "captains", "pirates", "piracy"],
      BasePersonality::Snake => &["snake", "serpent", "snakes", "serpents"],
      BasePersonality::Dog => &["dog", "dogs", "woof", "arf"],
      BasePersonality::Witch => &["witch", "witches", "warlock", "warlocks", "halloween"],
      BasePersonality::Tourist => &["tourism", "tourist", "tourists"],
      BasePersonality::MovieNarrator => &["narrator", "narrators"],
      BasePersonality::AncientWizard => &["wizard", "wizards"],
      BasePersonality::ConspiracyTheorist => &["conspiracy", "conspiracies"],
      BasePersonality::SantaClaus => &["santa", "claus", "christmas"],
      BasePersonality::ToothFairy => &["tooth", "teeth", "fairy", "fairies", "dentist", "dentists",
                                       "dentistry", "orthodontist", "orthodontists"],
      BasePersonality::FrenchPoet => &["poet", "poetry", "poem", "poets", "poems", "shakespeare"],
      BasePersonality::FraternityBoy => &["fraternity", "fraternities"],
      BasePersonality::SororityGirl => &["sorority", "sororities"],
      BasePersonality::MafiaGoon => &["goon", "goons", "mafia", "mafias", "gang", "gangs", "gangster",
                                      "gangsters", "mafioso"],
      BasePersonality::OldLibrarian => &["librarian", "librarians", "library", "libraries"],
      BasePersonality::RadioHost => &["radio", "radios"],
      BasePersonality::Goblin => &["goblin", "goblins", "orc", "orcs", "ogre", "ogres"],
      BasePersonality::Superhero => &["hero", "heroes", "superhero", "superheroes"],
      BasePersonality::Butler => &["butler", "butlers"],
      BasePersonality::Professor => &["professor", "professors"],
      BasePersonality::Yoda => &["yoda", "jedi"],
    }
  }
}

impl Distribution<Adjective> for StandardUniform {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Adjective {
    *Adjective::VARIANTS.choose(rng).unwrap()
  }
}
