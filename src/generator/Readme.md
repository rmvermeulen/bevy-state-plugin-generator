# generator

Currently it's a mess so I'm not describing that now.

The new plan:

1. Convert the root `ParseNode` to a flat list.
   - depth-first, put the children directly after the parent
   - nodes can refer to each other by index
   - nodes can find their parent by looking below their index
2. Apply the correct `NamingScheme` (see [naming](../naming/Readme.md))
   - split at case-boundaries, but maintain inner name
   - always suffix the enum/struct with `State` but not the variants, e.g.
     `Map::Fishing` -> `MapState::Fishing` with sub-state `MapStateFishingState`
     - if `config.merge=true` remove `State` from the end of names when joining
       them e.g. `Map::Fishing` -> `MapState::Fishing` with sub-state `MapFishingState`
   - detect collisions
3. generate a definition for each item in the list.
