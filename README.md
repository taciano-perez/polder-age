# polder-age
Turn-based strategy game in a Dutch history setting.

## Project Name
Ideas:
- The Polder Age
- Age of Polders

## Short Description
Turn-based strategy game. Uses a tile map (top-down view). Each tile has an altitude level, and low tiles are periodically flooded by sea and rivers. Tiles can have improvements that modify altitude and productivity. Production is required to build improvements.

## Story
You are the leader of a small farming community constantly threatened by floods, and your quest is to turn a swamp into a burgeoning civilization. In your journey, you will not change only history, but geography. "God made the World, but the Dutch made The Netherlands."

## Basic Game Loops
1. Turn start (show map).
2. Tiles yield production.
3. Player actions:
	1. Save/load game.
	2. Build improvements.
	3. Move units.
	4. End turn.
4. Random events:
	1. Sea flood
	2. River flood
5. If the population is wiped, end the game.

## Minimum Viable Product
1. Create a basic map.
2. Tiles yield production. Show available resources.
3. Let the player improve tiles. Show improved tiles.
4. Let the player end the turn. Show turn/year.
5. Random flooding. Show flooding consequences.
6. Check for endgame conditions.

## Tile Improvements
- Embankment: raises the altitude of a tile.
- Trench: lowers the altitude of a tile.
- Dike: controls the flow of water to a neighboring tile.
- Windwill: can pump water to a neighboring tile.
- Farm/farmland: produce food.
- City: produce tools and money.
- Port: produce money.

## Units
- Worker: builds improvements.
- Farmer: builds farms.

## Stretch Goals
- Introduce units.
- Animate tiles/units/improvements.
- Music and sound effects.
- Web deployment with WASM.
- Random events (beyond flooding) with positive and/or negative consequences (similar to Crusader Kings)
- Seasons.
- Fogged map that is progressively discovered.
- Add competing nations and introduce war mechanics. War mechanics would be turn-based, similar to Imperialism.
- Introduce commerce.
- Add governants with traits that modify gameplay (a la Crusader Kings).


### Notes for water spread algorithm

every turn:

Increase one water on river source.
recalculate_water (source)

recalculate_water (origin)

if origin is of drain type, set water to default and return

if origin has water {
	while (there's a lower water_level neighbor) {
		transfer one water from origin to neighbor
		recalculate_water(neighbor)
	}
}
while (there's a higher water_level neighbor with water) {
	transfer water from neighbor to origin
	recalculate_water(neighbor)
}



####

every turn:
Increase one water on river source.

Recursively (starting on source):
- neighbor = first lowest neighboring tile
- if neighbor is of drain type, then origin.water -= 1
- if origin.water_level > neighbor.water_level+1, then neighbor.water += 1, origin.water -= 1
- Call recursive (passing neighbor as node)

To trigger a flood:
- Sea: increase one water per turn in a sea tile
- trigger recalculation of that tile

- Increase two (or more) water per turn on river source


Recalculate (origin):
if origin.water > 0
	- neighbor = first lowest neighboring tile
	- if neighbor is of drain type, then origin.water -= 1, return
	- if origin.water_level > neighbor.water_level+1, then neighbor.water += 1, origin.water -= 1
	- else
		- neighbor = first highest neighboring tile
- Call recursive (passing neighbor as node)
