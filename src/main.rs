use std::env;
use std::fs;
//use std::io;
//use unicode_segmentation::UnicodeSegmentation; //To iterate over elements of a string.
use std::string::String;
//use std::fmt::{Error, Write};
//use std::io::prelude::*;
use std::io::Write;
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    //let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let args: Vec<String> = env::args().collect();
    let wordlist_path = &args[1];
    let madlib_path = &args[2];
    println!("In query {}", wordlist_path);
    println!("In file {}", madlib_path);
    
    //Reading the file.
    let madlib_unsolved = fs::read_to_string(madlib_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{madlib_unsolved}");
    //let mut madlib_solved = "";
    //for graphene in my_str.bytes()
     //   {
      //      let mut j;
       //     if i == '<'
        //    {
         //       madlib_solved.insert("ligma");
          //  }
           // else 
            //{
             //   &madlib_solved[j] = madlib_unsolved[i];
              //  j = j + 1;
            //}
        //} 
                //Reads the list of words
    let wordlist = fs::read_to_string(wordlist_path)
            .expect("Should have been able to read the file");
    let mut verbs: Vec<_> = wordlist.lines().collect();
    //Randomizes order of words
    //let y: f64 = rng.gen(); // generates a float between 0 and 1
    //let length = len(verbs);
    //let mut nums: Vec<i32> = (1..100).collect();
    let mut rng = thread_rng();
    verbs.shuffle(&mut rng);
    //
    // Creates a mutable madlib string
    let mut madlib_solved = String::new();
    // This 
    let verb_splits = madlib_unsolved.split("<v>");
    let mut i: usize = 0;
    let mut j: usize = 0;
    let verblist_length: usize = verbs.len().try_into().unwrap();
    for verb_split in verb_splits {
        //This conditional prevents an extra line being placed first
        if j > 0 {
            madlib_solved.push_str(verbs[i]);
            }
        madlib_solved.push_str(verb_split); //Previously used .replacen(<v>,"fart",1);
        j = j + 1;
        i = (j - 1) % verblist_length;
    }
    let mut f = fs::File::create("output.txt")?;
    f.write_all(madlib_solved.as_bytes())?;
    //writeln!(&mut f, madlib_unsolved);
    Ok(())
}
