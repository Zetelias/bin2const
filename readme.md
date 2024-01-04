# bin2const
**bin2const** is a simple command line tool to convert arbitrary
files into constant declarations in your favourite
programming language. 

## Description and usage
You give this a file (sound, image, binary, whatever)
and this marvel of modern technology will give you
a constant declaration in your favourite programming
language.

What does that mean? Say I wanted to load a sound in my rust 
program without having to load it from a file at run-time,
I could simply
run the command :
```shell
bin2const sound.wav SOUND rust 4 sound.rs
```

to get the following output in sound.rs 

```rust
const SOUND: [u8; 1234] = [
    0x52, 0x49, 0x46, 0x46,
    /* [All the other bytes...] */
 ];
```

Then, I would just have to make the constant public,
import the constant, and I could use it in my program.

## Why ?
Because sometimes you want to load your files without
suffering file IO, or you want to reduce the size of
your resources with compile time optimizations, or even
you want to hide your files from the user.

## Why not ?
There are multiple reasons not to but the most notable for me are:
- The file with the constant declaration is oftentimes 5x larger 
  than the original file (although it can be reduced with the)
- Decreases maintainability
- Decreases convenience, you need to rerun the command every time
  you change the file you want to convert. For example, it's an
  issue if you want to use this for a game, and you want to 
  change the sprites often

## Sharp edges
- The tab_size argument doesn't work with most languages
- The tool is not very well tested, so it might not work as expected.
  Please report any issues you encounter.

## Usage
```shell
bin2const <input_file> <output_const_name> <conversion_type> [tab_size] [output_file]
```
    <input_file>        The file to convert.
    <output_const_name> The name of the constant to generate. Has no effect if the conversion type
                        is bin or hex.
    <conversion_type>   The type of conversion to use. Can be hex, bin, c, cdef, rust, csharp, python, javascript.
                        as well as most of their aliases.
    [tab_size]          The size of a tabulation in the output file. Per default is 4.
    [output_file]       Optional output file, if not specified, the output will be printed to stdout.

## conversion_type parameter
### "hex" | "hexadecimal" | "hexa" | "hexa-decimal" | "hexa_decimal"
This converts a file into hexadecimal disassembly, example:
```shell
00000000  2f 74 61 72  67 65 74 0a  2f 2e 69 64  65 61 0a 2f   |/target./.idea./|
00000010  2e 76 73 63  6f 64 65                                |.vscode         |
```
### "bin" | "binary"
This converts a file into binary disassembly, example:
```shell
00000000  00101111 01110100 01100001 01110010  01100111 01100101 01110100 00001010  |/target./.idea./|
00000010  00101111 00101110 01101001 01100100  01100101 01100001 00001010 00101110  |.vscode         |
```
### "c" | "cpp" | "c++" | "cxx" | "h" | "hpp" | "h++" | "hxx"
This converts a file into a C array, example:
```c
const unsigned char TARGET[] = {
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 };
 ```
### "cdef" | "c-def" | "c_def" | "def" | "define" | "cppdef"
This converts a file into a C/C++ multiline #define, example:
```c
#define TARGET_SIZE 1234
#define TARGET {0x2f, 0x74, 0x61, 0x72, /* [All the other bytes...] */ }
```
### "rust" | "rs" | "rustlang" | "rust-lang
This converts a file into a Rust const [u8] array, example:
```rust
const TARGET: [u8; 1234] = [
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 ];
```
### "csharp" | "cs" | "c#" | "c-sharp" | "c_sharp"
This converts a file into a C# byte[] array, example:
```csharp
public static readonly byte[] TARGET = new byte[] {
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 };
```
### "python" | "py" | "python3" | "py3" | "python_3" => {
This converts a file into a Python bytes array, example:
```python
TARGET = bytes([
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 ])
```
### "javascript" | "js" | "typescript" | "ts"
This converts a file into a Javascript Uint8Array, example:
```javascript
const TARGET = new Uint8Array([
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 ]);
```
### "go" | "golang" | "go-lang" | "go_lang" 
This converts a file into a Go []byte, example:
```go
var TARGET = []byte{
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 };
```
### "java" | "java8" | "java-8" | "java_8" 
This converts a file into a Java byte[], example:
```java
public static final byte[] TARGET = new byte[] {
    0x2f, 0x74, 0x61, 0x72,
    /* [All the other bytes...] */
 };
