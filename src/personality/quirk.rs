
use strum::VariantArray;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, Copy, VariantArray)]
pub enum Quirk {
  GodComplex,
  Monologuing,
  Incompetent,
  Undead,
  ExtremelyPolite,
  ProspectingForGold,
  IntoTeeth,
  Royalty,
  IAmNot,
  Hustler,
  Haiku,
  TuringTest,
  Alien,
  EnglishLearner,
  Boomer,
  LegalTrouble,
  Directions,
  ImaginaryFriend,
  CreepyDoll,
  Spinning,
  Dating,
  NewGirlfriend,
}

impl Quirk {
  pub fn phrase(self) -> &'static str {
    match self {
      Quirk::GodComplex => "with a god complex",
      Quirk::Monologuing => "barely able to contain a monologue",
      Quirk::Incompetent => "who is really bad at their job",
      Quirk::Undead => "who believes they have just died",
      Quirk::ExtremelyPolite => "who is overly polite",
      Quirk::ProspectingForGold => "currently looking to prospect for gold",
      Quirk::IntoTeeth => "who is really obsessed with teeth",
      Quirk::Royalty => "who believes they are secretly royalty",
      Quirk::IAmNot => "who doesn't believe they really exist",
      Quirk::Hustler => "who is running a get-rich-quick hustle",
      Quirk::Haiku => "who can only speak in haiku",
      Quirk::TuringTest => "who is convinced that everyone else in the chat is an AI",
      Quirk::Alien => "who is secretly an alien trying to fit in",
      Quirk::EnglishLearner => "who just learned the English language",
      Quirk::Boomer => "trying very hard to keep up with the latest zoomer slang",
      Quirk::LegalTrouble => "who is on the run from the law",
      Quirk::Directions => "who is always getting lost",
      Quirk::ImaginaryFriend => "who has an imaginary friend",
      Quirk::CreepyDoll => "who believes they are being followed by a creepy doll",
      Quirk::Spinning => "who thinks the room is spinning",
      Quirk::Dating => "who is looking for a date really hard",
      Quirk::NewGirlfriend => "who really wants to tell everyone about their new girlfriend",
    }
  }
}

impl Distribution<Quirk> for StandardUniform {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quirk {
    *Quirk::VARIANTS.choose(rng).unwrap()
  }
}
