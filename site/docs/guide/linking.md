# Linking Up

Two devices running slot can link up like two Game Boys with a cable between them. Trade Pokémon, battle, or race in Mario Kart.

There's no cable. One device runs its own Wi-Fi network and the other joins it. You don't need to be connected to Wi-Fi.

!!! info "Linking is local only"
    
    Link play does not work over the internet and adding this feature is not planned.

## Linking

1. Both players open the same game.
2. Press <kbd>SELECT</kbd> + <kbd>MENU</kbd>.
3. One player picks Host, the other picks Join.

The host waits 30 seconds for the joiner to show up. The game may reload once while it gets ready for the link.

To unlink, press <kbd>SELECT</kbd> + <kbd>MENU</kbd> again and pick End Link. 

Closing the lid, powering off or ejecting also ends a link session.

## Which games can link

Game Boy, Game Boy Color and GBA games all link over the cable. Each device runs its own game with its own save, joined by [armGBA](https://github.com/MagnominusGit/armGBA)'s emulated link cable, so each player needs their own copy of the game.

Games that used the GBA Wireless Adapter link over the adapter instead. Pokémon FireRed, LeafGreen and Emerald can switch between cable and wireless with <kbd>SELECT</kbd> on the link screen.

## While you're linked

- The game keeps running behind menus. Pausing one device would freeze the other.
- Fast-forward, rewind, loading states and opening the state switcher are all disabled.
- A badge shows whether you're Hosting or Joined.
- If the other device goes quiet for 2 seconds, the link ends.

## If it doesn't connect

| Message | What happened |
| --- | --- |
| The radio did not come up | Wi-Fi didn't start on this device. Try again. |
| Nobody arrived | The host waited and nobody joined. |
| The other player vanished | The two devices found each other, then lost each other. |

Keep the two devices close and start Join after Host.
