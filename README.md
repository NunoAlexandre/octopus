## Intro

This repository holds a CLI tool written in Rust which reads a file where each
line captures an arbitrary JSON object with a field named `type`. 

It then process each line (aka _message_) and outputs statistics groupped by said
`type`: the number of messages and the total byte size.

For example, running `cargo run assets/heavy-input` outputs:

```
A: 46116 times, 1890756 bytes
B: 47214 times, 1412028 bytes
C: 23058 times, 945378 bytes
```

## Structure

- `bin`
  
    You can execute this bin directly by running:

    ``` bash
    cargo run <input_file> 
    ```

    Run `cargo run -- --help` to for further details on the CLI API.

- `lib` - _octopus_

    _Octopus_ is the library that provides the machinery and API to produce
    the desired output.

