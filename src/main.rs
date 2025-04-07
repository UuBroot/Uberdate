mod brew;

use colored::Colorize;
use brew::check_brew_installed;

fn main() {
    println!("{}","##############################".yellow());
    println!("{}", "########### Update ###########".green());
    println!("{}","##############################".yellow());

    //Brew
    if check_brew_installed(){
        println!("brew installed");
    }
}
