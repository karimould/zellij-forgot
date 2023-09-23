# ❔Zellij Forgot

Zellij is your solution for quickly accessing and remembering your keybindings (or anything else) in Zellij.
Forget the hassle of remembering all your keybindings (or other stuff), let Zellij Forgot do it for you!

## 🛠️ Installation

To install the Zellij Forgot plugin, follow the standard plugin installation process for Zellij.
Place the compiled zellij_forgot.wasm file in the appropriate directory.
You can find the current release [here](https://github.com)

1. Download the `zellij_forgot.wasm` file from the release matching your installed Zellij version
2. Place it in `~/zellij-plugins`
3. From inside Zellij, run `zellij action new-pane --plugin file:~/zellij-plugins/zellij_forgot.wasm --floating`

## 🎹 Set Up Your Keybinds and Activate 'Zellij Forgot' with Ease

The 'Zellij Forgot' plugin fetches the displayed keybinds directly from the plugin's configuration key-value map.
Below is an example using Zellij's default keybinds.
To integrate this into your setup, append the following snippet to your zellij config, specifically within the keybinds segment:

Add the following to your [zellij config](https://zellij.dev/documentation/configuration.html) somewhere inside the [`keybinds`](https://zellij.dev/documentation/keybindings.html) section:

```kdl
shared_except "locked" {
    bind "Ctrl y" {
        LaunchOrFocusPlugin "file:~/zellij-plugins/zellij_forgot.wasm" {
            "lock"                  "ctrl + g"
            "unlock"                "ctrl + g"
            "new pane"              "ctrl + p + n"
            "change focus of pane"  "ctrl + p + arrow key"
            "close pane"            "ctrl + p + x"
            "rename pane"           "ctrl + p + c"
            "toggle fullscreen"     "ctrl + p + f"
            "toggle floating pane"  "ctrl + p + w"
            "toggle embed pane"     "ctrl + p + e"
            "choose right pane"     "ctrl + p + l"
            "choose left pane"      "ctrl + p + r"
            "choose upper pane"     "ctrl + p + k"
            "choose lower pane"     "ctrl + p + j"
            "new tab"               "ctrl + t + n"
            "close tab"             "ctrl + t + x"
            "change focus of tab"   "ctrl + t + arrow key"
            "rename tab"            "ctrl + t + r"
            "sync tab"              "ctrl + t + s"
            "brake pane to new tab" "ctrl + t + b"
            "brake pane left"       "ctrl + t + ["
            "brake pane right"      "ctrl + t + ]"
            "toggle tab"            "ctrl + t + tab"
            "increase pane size"    "ctrl + n + +"
            "decrease pane size"    "ctrl + n + -"
            "increase pane top"     "ctrl + n + k"
            "increase pane right"   "ctrl + n + l"
            "increase pane bottom"  "ctrl + n + j"
            "increase pane left"    "ctrl + n + h"
            "decrease pane top"     "ctrl + n + K"
            "decrease pane right"   "ctrl + n + L"
            "decrease pane bottom"  "ctrl + n + J"
            "decrease pane left"    "ctrl + n + H"
            "move pane to top"      "ctrl + h + k"
            "move pane to right"    "ctrl + h + l"
            "move pane to bottom"   "ctrl + h + j"
            "move pane to left"     "ctrl + h + h"
            "search"                "ctrl + s + s"
            "go into edit mode"     "ctrl + s + e"
            "detach session"        "ctrl + o + w"
            "open session manager"  "ctrl + o + w"
            "quit zellij"           "ctrl + q"
            floating true
        }
    }
}
```

Remember, this is just a template. Feel free to customize the keybindings to better suit your workflow!

## Development

Load the `dev.kdl` layout from inside zellij: `zellij action new-tab -l dev.kdl` or from outside Zellij with `zellij -l dev.kdl`
