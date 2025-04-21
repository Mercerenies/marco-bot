
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
    }
  }
}

impl Distribution<Quirk> for StandardUniform {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quirk {
    *Quirk::VARIANTS.choose(rng).unwrap()
  }
}
