If you are not famillar with markdown I suggest you [do](https://www.markdownguide.org/).
It's aesthetic, it's damn fast (as fast as any text editor).


Variables
Mutability
Variable symbol lifetime.
Constants
	they should be UPPER_CASE

Functions
	doesn't have poliformism.
	no default optional arguments, you need to use Option/Enum

if
arrays
slices
tuples
	destructuring tuples

vectors
for-in loops
moving variables
borrowing, and cloning

structs
	templates
	impl
	self

enums

match
nice code generation in VSCode

strings
string slices

modules
	function visibility

hashmaps
	concept of borrowing, with no creation

Options

if let
while let
	it's a fucking pattern maching!

ref
Result
? - error propagation symbol

map_err

generics
	generic impl

self vs Self (capitalized)
	object vs type

traits
	default implementations
	as parameters
		multiple traits as one paramter

assert!
assert_eq!

borrow checker (part of compiler)
lifetime annotations
	

iterators
	Collect
	fold

Box
	a smart pointer


Arc
	Concurency
	https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html

Rc
	reference counter

Cow
	clone on write (smart pointer)

threads
Mutex

channels
	a transmitter and reciver.

macros
	macro exporting
	macro arm

	more on macros:
	https://veykril.github.io/tlborm/

type casting
	aka 'as'


From, FromStr, TryFrom, (AsRef, AsMut) traits
