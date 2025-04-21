
//! Regex, or more precisely [`RegexSet`], for detecting words that
//! trigger a personality change.

use super::{Personality, BasePersonality};
use super::base::RANDOM_TRIGGER_WORDS;

use regex::{RegexSet, RegexSetBuilder, escape};
use rand::rng;
use rand::seq::IndexedRandom;
use strum::VariantArray;

use std::sync::LazyLock;

const TRIGGER_WORDS: LazyLock<Vec<(String, Box<dyn PersonalityShifter>)>> = LazyLock::new(|| {
  let mut triggers = Vec::<(String, Box<dyn PersonalityShifter>)>::new();

  for random_word in RANDOM_TRIGGER_WORDS {
    triggers.push((random_word.to_string(), Box::new(ShiftToRandom)));
  }

  for personality in BasePersonality::VARIANTS {
    for word in personality.trigger_words() {
      triggers.push((word.to_string(), Box::new(ShiftTo::new(*personality))));
    }
  }

  triggers
});

const TRIGGER_REGEXES: LazyLock<RegexSet> = LazyLock::new(|| {
  RegexSetBuilder::new(TRIGGER_WORDS.iter().map(|(word, _)| format!(r"\b{}\b", escape(word))))
    .case_insensitive(true)
    .build()
    .unwrap()
});

trait PersonalityShifter {
  fn can_trigger(&self, personality: &Personality) -> bool;
  fn trigger_shift(&self, personality: &mut Personality);
}

#[derive(Debug, Clone)]
struct ShiftTo {
  target: BasePersonality,
}

#[derive(Debug, Clone)]
struct ShiftToRandom;

impl ShiftTo {
  fn new(target: BasePersonality) -> Self {
    Self { target }
  }
}

impl PersonalityShifter for ShiftTo {
  fn can_trigger(&self, personality: &Personality) -> bool {
    personality.base_personality != self.target
  }

  fn trigger_shift(&self, personality: &mut Personality) {
    *personality = Personality::generate(self.target);
  }
}

impl PersonalityShifter for ShiftToRandom {
  fn can_trigger(&self, _: &Personality) -> bool {
    true
  }

  fn trigger_shift(&self, personality: &mut Personality) {
    let mut random = rng();
    let mut options: Vec<_> = BasePersonality::VARIANTS.into_iter().copied().collect();
    options.remove(0); // Never generate the [`BasePersonality::Basic`] personality.
    *personality = Personality::generate(*options.choose(&mut random).unwrap());
  }
}

/// Runs the personality shifter and returns true if any changes were
/// made to the personality.
pub fn run_personality_shift(message_text: &str, personality: &mut Personality) -> bool {
  for m in TRIGGER_REGEXES.matches(message_text) {
    let trigger = &TRIGGER_WORDS[m].1;
    if trigger.can_trigger(&personality) {
      trigger.trigger_shift(personality);
      return true;
    }
  }
  false
}
