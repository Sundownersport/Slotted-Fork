# Playing Games

## Inserting a cart

Tap <kbd>A</kbd> on a cart to insert it. If you've played it before, you pick up right where you left off.

Hold <kbd>A</kbd> to start the game fresh. The game will launch without autoloading your save state.

If you'd rather always start fresh, turn off Load State on Insert in Settings → Gameplay.

## Ejecting

Hold <kbd>MENU</kbd> to eject the cart and go back to the shelf. 

slot creates a save state on the way out, unless you've turned off Save State on Eject in Settings → Gameplay.

## The in-game menu

Tap <kbd>MENU</kbd> while playing. The game pauses behind it.

| Row | Does |
| --- | --- |
| Resume | Back to the game |
| States | Opens the [save state switcher](save-states.md) |
| Screenshots | Opens the [screenshot gallery](screenshots.md) |
| Settings | Screen and Gameplay for the system you're playing, and Controls |
| Exit Game | Ejects the cart |

## The emulator

Every game runs on [armGBA](https://github.com/MagnominusGit/armGBA). Its GBA side is a recompiler built on parts of mGBA, and its Game Boy and Game Boy Color side is Gambatte.

!!! warning "Save states from mGBA and gpSP don't carry over"

    States made by older versions of slot on mGBA or gpSP don't show up on armGBA. Your battery save is shared, so in-game saves carry over.

## Battery saves

Games that save on the cart write to `Saves/`.

## Rumble

Games with a rumble pak rumble. Turn it off in Settings → Gameplay if you'd rather they didn't.

Pokémon Pinball: Ruby & Sapphire still needs rumble turned on in the game's own options.
