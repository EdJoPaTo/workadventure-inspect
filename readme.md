# WorkAdventure Inspect

> Inspect a WorkAdventure map for some useful URLs and Jitsi links

[WorkAdventure](https://workadventu.re/) basically used [tiled](https://www.mapeditor.org/) json maps which are inspected here.

This might work… Sometimes.

## Usage

```bash
workadventure-inspect --jitsi-host <configured host of your instance> <work adventure link with map url>
workadventure-inspect --jitsi-host https://meet02.verstehbahnhof.de/ https://visit.alpaka.world/_/global/wikipaka.world/alpaka-island.json
```

## Example output

(shortened)

```plaintext
$ workadventure-inspect --jitsi-host https://meet02.verstehbahnhof.de/ https://visit.alpaka.world/_/global/wikipaka.world/alpaka-island.json
get map url... https://wikipaka.world/alpaka-island.json
Layer start
  startLayer                https://visit.alpaka.world/_/global/wikipaka.world/alpaka-island.json#start

Layer start-spaceship
  startLayer                https://visit.alpaka.world/_/global/wikipaka.world/alpaka-island.json#start-spaceship

Layer jitsi-pool
  jitsiRoom                 "alpaka-island-pool" -> https://meet02.verstehbahnhof.de/globalalpakaislandpool
  jitsiTrigger              "onaction"
  jitsiTriggerMessage       "Press SPACE to join others in the pool (open Jisti Room)"

Layer jitsi-ballpit
  jitsiRoom                 "alpaka-island-ballpit" -> https://meet02.verstehbahnhof.de/globalalpakaislandballpit
  jitsiTrigger              "onaction"
  jitsiTriggerMessage       "Press SPACE to join others in the ballpit (open Jisti Room)"

Layer url-scratchpad
  openWebsite               https://pad.abc-huell.de/alpaka-world-scratchpad
  openWebsiteTrigger        "onaction"
  openWebsiteTriggerMessage "Press SPACE to open scratchpad"

Layer mute-start
  silent                    true
```
