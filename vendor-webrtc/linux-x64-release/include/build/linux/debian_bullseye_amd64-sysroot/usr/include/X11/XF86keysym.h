/*
 * XFree86 vendor specific keysyms.
 *
 * The XFree86 keysym range is 0x10080001 - 0x1008FFFF.
 *
 * X.Org will not be adding to the XF86 set of keysyms, though they have
 * been adopted and are considered a "standard" part of X keysym definitions.
 * XFree86 never properly commented these keysyms, so we have done our
 * best to explain the semantic meaning of these keys.
 *
 * XFree86 has removed their mail archives of the period, that might have
 * shed more light on some of these definitions. Until/unless we resurrect
 * these archives, these are from memory and usage.
 */

/*
 * ModeLock
 *
 * This one is old, and not really used any more since XKB offers this
 * functionality.
 */

#define XF86XK_ModeLock		0x1008FF01	/* Mode Switch Lock */

/* Backlight controls. */
#define XF86XK_MonBrightnessUp    0x1008FF02  /* Monitor/panel brightness */
#define XF86XK_MonBrightnessDown  0x1008FF03  /* Monitor/panel brightness */
#define XF86XK_KbdLightOnOff      0x1008FF04  /* Keyboards may be lit     */
#define XF86XK_KbdBrightnessUp    0x1008FF05  /* Keyboards may be lit     */
#define XF86XK_KbdBrightnessDown  0x1008FF06  /* Keyboards may be lit     */
#define XF86XK_MonBrightnessCycle 0x1008FF07  /* Monitor/panel brightness */

/*
 * Keys found on some "Internet" keyboards.
 */
#define XF86XK_Standby		0x1008FF10   /* System into standby mode   */
#define XF86XK_AudioLowerVolume	0x1008FF11   /* Volume control down        */
#define XF86XK_AudioMute	0x1008FF12   /* Mute sound from the system */
#define XF86XK_AudioRaiseVolume	0x1008FF13   /* Volume control up          */
#define XF86XK_AudioPlay	0x1008FF14   /* Start playing of audio >   */
#define XF86XK_AudioStop	0x1008FF15   /* Stop playing audio         */
#define XF86XK_AudioPrev	0x1008FF16   /* Previous track             */
#define XF86XK_AudioNext	0x1008FF17   /* Next track                 */
#define XF86XK_HomePage		0x1008FF18   /* Display user's home page   */
#define XF86XK_Mail		0x1008FF19   /* Invoke user's mail program */
#define XF86XK_Start		0x1008FF1A   /* Start application          */
#define XF86XK_Search		0x1008FF1B   /* Search                     */
#define XF86XK_AudioRecord	0x1008FF1C   /* Record audio application   */

/* These are sometimes found on PDA's (e.g. Palm, PocketPC or elsewhere)   */
#define XF86XK_Calculator	0x1008FF1D   /* Invoke calculator program  */
#define XF86XK_Memo		0x1008FF1E   /* Invoke Memo taking program */
#define XF86XK_ToDoList		0x1008FF1F   /* Invoke To Do List program  */
#define XF86XK_Calendar		0x1008FF20   /* Invoke Calendar program    */
#define XF86XK_PowerDown	0x1008FF21   /* Deep sleep the system      */
#define XF86XK_ContrastAdjust	0x1008FF22   /* Adjust screen contrast     */
#define XF86XK_RockerUp		0x1008FF23   /* Rocker switches exist up   */
#define XF86XK_RockerDown	0x1008FF24   /* and down                   */
#define XF86XK_RockerEnter	0x1008FF25   /* and let you press them     */

/* Some more "Internet" keyboard symbols */
#define XF86XK_Back		0x1008FF26   /* Like back on a browser     */
#define XF86XK_Forward		0x1008FF27   /* Like forward on a browser  */
#define XF86XK_Stop		0x1008FF28   /* Stop current operation     */
#define XF86XK_Refresh		0x1008FF29   /* Refresh the page           */
#define XF86XK_PowerOff		0x1008FF2A   /* Power off system entirely  */
#define XF86XK_WakeUp		0x1008FF2B   /* Wake up system from sleep  */
#define XF86XK_Eject            0x1008FF2C   /* Eject device (e.g. DVD)    */
#define XF86XK_ScreenSaver      0x1008FF2D   /* Invoke screensaver         */
#define XF86XK_WWW              0x1008FF2E   /* Invoke web browser         */
#define XF86XK_Sleep            0x1008FF2F   /* Put system to sleep        */
#define XF86XK_Favorites	0x1008FF30   /* Show favorite locations    */
#define XF86XK_AudioPause	0x1008FF31   /* Pause audio playing        */
#define XF86XK_AudioMedia	0x1008FF32   /* Launch media collection app */
#define XF86XK_MyComputer	0x1008FF33   /* Display "My Computer" window */
#define XF86XK_VendorHome	0x1008FF34   /* Display vendor home web site */
#define XF86XK_LightBulb	0x1008FF35   /* Light bulb keys exist       */
#define XF86XK_Shop		0x1008FF36   /* Display shopping web site   */
#define XF86XK_History		0x1008FF37   /* Show history of web surfing */
#define XF86XK_OpenURL		0x1008FF38   /* Open selected URL           */
#define XF86XK_AddFavorite	0x1008FF39   /* Add URL to favorites list   */
#define XF86XK_HotLinks		0x1008FF3A   /* Show "hot" links            */
#define XF86XK_BrightnessAdjust	0x1008FF3B   /* Invoke brightness adj. UI   */
#define XF86XK_Finance		0x1008FF3C   /* Display financial site      */
#define XF86XK_Community	0x1008FF3D   /* Display user's community    */
#define XF86XK_AudioRewind	0x1008FF3E   /* "rewind" audio track        */
#define XF86XK_BackForward	0x1008FF3F   /* ??? */
#define XF86XK_Launch0		0x1008FF40   /* Launch Application          */
#define XF86XK_Launch1		0x1008FF41   /* Launch Application          */
#define XF86XK_Launch2		0x1008FF42   /* Launch Application          */
#define XF86XK_Launch3		0x1008FF43   /* Launch Application          */
#define XF86XK_Launch4		0x1008FF44   /* Launch Application          */
#define XF86XK_Launch5		0x1008FF45   /* Launch Application          */
#define XF86XK_Launch6		0x1008FF46   /* Launch Application          */
#define XF86XK_Launch7		0x1008FF47   /* Launch Application          */
#define XF86XK_Launch8		0x1008FF48   /* Launch Application          */
#define XF86XK_Launch9		0x1008FF49   /* Launch Application          */
#define XF86XK_LaunchA		0x1008FF4A   /* Launch Application          */
#define XF86XK_LaunchB		0x1008FF4B   /* Launch Application          */
#define XF86XK_LaunchC		0x1008FF4C   /* Launch Application          */
#define XF86XK_LaunchD		0x1008FF4D   /* Launch Application          */
#define XF86XK_LaunchE		0x1008FF4E   /* Launch Application          */
#define XF86XK_LaunchF		0x1008FF4F   /* Launch Application          */

#define XF86XK_ApplicationLeft	0x1008FF50   /* switch to application, left */
#define XF86XK_ApplicationRight	0x1008FF51   /* switch to application, right*/
#define XF86XK_Book		0x1008FF52   /* Launch bookreader           */
#define XF86XK_CD		0x1008FF53   /* Launch CD/DVD player        */
#define XF86XK_Calculater	0x1008FF54   /* Launch Calculater           */
#define XF86XK_Clear		0x1008FF55   /* Clear window, screen        */
#define XF86XK_Close		0x1008FF56   /* Close window                */
#define XF86XK_Copy		0x1008FF57   /* Copy selection              */
#define XF86XK_Cut		0x1008FF58   /* Cut selection               */
#define XF86XK_Display		0x1008FF59   /* Output switch key           */
#define XF86XK_DOS		0x1008FF5A   /* Launch DOS (emulation)      */
#define XF86XK_Documents	0x1008FF5B   /* Open documents window       */
#define XF86XK_Excel		0x1008FF5C   /* Launch spread sheet         */
#define XF86XK_Explorer		0x1008FF5D   /* Launch file explorer        */
#define XF86XK_Game		0x1008FF5E   /* Launch game                 */
#define XF86XK_Go		0x1008FF5F   /* Go to URL                   */
#define XF86XK_iTouch		0x1008FF60   /* Logitech iTouch- don't use  */
#define XF86XK_LogOff		0x1008FF61   /* Log off system              */
#define XF86XK_Market		0x1008FF62   /* ??                          */
#define XF86XK_Meeting		0x1008FF63   /* enter meeting in calendar   */
#define XF86XK_MenuKB		0x1008FF65   /* distinguish keyboard from PB */
#define XF86XK_MenuPB		0x1008FF66   /* distinguish PB from keyboard */
#define XF86XK_MySites		0x1008FF67   /* Favourites                  */
#define XF86XK_New		0x1008FF68   /* New (folder, document...    */
#define XF86XK_News		0x1008FF69   /* News                        */
#define XF86XK_OfficeHome	0x1008FF6A   /* Office home (old Staroffice)*/
#define XF86XK_Open		0x1008FF6B   /* Open                        */
#define XF86XK_Option		0x1008FF6C   /* ?? */
#define XF86XK_Paste		0x1008FF6D   /* Paste                       */
#define XF86XK_Phone		0x1008FF6E   /* Launch phone; dial number   */
#define XF86XK_Q		0x1008FF70   /* Compaq's Q - don't use      */
#define XF86XK_Reply		0x1008FF72   /* Reply e.g., mail            */
#define XF86XK_Reload		0x1008FF73   /* Reload web page, file, etc. */
#define XF86XK_RotateWindows	0x1008FF74   /* Rotate windows e.g. xrandr  */
#define XF86XK_RotationPB	0x1008FF75   /* don't use                   */
#define XF86XK_RotationKB	0x1008FF76   /* don't use                   */
#define XF86XK_Save		0x1008FF77   /* Save (file, document, state */
#define XF86XK_ScrollUp		0x1008FF78   /* Scroll window/contents up   */
#define XF86XK_ScrollDown	0x1008FF79   /* Scrool window/contentd down */
#define XF86XK_ScrollClick	0x1008FF7A   /* Use XKB mousekeys instead   */
#define XF86XK_Send		0x1008FF7B   /* Send mail, file, object     */
#define XF86XK_Spell		0x1008FF7C   /* Spell checker               */
#define XF86XK_SplitScreen	0x1008FF7D   /* Split window or screen      */
#define XF86XK_Support		0x1008FF7E   /* Get support (??)            */
#define XF86XK_TaskPane		0x1008FF7F   /* Show tasks */
#define XF86XK_Terminal		0x1008FF80   /* Launch terminal emulator    */
#define XF86XK_Tools		0x1008FF81   /* toolbox of desktop/app.     */
#define XF86XK_Travel		0x1008FF82   /* ?? */
#define XF86XK_UserPB		0x1008FF84   /* ?? */
#define XF86XK_User1KB		0x1008FF85   /* ?? */
#define XF86XK_User2KB		0x1008FF86   /* ?? */
#define XF86XK_Video		0x1008FF87   /* Launch video player       */
#define XF86XK_WheelButton	0x1008FF88   /* button from a mouse wheel */
#define XF86XK_Word		0x1008FF89   /* Launch word processor     */
#define XF86XK_Xfer		0x1008FF8A
#define XF86XK_ZoomIn		0x1008FF8B   /* zoom in view, map, etc.   */
#define XF86XK_ZoomOut		0x1008FF8C   /* zoom out view, map, etc.  */

#define XF86XK_Away		0x1008FF8D   /* mark yourself as away     */
#define XF86XK_Messenger	0x1008FF8E   /* as in instant messaging   */
#define XF86XK_WebCam		0x1008FF8F   /* Launch web camera app.    */
#define XF86XK_MailForward	0x1008FF90   /* Forward in mail           */
#define XF86XK_Pictures		0x1008FF91   /* Show pictures             */
#define XF86XK_Music		0x1008FF92   /* Launch music application  */

#define XF86XK_Battery		0x1008FF93   /* Display battery information */
#define XF86XK_Bluetooth	0x1008FF94   /* Enable/disable Bluetooth    */
#define XF86XK_WLAN		0x1008FF95   /* Enable/disable WLAN         */
#define XF86XK_UWB		0x1008FF96   /* Enable/disable UWB	    */

#define XF86XK_AudioForward	0x1008FF97   /* fast-forward audio track    */
#define XF86XK_AudioRepeat	0x1008FF98   /* toggle repeat mode          */
#define XF86XK_AudioRandomPlay	0x1008FF99   /* toggle shuffle mode         */
#define XF86XK_Subtitle		0x1008FF9A   /* cycle through subtitle      */
#define XF86XK_AudioCycleTrack	0x1008FF9B   /* cycle through audio tracks  */
#define XF86XK_CycleAngle	0x1008FF9C   /* cycle through angles        */
#define XF86XK_FrameBack	0x1008FF9D   /* video: go one frame back    */
#define XF86XK_FrameForward	0x1008FF9E   /* video: go one frame forward */
#define XF86XK_Time		0x1008FF9F   /* display, or shows an entry for time seeking */
#define XF86XK_Select		0x1008FFA0   /* Select button on joypads and remotes */
#define XF86XK_View		0x1008FFA1   /* Show a view options/properties */
#define XF86XK_TopMenu		0x1008FFA2   /* Go to a top-level menu in a video */

#define XF86XK_Red		0x1008FFA3   /* Red button                  */
#define XF86XK_Green		0x1008FFA4   /* Green button                */
#define XF86XK_Yellow		0x1008FFA5   /* Yellow button               */
#define XF86XK_Blue             0x1008FFA6   /* Blue button                 */

#define XF86XK_Suspend		0x1008FFA7   /* Sleep to RAM                */
#define XF86XK_Hibernate	0x1008FFA8   /* Sleep to disk               */
#define XF86XK_TouchpadToggle	0x1008FFA9   /* Toggle between touchpad/trackstick */
#define XF86XK_TouchpadOn	0x1008FFB0   /* The touchpad got switched on */
#define XF86XK_TouchpadOff	0x1008FFB1   /* The touchpad got switched off */

#define XF86XK_AudioMicMute	0x1008FFB2   /* Mute the Mic from the system */

#define XF86XK_Keyboard		0x1008FFB3   /* User defined keyboard related action */

#define XF86XK_WWAN		0x1008FFB4   /* Toggle WWAN (LTE, UMTS, etc.) radio */
#define XF86XK_RFKill		0x1008FFB5   /* Toggle radios on/off */

#define XF86XK_AudioPreset	0x1008FFB6   /* Select equalizer preset, e.g. theatre-mode */

#define XF86XK_RotationLockToggle 0x1008FFB7 /* Toggle screen rotation lock on/off */

#define XF86XK_FullScreen	0x1008FFB8   /* Toggle fullscreen */

/* Keys for special action keys (hot keys) */
/* Virtual terminals on some operating systems */
#define XF86XK_Switch_VT_1	0x1008FE01
#define XF86XK_Switch_VT_2	0x1008FE02
#define XF86XK_Switch_VT_3	0x1008FE03
#define XF86XK_Switch_VT_4	0x1008FE04
#define XF86XK_Switch_VT_5	0x1008FE05
#define XF86XK_Switch_VT_6	0x1008FE06
#define XF86XK_Switch_VT_7	0x1008FE07
#define XF86XK_Switch_VT_8	0x1008FE08
#define XF86XK_Switch_VT_9	0x1008FE09
#define XF86XK_Switch_VT_10	0x1008FE0A
#define XF86XK_Switch_VT_11	0x1008FE0B
#define XF86XK_Switch_VT_12	0x1008FE0C

#define XF86XK_Ungrab		0x1008FE20   /* force ungrab               */
#define XF86XK_ClearGrab	0x1008FE21   /* kill application with grab */
#define XF86XK_Next_VMode	0x1008FE22   /* next video mode available  */
#define XF86XK_Prev_VMode	0x1008FE23   /* prev. video mode available */
#define XF86XK_LogWindowTree	0x1008FE24   /* print window tree to log   */
#define XF86XK_LogGrabInfo	0x1008FE25   /* print all active grabs to log */
