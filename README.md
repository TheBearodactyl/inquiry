```rs
#[derive(Debug, Copy, Clone, Choice)]
enum ExampleChoice {
  #[desc(text = "Description of choice #1")]
  ChoiceOne,

  #[desc(text = "Description of choice #2")]
  ChoiceTwo,
}

fn main() -> InqureResult<()> {
  let first_choice = ExampleChoice::choice("Please choose one")?;

  match first_choice {
    ExampleChoice::ChoiceOne => println!("Chose `ChoiceOne`"),
    ExampleChoice::ChoiceTwo => println!("Chose `ChoiceTwo`"),
  }

  Ok(())
}
```
