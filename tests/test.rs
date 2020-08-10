use drop_some::DropSome;

#[test]
fn test()
{
 let x = ok_value_to_unit();

 // It is NOT drop `None`.
 assert_eq!(x.drop_some(), None);
}

fn ok_value_to_unit() -> Option<String>
{
 // If it would be compilable then `.drop_some()` is work expectedly.
 // Because, `match` must be return one value that is the same type for any patterns.
 match "something"
 {
  // Option<i8> -> Option<()> -> ()
  "pattern-x" => some_task1().drop_some()?,
  // Option<'a &str> -> Option<()> -> ()
  "pattern-y" => some_task2().drop_some()?,
  // Option<()> -> ()
  "pattern-z" => some_task3()?,
  // () from the function
  "pattern-w" => some_task4(),
  // () defined directly
  _ => ()
 }

 None
}

fn some_task1() -> Option<i8>
{
 Some(1)
}

fn some_task2<'a>() -> Option<&'a str>
{
 Some("abc")
}

fn some_task3() -> Option<()>
{
 Some(())
}

fn some_task4() -> ()
{
 ()
}
