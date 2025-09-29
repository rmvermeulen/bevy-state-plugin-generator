# naming

Apply the correct `NamingScheme`.

## split at case-boundaries, but maintain inner name

| input   | names   | parts      | notes                                |
| ------- | ------- | ---------- | ------------------------------------ |
| `ABC`   |         | `ABC`      | consecutive capitals are joined      |
| `ABBox` |         | `ABBox`    | lowercase sections are joined to the |
|         |         |            | preceding capital(s)                 |
| `AbBox` |         | `Ab`,`Box` | split at case-boundaries             |
| `AbBox` | `AbBox` | `Ab`,`Box` | do not split a name                  |

## normalization

- always suffix the enum/struct with `State` but not the variants, e.g.
  `Map::Fishing` -> `MapState::Fishing` with sub-state `MapStateFishingState`
  |origin name| result| sub-state

### merge

- if `config.merge=true` remove `State` from the end of names when joining
  them e.g. `Map::Fishing` -> `MapState::Fishing` with sub-state `MapFishingState`

## collisions

Abort on any collisions.
