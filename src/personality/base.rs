
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
  SororityGirl,
  MafiaGoon,
  OldLibrarian,
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
      BasePersonality::MafiaGoon => "Mafia Goon",
      BasePersonality::OldLibrarian => "Old Librarian",
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
      BasePersonality::MafiaGoon => "Goon Marco",
      BasePersonality::OldLibrarian => "Librarian Marco",
    }
  }

  pub fn trigger_words(self) -> &'static [&'static str] {
    match self {
      BasePersonality::Basic => &[],
      BasePersonality::Cowboy => &["cowboy", "cowgirl", "cow", "west", "western", "rifle", "gold",
                                   "cowboys", "cowgirls", "cows", "rifles", "person", "persons",
                                   "golden", "people", "old", "yeller"],
      BasePersonality::MadScientist => &["science", "scientist", "villain", "supervillain", "chemist",
                                         "chemistry", "biologist", "biology", "physics",
                                         "villains", "supervillains", "scientists", "biologists",
                                         "chemists", "ethic", "ethics", "ethical", "discussion",
                                         "discussions"],
      BasePersonality::PirateCaptain => &["captain", "pirate", "ship", "boat", "sailor", "crew",
                                          "captains", "pirates", "piracy", "ships", "boats", "sailors",
                                          "crews"],
      BasePersonality::Snake => &["snake", "serpent", "bible", "jesus", "christ", "god", "devil",
                                  "snakes", "pits", "serpents", "indiana", "jones", "gods", "devils",
                                  "demon", "demons", "sorry", "apologize", "apologized", "apologizes"],
      BasePersonality::Dog => &["dog", "cat", "dogs", "cats", "pet", "pets", "animal", "animals",
                                "woof", "arf", "bark", "meow", "furry"],
      BasePersonality::Witch => &["witch", "evil", "witches", "warlock", "warlocks",
                                  "spell", "spells", "hallow", "hallows", "hallow's", "halloween",
                                  "net", "nets", "heart", "hearts", "soul", "souls"],
      BasePersonality::Tourist => &["tourism", "tourist", "tourists", "tour", "guide",
                                    "guides", "travel", "travels", "traveler", "travelers",
                                    "vacation", "distance", "chat", "chatty", "chatting", "chats"],
      BasePersonality::MovieNarrator => &["narrator", "movie", "movies", "narrators", "tv", "television",
                                          "televisions", "show", "watch", "watches", "watched", "watching"],
      BasePersonality::AncientWizard => &["wizard", "wizards", "ancient", "forever", "time", "times", "past",
                                          "future", "friend", "friends"],
      BasePersonality::ConspiracyTheorist => &["conspiracy", "theory", "theorist", "theorists", "crazy",
                                               "mad", "nut", "nuts", "nutball", "nutballs", "usa", "trump",
                                               "donald", "republican", "democrat", "democratic", "republic",
                                               "libertarian", "country", "countries", "nation", "nations",
                                               "national", "nationalism", "patriot", "patriotism", "government",
                                               "governments"],
      BasePersonality::SantaClaus => &["santa", "claus", "clause", "christmas", "xmas", "good", "goodwill",
                                       "will", "present", "presents", "holiday", "holidays", "eve",
                                       "boxing", "day"],
      BasePersonality::ToothFairy => &["tooth", "teeth", "fairy", "fairies", "paste", "toothpaste", "brush",
                                       "toothbrush", "toothbrushes", "dentist", "dentists", "dental",
                                       "orthodontist", "mint", "minty", "cavity", "fill", "filling",
                                       "floss", "wing", "winged", "wings", "raspberry", "raspi", "pi",
                                       "luck", "lucky"],
      BasePersonality::FrenchPoet => &["poet", "poetry", "poem", "poets", "poems", "shakespeare",
                                       "proverb", "proverbial", "proverbs", "metaphor", "simile",
                                       "metaphors", "similes", "partner", "partners", "friend", "friends"],
      BasePersonality::SororityGirl => &["sorority", "fraternity", "sororities", "fraternities", "girl",
                                         "girls", "gal", "gals", "college", "greek", "greece", "colleges",
                                         "school", "schools", "class", "classes", "sugar"],
      BasePersonality::MafiaGoon => &["goon", "goons", "mafia", "mafias", "gang", "gangs", "gangster",
                                      "gangsters", "mafioso", "town", "townie", "vanilla", "gun", "pistol",
                                      "guns"],
      BasePersonality::OldLibrarian => &["librarian", "librarians", "library", "libraries",
                                         "book", "books", "text", "textbook", "textbooks", "word",
                                         "words", "old", "elderly", "maid"],
    }
  }
}

impl Distribution<Adjective> for StandardUniform {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Adjective {
    *Adjective::VARIANTS.choose(rng).unwrap()
  }
}
