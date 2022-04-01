pub struct Headers {
  pub record: usize,
  pub subrecord: usize,
  pub group: usize,
}

impl Headers {
  pub fn new() -> Self {
    Headers {
      record: 24,
      subrecord: 6,
      group: 24,
    }
  }
}
