# Microsoft Developer Studio Project File - Name="freetype" - Package Owner=<4>
# Microsoft Developer Studio Generated Build File, Format Version 6.00
# ** DO NOT EDIT **

# TARGTYPE "Win32 (x86) Static Library" 0x0104

CFG=freetype - Win32 Debug Singlethreaded
!MESSAGE This is not a valid makefile. To build this project using NMAKE,
!MESSAGE use the Export Makefile command and run
!MESSAGE
!MESSAGE NMAKE /f "freetype.mak".
!MESSAGE
!MESSAGE You can specify a configuration when running NMAKE
!MESSAGE by defining the macro CFG on the command line. For example:
!MESSAGE
!MESSAGE NMAKE /f "freetype.mak" CFG="freetype - Win32 Debug Singlethreaded"
!MESSAGE
!MESSAGE Possible choices for configuration are:
!MESSAGE
!MESSAGE "freetype - Win32 Release" (based on "Win32 (x86) Static Library")
!MESSAGE "freetype - Win32 Debug" (based on "Win32 (x86) Static Library")
!MESSAGE "freetype - Win32 Debug Multithreaded" (based on "Win32 (x86) Static Library")
!MESSAGE "freetype - Win32 Release Multithreaded" (based on "Win32 (x86) Static Library")
!MESSAGE "freetype - Win32 Release Singlethreaded" (based on "Win32 (x86) Static Library")
!MESSAGE "freetype - Win32 Debug Singlethreaded" (based on "Win32 (x86) Static Library")
!MESSAGE

# Begin Project
# PROP AllowPerConfigDependencies 0
# PROP Scc_ProjName ""
# PROP Scc_LocalPath ""
CPP=cl.exe
RSC=rc.exe

!IF  "$(CFG)" == "freetype - Win32 Release"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 0
# PROP BASE Output_Dir "Release"
# PROP BASE Intermediate_Dir "Release"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 0
# PROP Output_Dir "..\..\..\objs\release"
# PROP Intermediate_Dir "..\..\..\objs\release"
# PROP Target_Dir ""
# ADD BASE CPP /nologo /W3 /GX /O2 /D "WIN32" /D "NDEBUG" /D "_MBCS" /D "_LIB" /YX /FD /c
# ADD CPP /MD /Za /W4 /GX /O2 /I "..\..\..\include" /D "NDEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT2_BUILD_LIBRARY" /FD /c
# SUBTRACT CPP /nologo /Z<none> /YX
# ADD BASE RSC /l 0x409 /d "NDEBUG"
# ADD RSC /l 0x409 /d "NDEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo
# ADD LIB32 /nologo /out:"..\..\..\objs\freetype.lib"

!ELSEIF  "$(CFG)" == "freetype - Win32 Debug"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 1
# PROP BASE Output_Dir "Debug"
# PROP BASE Intermediate_Dir "Debug"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 1
# PROP Output_Dir "..\..\..\objs\debug"
# PROP Intermediate_Dir "..\..\..\objs\debug"
# PROP Target_Dir ""
# ADD BASE CPP /nologo /W3 /Gm /GX /ZI /Od /D "WIN32" /D "_DEBUG" /D "_MBCS" /D "_LIB" /YX /FD /GZ /c
# ADD CPP /MDd /Za /W4 /GX /Z7 /Od /I "..\..\..\include" /D "_DEBUG" /D "FT_DEBUG_LEVEL_ERROR" /D "FT_DEBUG_LEVEL_TRACE" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT2_BUILD_LIBRARY" /FD /GZ /c
# SUBTRACT CPP /nologo /X /YX
# ADD BASE RSC /l 0x409 /d "_DEBUG"
# ADD RSC /l 0x409 /d "_DEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo
# ADD LIB32 /nologo /out:"..\..\..\objs\freetype_D.lib"

!ELSEIF  "$(CFG)" == "freetype - Win32 Debug Multithreaded"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 1
# PROP BASE Output_Dir "freetype___Win32_Debug_Multithreaded"
# PROP BASE Intermediate_Dir "freetype___Win32_Debug_Multithreaded"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 1
# PROP Output_Dir "..\..\..\objs\debug_mt"
# PROP Intermediate_Dir "..\..\..\objs\debug_mt"
# PROP Target_Dir ""
# ADD BASE CPP /nologo /Za /W3 /Gm /GX /ZI /Od /I "..\include\\" /D "_DEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT_FLAT_COMPILE" /YX /FD /GZ /c
# SUBTRACT BASE CPP /X
# ADD CPP /MTd /Za /W4 /GX /Z7 /Od /I "..\..\..\include" /D "_DEBUG" /D "FT_DEBUG_LEVEL_ERROR" /D "FT_DEBUG_LEVEL_TRACE" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT2_BUILD_LIBRARY" /FD /GZ /c
# SUBTRACT CPP /nologo /X /YX
# ADD BASE RSC /l 0x409 /d "_DEBUG"
# ADD RSC /l 0x409 /d "_DEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo /out:"lib\freetype_D.lib"
# ADD LIB32 /nologo /out:"..\..\..\objs\freetypeMT_D.lib"

!ELSEIF  "$(CFG)" == "freetype - Win32 Release Multithreaded"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 0
# PROP BASE Output_Dir "freetype___Win32_Release_Multithreaded"
# PROP BASE Intermediate_Dir "freetype___Win32_Release_Multithreaded"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 0
# PROP Output_Dir "..\..\..\objs\release_mt"
# PROP Intermediate_Dir "..\..\..\objs\release_mt"
# PROP Target_Dir ""
# ADD BASE CPP /nologo /Za /W3 /GX /O2 /I "..\include\\" /D "NDEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT_FLAT_COMPILE" /YX /FD /c
# ADD CPP /MT /Za /W4 /GX /O2 /I "..\..\..\include" /D "NDEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT2_BUILD_LIBRARY" /FD /c
# SUBTRACT CPP /nologo /Z<none> /YX
# ADD BASE RSC /l 0x409 /d "NDEBUG"
# ADD RSC /l 0x409 /d "NDEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo /out:"lib\freetype.lib"
# ADD LIB32 /nologo /out:"..\..\..\objs\freetypeMT.lib"

!ELSEIF  "$(CFG)" == "freetype - Win32 Release Singlethreaded"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 0
# PROP BASE Output_Dir "freetype___Win32_Release_Singlethreaded"
# PROP BASE Intermediate_Dir "freetype___Win32_Release_Singlethreaded"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 0
# PROP Output_Dir "..\..\..\objs\release_st"
# PROP Intermediate_Dir "..\..\..\objs\release_st"
# PROP Target_Dir ""
# ADD BASE CPP /nologo /MD /Za /W4 /GX /Zi /O2 /I "..\..\..\include" /D "NDEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /FD /c
# SUBTRACT BASE CPP /YX
# ADD CPP /Za /W4 /GX /O2 /I "..\..\..\include" /D "NDEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT2_BUILD_LIBRARY" /FD /c
# SUBTRACT CPP /nologo /Z<none> /YX
# ADD BASE RSC /l 0x409 /d "NDEBUG"
# ADD RSC /l 0x409 /d "NDEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo /out:"..\..\..\objs\freetype.lib"
# ADD LIB32 /out:"..\..\..\objs\freetypeST.lib"
# SUBTRACT LIB32 /nologo

!ELSEIF  "$(CFG)" == "freetype - Win32 Debug Singlethreaded"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 1
# PROP BASE Output_Dir "freetype___Win32_Debug_Singlethreaded"
# PROP BASE Intermediate_Dir "freetype___Win32_Debug_Singlethreaded"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 1
# PROP Output_Dir "..\..\..\objs\debug_st"
# PROP Intermediate_Dir "..\..\..\objs\debug_st"
# PROP Target_Dir ""
# ADD BASE CPP /nologo /MDd /Za /W4 /Gm /GX /Zi /Od /I "..\..\..\include" /D "_DEBUG" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT_DEBUG_LEVEL_ERROR" /D "FT_DEBUG_LEVEL_TRACE" /FD /GZ /c
# SUBTRACT BASE CPP /X /YX
# ADD CPP /Za /W4 /GX /Z7 /Od /I "..\..\..\include" /D "_DEBUG" /D "FT_DEBUG_LEVEL_ERROR" /D "FT_DEBUG_LEVEL_TRACE" /D "WIN32" /D "_MBCS" /D "_LIB" /D "FT2_BUILD_LIBRARY" /FD /GZ /c
# SUBTRACT CPP /nologo /X /YX
# ADD BASE RSC /l 0x409 /d "_DEBUG"
# ADD RSC /l 0x409 /d "_DEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo /out:"..\..\..\objs\freetype_D.lib"
# ADD LIB32 /nologo /out:"..\..\..\objs\freetypeST_D.lib"

!ENDIF

# Begin Target

# Name "freetype - Win32 Release"
# Name "freetype - Win32 Debug"
# Name "freetype - Win32 Debug Multithreaded"
# Name "freetype - Win32 Release Multithreaded"
# Name "freetype - Win32 Release Singlethreaded"
# Name "freetype - Win32 Debug Singlethreaded"
# Begin Group "Source Files"

# PROP Default_Filter "cpp;c;cxx;rc;def;r;odl;idl;hpj;bat"
# Begin Source File

SOURCE=..\..\..\src\autofit\autofit.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\bdf\bdf.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\cff\cff.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftbase.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftbbox.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftbdf.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftbitmap.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftcid.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftfstype.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftgasp.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\cache\ftcache.c
# End Source File
# Begin Source File

SOURCE=..\ftdebug.c
# ADD CPP /Ze
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftglyph.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftgxval.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\gzip\ftgzip.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftinit.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\lzw\ftlzw.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftmm.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftotval.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftpatent.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftpfr.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftstroke.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftsynth.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftsystem.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\fttype1.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\base\ftwinfnt.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\pcf\pcf.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\pfr\pfr.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\psaux\psaux.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\pshinter\pshinter.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\psnames\psmodule.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\raster\raster.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\sfnt\sfnt.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\smooth\smooth.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\truetype\truetype.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\type1\type1.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\cid\type1cid.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\type42\type42.c
# End Source File
# Begin Source File

SOURCE=..\..\..\src\winfonts\winfnt.c
# End Source File
# End Group
# Begin Group "Header Files"

# PROP Default_Filter "h;hpp;hxx;hm;inl"
# Begin Source File

SOURCE=..\..\..\include\ft2build.h
# End Source File
# Begin Source File

SOURCE=..\..\..\include\freetype\config\ftconfig.h
# End Source File
# Begin Source File

SOURCE=..\..\..\include\freetype\config\ftheader.h
# End Source File
# Begin Source File

SOURCE=..\..\..\include\freetype\config\ftmodule.h
# End Source File
# Begin Source File

SOURCE=..\..\..\include\freetype\config\ftoption.h
# End Source File
# Begin Source File

SOURCE=..\..\..\include\freetype\config\ftstdlib.h
# End Source File
# End Group
# Begin Group "Resource Files"

# PROP Default_Filter "rc;ico;cur;bmp;dlg;rc2;rct;bin;rgs;gif;jpg;jpeg;jpe;resx"
# Begin Source File

SOURCE=..\..\..\src\base\ftver.rc
# End Source File
# End Group
# End Target
# End Project
