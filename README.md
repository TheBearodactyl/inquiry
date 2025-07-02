# Inquiry - Create menus out of enumerations with the power of `inquire`

```rs
#[derive(Debug, Copy, Clone, Choice)]
enum ExampleChoice {
  /// Description of choice 1
  ChoiceOne,

  /// Description of choice 2
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
