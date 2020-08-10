# drop_some

- This crate defines `DropSome` trait and `impl` for `Option`.
- `DropSome` trait has `.drop_some(self) -> Option<()>` function.
- This is a syntax sugar for `.map(|_|())`.

## Useful scene

```rust
fn some_function() -> Option<()>
{
 match switcher
 {
  pattern_a => some_task1().drop_some()?
  pattern_b => some_task2().drop_some()?
  pattern_c => some_task3()?
  pattern_d => some_task4()
  _ => ()
 }
}

fn some_task1()     -> Option<i8     > { /* abbr */ }
fn some_task2<'a>() -> Option<&'a str> { /* abbr */ }
fn some_task3()     -> Option<()     > { /* abbr */ }
fn some_task4() { /* abbr */ }
```

- See also: <tests/test.rs>

## Note

> "I don't need it, because ..."

- Yes, your are right to your world.
    - But, I tired to type/see `.map(|_|())`. So I'm happy with`.drop_some()`.

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
