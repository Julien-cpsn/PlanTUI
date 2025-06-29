# PlanTUI

![](./examples/demo.png)

> [!WARNING]
> You must have a command called `plantuml` *OR* set the `PLANT_UML` env var to points towards your plantuml compiler. Likely something like `java -jar my_compiler`.
> For more information, please visit https://plantuml.com/starting

PlantUML editor and renderer in your terminal!

> [!NOTE]
> It will use graphical protocols like Kitty when available and unicode ASCII otherwise

## How to use

### TUI from existing file

```shell
plantui examples/bob_alice.pmu
```

### Temp file

Simply with not input file

```shell
plantui
```

### Render

```shell
plantui examples/bob_alice.pmu render ./my_output_folder 
```

### Help

```shell
Usage: plantui [OPTIONS] [INPUT] [COMMAND]

Commands:
  render  Render the input to the output
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [INPUT]  PlantUML file to edit

Options:
  -l, --light-mode  Activate light mode instead of dark mode
  -d, --dark-mode   Activate dark mode instead of light mode
  -h, --help        Print help
```

### Key bindings

```shell
ctrl-shift-Y: Copy output into clipboard
ctrl-shift-D: Toggle light/dark mode
```