# WorkAdventure Inspect

> Inspect a WorkAdventure map for some useful URLs and Jitsi links

[WorkAdventure](https://workadventu.re/) basically used [tiled](https://www.mapeditor.org/) json maps which are inspected here.

This might work… Sometimes.

## Usage

```bash
workadventure-inspect --jitsi-host <configured host of your instance> <work adventure link with map url>
```

## Example output

(shortened for better overview)

```plaintext
Layer start
  startLayer                https://***/alpaka-island.json#start

Layer start-spaceship
  startLayer                https://***/alpaka-island.json#start-spaceship

Layer jitsi-pool
  jitsiRoom                 "alpaka-island-pool" -> <jitsi-server>/globalalpakaislandpool
  jitsiTrigger              "onaction"
  jitsiTriggerMessage       "Press SPACE to join others in the pool (open Jisti Room)"

Layer jitsi-ballpit
  jitsiRoom                 "alpaka-island-ballpit" -> <jitsi-server>/globalalpakaislandballpit
  jitsiTrigger              "onaction"
  jitsiTriggerMessage       "Press SPACE to join others in the ballpit (open Jisti Room)"

Layer url-scratchpad
  openWebsite               https://***/alpaka-world-scratchpad
  openWebsiteTrigger        "onaction"
  openWebsiteTriggerMessage "Press SPACE to open scratchpad"

Layer mute-start
  silent                    true
```
