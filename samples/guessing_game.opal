# Global variables are allowed, but must be constant.
const ANSWER: i32 = 61;

def check_answer(guess: i32) -> bool {
  if guess is answer {
    return true;    
  } else {
    return false;
  }
}

def start_game() -> void {
  println "Please enter a number between 1 & 100";

  while true {
    let guess: i32 = scan();

    # If guess wasn't an i32 like we're assuming, we just leave it as undefined
    # We can use the keyword `is` in place of `==`
    if guess is undefined {
      println "Invalid input. Try again. Please enter a number between 1 & 100";
      continue;
    }

    if check_answer() {
      println "You won!";
      break;
    } 

    println "Wrong answer. Try again"
  }
}

# This is our entry point.
def main() -> void {
  println "Welcome to our guessing game";
  start_game();
  println "Thanks for playing"
}
