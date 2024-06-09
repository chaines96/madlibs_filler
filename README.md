# Madlibs Filler

A program which inserts words in a string of text at indicated points with randomly chosen words much like a Mad Libs activity. 

To build this program, run 'cargo build'. This program requires the rand crate, so be sure to run 'cargo add rand 0.8.5' if the corresponding entry is not present in Cargo.toml.

To provide input to the program, create a text file and pass it as a sole parameter for the program. It will look for "input.txt" if none is supplied. It will replace each placeholder with a randomly chosen word in wordlist.txt of its category. The following placeholders work for the corresponding categories:

<p>&#60;v&#62; present tense verbs &#10;</p>
<p>&#60;prv&#62; present progressive verbs (i.e. verbs that end in "ing") &#10;</p>
<p>&#60;pv&#62; past tense verbs &#10;</p>
<p>&#60;n&#62; nouns &#10;</p>
<p>&#60;a&#62; adjectives &#10;</p>
<p>&#60;ad&#62; adverbs &#10;</p>

The output will be written to <b> output.txt </b> .
