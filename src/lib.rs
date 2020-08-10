/// Option<????> -> Option<()>
pub trait DropSome
{
 /// Only drop Ok value, no drop Err value.
 /// # Example
 /// ```
 /// use drop_some::DropSome;
 ///
 /// fn some_function() -> Option<()>
 /// {
 ///  match "something"
 ///  {
 ///   "pattern-x" => some_task1().drop_some()?,
 ///   "pattern-y" => some_task2().drop_some()?,
 ///   "pattern-z" => some_task3()?,
 ///   "pattern-w" => some_task4(),
 ///   _ => ()
 ///  }
 ///  Ok(())
 /// }
 ///
 /// fn some_task1()     -> Option<i8     > { Some(1) }
 /// fn some_task2<'a>() -> Option<&'a str> { Some("abc") }
 /// fn some_task3()     -> Option<()     > { Some(()) }
 /// fn some_task4() { }
 /// ```
 /// See also: <https://github.com/usagi/drop_some/tests/test.rs>
 fn drop_some(self) -> Option<()>;
}

impl<V> DropSome for Option<V>
{
 fn drop_some(self) -> Option<()>
 {
  self.map(|_| ())
 }
}
