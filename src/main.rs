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
    let args: Vec<String> = env::args().collect();
    let wordlist_path = String::from("wordlist.txt");
    let madlib_path = String::from("input.txt");
    let mut wordlist_path_ptr = &wordlist_path; //A pointer to the wordlist is used so that it can be changed from within the conditional block. That way it only writes if arguments are passed.
    let mut madlib_path_ptr = &madlib_path;
    if args.len() > 1 {
        wordlist_path_ptr = &args[1];
        madlib_path_ptr = &args[2];
    }
    
    //Reading the file.
    let madlib_unsolved = fs::read_to_string(madlib_path_ptr)
        .expect("Error: No madlibs input file present!");
    println!("With text:\n{madlib_unsolved}");
                //Reads the list of words
    let wordlist = fs::read_to_string(wordlist_path_ptr)
            .expect("Error: Wordlist file missing!");


    // Creates a mutable madlib string
    let mut madlib_solved = String::new();
    madlib_solved.push_str(&transform(wordlist, madlib_unsolved, "<v>"));
    let mut f = fs::File::create("output.txt")?;
    f.write_all(madlib_solved.as_bytes())?;
    
    //writeln!(&mut f, madlib_unsolved);
    Ok(())
}

fn transform(wordlist: String, madlib_unsolved: String, delimiter: &str) -> String {
        let mut yvqf:  Vec<&str> = wordlist.split(delimiter).collect();
        let mut verbs: Vec<_> = yvqf[1].lines().collect();
            //Randomizes order of words
        let mut rng = thread_rng();
        verbs.shuffle(&mut rng);
    
        let mut newthing = String::new();
        // This 
        let verb_splits = madlib_unsolved.split(delimiter);
        let mut i: usize = 0;
        let mut j: usize = 0;
        let verblist_length: usize = verbs.len().try_into().unwrap();
        for verb_split in verb_splits {
            //This conditional prevents an extra line being placed first
            if j > 0 {
                newthing.push_str(verbs[i]);
                }
            newthing.push_str(verb_split); //Previously used .replacen(<v>,"fart",1);
            j = j + 1;
            i = (j - 1) % verblist_length;
        }
        
    return newthing.to_string();
}