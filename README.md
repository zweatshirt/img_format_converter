# image_format_converter

--- 

An example of running the project assuming you are in the project's working directory:
`./imgfmt -i home/Desktop/image.jpeg -o home/Desktop/image.png`

run `./imgfmt -h` for a list of format options

-i: input flag

-o: output flag

-h: help flag

If you have Linux: while in the project's working directory run `sudo cp ./imgfmt /usr/local/bin/` to turn the project into an easily accessible executable.

After doing this, you can access the executable any time by running `imgfmt` in the terminal.

---

## NOTE:

This project was written on a Linux machine and as such the executable may not work on other platforms. If so, run `cargo build --release` on your native machine.
