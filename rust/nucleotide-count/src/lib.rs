use std::collections::HashMap;

fn is_valid(dna: &str) -> Result<(), char> {
  let mut erring = '0';
  match dna.chars().all(|c| {
    c.is_ascii_alphabetic()
      && match c {
        'A' | 'C' | 'G' | 'T' => true,
        e => {
          erring = e;
          false
        }
      }
  }) {
    true => Ok(()),
    false => Err(erring),
  }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
  if !matches!(nucleotide, 'A' | 'C' | 'G' | 'T') {
    return Err(nucleotide);
  }
  match is_valid(dna) {
    Ok(_) => Ok(dna.chars().filter(|&c| c == nucleotide).count()),
    Err(e) => Err(e),
  }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
  if let Err(e) = is_valid(dna) {
    return Err(e);
  }
  let mut map = HashMap::<char, usize>::with_capacity(4);
  map.insert('A', 0);
  map.insert('C', 0);
  map.insert('G', 0);
  map.insert('T', 0);
  Ok(dna.chars().fold(map, |mut map, b| {
    match b {
      'A' | 'C' | 'T' | 'G' => map.entry(b).and_modify(|v| *v += 1),
      _ => unreachable!(),
    };
    map
  }))
}
