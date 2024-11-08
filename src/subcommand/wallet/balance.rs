use {super::*, crate::wallet::Wallet, std::collections::BTreeSet};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Output {
  pub cardinal: u64,
  pub ordinal: u64,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub lunes: Option<BTreeMap<Lune, u128>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub dunic: Option<u64>,
  pub total: u64,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;
  index.update()?;

  let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;

  let inscription_outputs = index
    .get_inscriptions(None)?
    .keys()
    .map(|satpoint| satpoint.outpoint)
    .collect::<BTreeSet<OutPoint>>();

  let mut cardinal = 0;
  let mut ordinal = 0;
  let mut lunes = BTreeMap::new();
  let mut dunic = 0;
  for (outpoint, amount) in unspent_outputs {
    let lune_balances = index.get_lune_balances_for_outpoint(outpoint)?;

    if inscription_outputs.contains(&outpoint) {
      ordinal += amount.to_sat();
    } else if !lune_balances.is_empty() {
      for (spaced_lune, pile) in lune_balances {
        *lunes.entry(spaced_lune.lune).or_default() += pile.amount;
      }
      dunic += amount.to_sat();
    } else {
      cardinal += amount.to_sat();
    }
  }

  Ok(Box::new(Output {
    cardinal,
    ordinal,
    lunes: index.has_lune_index().then_some(lunes),
    dunic: index.has_lune_index().then_some(dunic),
    total: cardinal + ordinal + dunic,
  }))
}
