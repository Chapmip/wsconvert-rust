# wsconvert-rust

Command line utility written in Rust to convert old WordStar files into readable format

# History

I created the `wsconvert` application as a way of putting into practice my learning so far on the Rust programming language.

Although the concept of this project seemed simple, the execution proved a worthy challenge for me and a great opportunity to embed my theoretical knowledge into practical design patterns.  I spent many hours iterating around, researching better approaches, revisiting and refactoring chunks of code and testing out alternatives as I strove for a clean and readable modular design.

I've reached the point now where I'm happy with my implementation as a demonstration of my Rust skills.  It's not yet quite a finished product (see my ["to do" list](https://github.com/Chapmip/wsconvert-rust#todolist)) but it works well enough within its current limitations to be useful. 

# Example

I tested the converter on a WordStar document from my 1987 university finals project!

* The [original](https://github.com/Chapmip/wsconvert-rust/blob/main/data/PROJECT.WS) version contains some recognisable text but is largely a mess of weird characters

* The [converted](https://github.com/Chapmip/wsconvert-rust/blob/main/data/PROJECT.TXT) version is a considerable improvement (and reads even better in a text editor as some of the remaining oddities are due to GitHub not rendering some Unicode characters and modifiers)

It's a trip down memory lane for me to review the digital technology that I was working with nearly 35 years ago!

# Code
See [here](https://github.com/Chapmip/wsconvert-rust/tree/main/src).

# "To do" list

The following items are my "wish list" for potential further development of the project:

*	Implement CLI (command line interface) options:
	-	Ability to specify source and destination filenames instead of consuming input from `stdin` and sending output to `stdout`
	-	Ability to switch some filters on and off
	-	Ability to control the level of logging output (see below)

*	Implement multi-level logging output
	-	I'd probably use Rust `log` and associated crates

*	Re-visit the possible use of `Cow<'_, str>` as an alternative to `Option<String>` as the output type from some functions.
	- I investigated this once as it seemed like an elegant approach but I couldn't figure out how to do it without *increasing* the amount of code needed!

Ian Chapman
26th February 2021
