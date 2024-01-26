#[macro_export]
macro_rules! css {
 // This rule handles values. It pushes the value onto the `cs` string.
 ($cs:ident $v:expr, $($rest:tt)*) => {{
    $cs.push_str(&format!("{}\n", $v));
    css!($cs $($rest)*)
 }};

 // This rule handles key-value pairs. It pushes the pair onto the `cs` string.
 ($cs:ident $k:ident: $v:expr, $($rest:tt)*) => {{
    $cs.push_str(&format!("{}: {};\n", stringify!($k), $v));
    css!($cs $($rest)*)
 }};

 // This rule handles the exit condition of the recursion. It trims the `cs` string and returns it.
 ($cs:ident) => {
    $cs.trim().to_string()
 };

 // This rule is the starting point of the recursion. It initializes a new string and starts the recursion.
 ($($rest:tt)+) => {{
    let mut cs = String::new();
    css!(cs $($rest)+)
 }};
}


fn main() {
  let left_string = String::from("left: 4px;");
    println!("{}", css![
                top: 0,
                "bottom: 0;",
                left_string,
                "right: 2vh;",
            ]);
} 