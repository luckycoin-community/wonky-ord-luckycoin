use super::*;

#[derive(Default, Serialize, Debug, PartialEq, Copy, Clone)]
pub struct Edict {
  pub id: u128,
  pub amount: u128,
  pub output: u128,
}

impl Edict {
  pub(crate) fn from_integers(
    tx: &Transaction,
    id: u128,
    amount: u128,
    output: u128,
  ) -> Option<Self> {
    let lune_id = LuneId::try_from(id).ok()?;

    if lune_id.height == 0 && lune_id.index > 0 {
      return None;
    }

    if output > u128::try_from(tx.output.len()).ok()? {
      return None;
    }

    Some(Self { id, amount, output })
  }
}
