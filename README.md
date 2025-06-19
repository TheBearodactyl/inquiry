```rs
#[derive(Choice, Debug, Clone, Copy)]
enum MyChoice {
  Variant1,
  Variant2
}

impl std::fmt::Display for MyChoice {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      MyChoice::Variant1 => "Variant 1",
      MyChoice::Variant2 => "Variant 2"
    }
  }
}

fn main() {
  let choose = MyChoice::choice("Please choose one")
    .expect("Failed to get choice");

  match choose {
    MyChoice::Variant1 => todo!(),
    MyChoice::Variant2 => todo!()
  }
}
```
