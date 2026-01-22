<h2>Stick Programming Language</h2>
<h3>placeholder readme</h3>
<b>view: <a href="https://github.com/gamerjamer43/stickvm">StickVM</a></b>

### Planned Syntax:
**ALSO ON THE TODO: complete the spec.
Imports are easy, potentially will make a preprocessor, which will then make it a directive using `#`
```
// libs will already be compiled (i hope)
#import lib
#import lib.sub
#import std.io

// files have to get included to be compiled along with the program
#import "file.file"
```
Declarations are too, just:
```
// deciding on := or just plain = rn but i like assignment vs reassignment
// <modifiers> <type> <name> := <val>
i32 number = 1;
const i32 zero = 0;
```
The only things that can be defined outside of a function scope are constant (fixed mem location fixed value), and globals (fixed mem location aka the global pool)
```
// anything outside of main scope must be constant or global
// value is constant at runtime. immutable
const i32 fuck = 42

// you can define globals outside because their memory location is fixed
// this means a lazy that is evaluated at run time is ok because we know its size at compile time
// thanks rust
global i32 shit = 42
```
You can write function prototypes similar to C, and they can be hidden away with your docstrings attached
```
// will allow for prototyping in headers/interfaces
//! this is a docstring.
//! title: name
//! desc: returns a greeting with your name
//! params: name: str = your name
func name (str name) -> str;
```
Functions are simple. Define one with the func keyword and attach params and type.
```
func meaning_of_life () -> i64 {
    return 42;
}

func name (str name) -> str {
    // potentially making strings use String.new() for heap alloc
    str string = "Hello, " .. name!
    return string;
}
```
Main is very similar, but will always return an i32 containing 0 if successful or a panic if not (it is implicit so dw).
I may change this to work so that it returns a unit type, but uncertain of what a unit type necessarily means in my language.
```
// deciding on i32 or unit type for main return
func main (i32 argc, str argv[]) -> i32 {
    // this is alr pretty go flavored so i might just keep =
    str yoName := readln("> ")
    yoName = name(yoName)

    // i may go parenthesis optional and do
    // writeln name yoName
    writeln(yoName + “ dat yo name”)
    writefn(“%s”, yoName)
}
```
I may also go parenthesis optional, allowing for:
```
writeln yoName + “ dat yo name”
writefn “%s”, yoName
```
You may find a lot of the syntax similar to Go and Rust. This is because I've done a lot of reading into the design of C++, Java, Go, Rust, Lua, Python, and as you may see a wide selection of other Bytecode VMs AND fully compiled features (with the goal of making this super easily embeddable. Into what? I don't know, but I'm keeping the footpring light!)
