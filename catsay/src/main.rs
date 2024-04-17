/*
page 10

Features:

• Take a message string as the positional argument.4
• Take a -h/--help flag to print a help message.
• Take a -d/--dead flag that makes the cat’s eyes become xx, which is
the comical expression of dead eyes.
• Print in color.
• Print the error message to STDERR for error handling.
• Accept STDIN for piping input and pipe the output to other
programs.
• Perform integration tests.
• Package and publish to crates.io.


Hello I'm a cat
 \
  \
    /\_/\
   ( o o )
   =( I )=

*/

fn main() {
    let message = std::env::args()
        .nth(1)
        .expect("Missing the message. Usage: catsay <message>");

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("     ( o o )");
    println!("     =( I )=");
}
