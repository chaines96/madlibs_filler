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
    let wordlist_path_ptr = &wordlist_path; //A pointer to the wordlist is used so that it can be changed from within the conditional block. That way it only writes if arguments are passed.
    let mut madlib_path_ptr = &madlib_path;
    if args.len() > 1 {
        madlib_path_ptr = &args[0];
    }
    
    //Reading the file.
    let mut madlib_unsolved = fs::read_to_string(madlib_path_ptr)
        .expect("Error: No madlibs input file present!");

                //Reads the list of words
    let wordlist = fs::read_to_string(wordlist_path_ptr)
            .expect("Error: Wordlist file missing!");


    // Creates a mutable madlib string
    //let mut madlib_solved = String::new();
    madlib_unsolved = transform(&wordlist, &madlib_unsolved, "<n>"); //Replaces nouns
    madlib_unsolved = transform(&wordlist, &madlib_unsolved, "<v>"); //Replaces present tense verbs
    madlib_unsolved = transform(&wordlist, &madlib_unsolved, "<pv>"); //Replaces past tense verbs
    madlib_unsolved = transform(&wordlist, &madlib_unsolved, "<prv>"); //Replaces present progressive tense verbs (i.e. verbs that end in 'ing")
    madlib_unsolved = transform(&wordlist, &madlib_unsolved, "<a>"); //Replaces adjectives
    madlib_unsolved =  transform(&wordlist, &madlib_unsolved, "<ad>"); //Replaces adverbs
    let mut f = fs::File::create("output.txt")?;
    f.write_all(madlib_unsolved.as_bytes())?;
    Ok(())
}

//Inputs: wordlist_point: String of words to be inserted with seperators between each category.
//             madlib_unsolved_point: String of text whose words are to be replaced using the specified seperator.
//              delimiter: String which acts as a seperator for both strings.
//Outputs: A string which contains a modified version of madlib_unsolved_point with the specified positoins replaced with random words from the wordlist.
fn transform(wordlist_point: &str, madlib_unsolved_point: &str, delimiter: &str) -> String {
        let mut wordlist = String::from(wordlist_point);
        let mut madlib_unsolved = String::from(madlib_unsolved_point);
        let mut word_collector:  Vec<&str> = wordlist.split(delimiter).collect();
        let mut verbs: Vec<_> = word_collector[1].lines().collect();
            //Randomizes order of words
        let mut rng = thread_rng();
        verbs.shuffle(&mut rng);
    
        let mut newthing = String::new();
        let verb_splits = madlib_unsolved.split(delimiter);
        let mut i: usize = 0;
        let mut j: usize = 0;
        let verblist_length: usize = verbs.len().try_into().unwrap();
        for verb_split in verb_splits {
            //This conditional prevents an extra line being placed first
            let mut superword = verbs[i];
            //This loop will check to make sure the word being inserted is not empty. 
            while superword == "" {
                i = (i + 1) % verblist_length;
                superword = verbs[i];
            }
            if j > 0 {
                newthing.push_str(superword);
                }
            newthing.push_str(verb_split); 
            j = j + 1;
            i = j % verblist_length;
        }
        
    return newthing.to_string();
}