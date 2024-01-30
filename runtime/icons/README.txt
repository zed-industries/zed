Choose your preferred icon and replace the standard Vim icon with it.
[This is for the Amiga]

When started from Workbench, Vim opens a window of standard terminal size
(80 x 25). Trying to change this by adding a tool type results in a window
that disappears before Vim comes up in its own window.
If you want Vim to start with another size, it can be done using
IconX.

Follow these steps:

1. Create a script file called e.g. Vim.WB, with a single line in which the
   Vim executable is started:
      Echo "Vim" > Vim.WB
      Protect Vim.WB +s

2. Rename the Vim icon to Vim.WB.

3. By default, the Vim icon is a program icon.
   Change the icon type from "program" to "project" using IconEdit from the
   "Tools" directory.

4. Change the icon settings using "information" from the WorkBench's "icon"
   menu:
   - The default program, of course, is "IconX".
   - A stack size of 4096 should be sufficient.
   - Create a WINDOW tooltype of the desired size.
     The appropriate values depend on your WB font.

   Example:
   On a standard non-interlaced WB screen with full overscan resolution
   (724 x 283 ), the WINDOW tooltype "CON:30/10/664/273" results in a
   horizontally centered window with 80 columns and 32 lines.

Now Vim comes up with the new window size.
