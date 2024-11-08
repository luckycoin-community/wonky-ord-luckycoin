use super::*;

pub(super) enum Block {
  Height(u32),
  Hash(BlockHash),
}

impl FromStr for Block {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(if s.len() == 64 {
      Self::Hash(s.parse()?)
    } else {
      Self::Height(s.parse()?)
    })
  }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Inscription {
  Id(InscriptionId),
  Number(i32),
}

impl FromStr for Inscription {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(if s.contains('i') {
      Self::Id(s.parse()?)
    } else {
      Self::Number(s.parse()?)
    })
  }
}

impl Display for Inscription {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Id(id) => write!(f, "{id}"),
      Self::Number(number) => write!(f, "{number}"),
    }
  }
}

pub(super) enum Lune {
  SpacedLune(SpacedLune),
  LuneId(LuneId),
}

impl FromStr for Lune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.contains(':') {
      Ok(Self::LuneId(s.parse()?))
    } else {
      Ok(Self::SpacedLune(s.parse()?))
    }
  }
}
