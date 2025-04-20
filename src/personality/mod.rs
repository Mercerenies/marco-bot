
mod base;
mod quirk;

pub use base::{BasePersonality, Adjective};
pub use quirk::Quirk;

use rand::{rng, Rng};

use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Personality {
  pub base_personality: BasePersonality,
  pub adjective: Option<Adjective>,
  pub quirk: Option<Quirk>,
}

impl Personality {
  pub fn generate(base_personality: BasePersonality) -> Self {
    let mut random = rng();
    let adjective = if random.random::<f32>() < 0.2 {
      Some(random.random())
    } else {
      None
    };
    let quirk = Some(random.random());
    Personality {
      base_personality,
      adjective,
      quirk,
    }
  }
}

impl Display for Personality {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.base_personality.long_name(self.adjective))?;
    if let Some(quirk) = &self.quirk {
      write!(f, " {}", quirk.phrase())?;
    }
    Ok(())
  }
}

impl Default for Personality {
  fn default() -> Self {
    Personality {
      base_personality: BasePersonality::default(),
      adjective: None,
      quirk: None,
    }
  }
}
