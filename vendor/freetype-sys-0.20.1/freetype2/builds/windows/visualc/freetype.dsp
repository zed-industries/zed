# Microsoft Developer Studio Project File - Name="freetype" - Package Owner=<4>
# Microsoft Developer Studio Generated Build File, Format Version 6.00
# ** DO NOT EDIT **

# TARGTYPE "Win32 (x86) Dynamic-Link Library" 0x0102
# TARGTYPE "Win32 (x86) Static Library" 0x0104

CFG=freetype - Win32 Release
!MESSAGE This is not a valid makefile. To build this project using NMAKE,
!MESSAGE use the Export Makefile command and run
!MESSAGE
!MESSAGE NMAKE /f "freetype.mak".
!MESSAGE
!MESSAGE You can specify a configuration when running NMAKE
!MESSAGE by defining the macro CFG on the command line. For example:
!MESSAGE
!MESSAGE NMAKE /f "freetype.mak" CFG="freetype - Win32 Release"
!MESSAGE
!MESSAGE Possible choices for configuration are:
!MESSAGE
!MESSAGE "freetype - Win32 Release" (based on "Win32 (x86) Dynamic-Link Library")
!MESSAGE "freetype - Win32 Debug" (based on "Win32 (x86) Dynamic-Link Library")
!MESSAGE "freetype - Win32 Release Static" (based on "Win32 (x86) Static Library")
!MESSAGE "freetype - Win32 Debug Static" (based on "Win32 (x86) Static Library")
!MESSAGE

# Begin Project
# PROP AllowPerConfigDependencies 0
# PROP Scc_ProjName ""
# PROP Scc_LocalPath ""

!IF  "$(CFG)" == "freetype - Win32 Release"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 0
# PROP BASE Output_Dir "Release"
# PROP BASE Intermediate_Dir "Release"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 0
# PROP Output_Dir "..\..\..\objs\Win32\Release"
# PROP Intermediate_Dir "..\..\..\objs\Win32\Release"
# PROP Ignore_Export_Lib 0
# PROP Target_Dir ""
CPP=cl.exe
# ADD BASE CPP /nologo /MD /W3 /O2 /D "WIN32" /D "NDEBUG" /FD /c
# SUBTRACT BASE CPP /YX /Yc /Yu
# ADD CPP /nologo /Za /MD /W3 /O2 /Oi /D "WIN32" /I "..\..\..\include" /D "_CRT_SECURE_NO_WARNINGS" /D "NDEBUG" /D "FT2_BUILD_LIBRARY" /D "DLL_EXPORT" /FD /c
# SUBTRACT CPP /YX /Yc /Yu
MTL=midl.exe
# ADD BASE MTL /nologo /D "NDEBUG" /mktyplib203 /win32
# ADD MTL /nologo /D "NDEBUG" /mktyplib203 /win32
RSC=rc.exe
# ADD BASE RSC /l 0x409 /d "NDEBUG"
# ADD RSC /l 0x409 /d "NDEBUG" /d "DLL_EXPORT"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LINK32=link.exe
# ADD BASE LINK32 kernel32.lib user32.lib gdi32.lib winspool.lib comdlg32.lib advapi32.lib shell32.lib ole32.lib oleaut32.lib uuid.lib odbc32.lib odbccp32.lib /nologo /dll /machine:I386
# ADD LINK32 /nologo /dll /machine:I386 /opt:REF,ICF /out:"$(OutDir)\freetype.dll"

!ELSEIF  "$(CFG)" == "freetype - Win32 Debug"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 1
# PROP BASE Output_Dir "Debug"
# PROP BASE Intermediate_Dir "Debug"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 1
# PROP Output_Dir "..\..\..\objs\Win32\Debug"
# PROP Intermediate_Dir "..\..\..\objs\Win32\Debug"
# PROP Ignore_Export_Lib 0
# PROP Target_Dir ""
CPP=cl.exe
# ADD BASE CPP /nologo /MDd /W3 /Gm /ZI /Od /D "WIN32" /D "_DEBUG" /FD /GZ /c
# SUBTRACT BASE CPP /YX /Yc /Yu
# ADD CPP /nologo /Za /MDd /W3 /Gm /ZI /Od /I "..\..\..\include" /D "WIN32" /D "_CRT_SECURE_NO_WARNINGS" /D "_DEBUG" /D "FT_DEBUG_LEVEL_ERROR" /D "FT_DEBUG_LEVEL_TRACE" /D "FT2_BUILD_LIBRARY" /D "DLL_EXPORT" /FR /FD /GZ /c
# SUBTRACT CPP /YX /Yc /Yu
MTL=midl.exe
# ADD BASE MTL /nologo /D "_DEBUG" /mktyplib203 /win32
# ADD MTL /nologo /D "_DEBUG" /mktyplib203 /win32
RSC=rc.exe
# ADD BASE RSC /l 0x409 /d "_DEBUG"
# ADD RSC /l 0x409 /d "_DEBUG" /d "DLL_EXPORT"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LINK32=link.exe
# ADD BASE LINK32 kernel32.lib user32.lib gdi32.lib winspool.lib comdlg32.lib advapi32.lib shell32.lib ole32.lib oleaut32.lib uuid.lib odbc32.lib odbccp32.lib /nologo /dll /debug /machine:I386 /pdbtype:sept
# ADD LINK32 /nologo /dll /debug /machine:I386 /out:"$(OutDir)\freetype.dll" /pdbtype:sept

!ELSEIF  "$(CFG)" == "freetype - Win32 Release Static"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 0
# PROP BASE Output_Dir "Release Static"
# PROP BASE Intermediate_Dir "Release Static"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 0
# PROP Output_Dir "..\..\..\objs\Win32\Release Static"
# PROP Intermediate_Dir "..\..\..\objs\Win32\Release Static"
# PROP Target_Dir ""
CPP=cl.exe
# ADD BASE CPP /nologo /MD /W3 /O2 /D "WIN32" /D "NDEBUG" /FD /c
# SUBTRACT BASE CPP /YX /Yc /Yu
# ADD CPP /nologo /Za /MD /W3 /O2 /Oi /D "WIN32" /I "..\..\..\include" /D "_CRT_SECURE_NO_WARNINGS" /D "NDEBUG" /D "FT2_BUILD_LIBRARY" /FD /c
# SUBTRACT CPP /YX /Yc /Yu
RSC=rc.exe
# ADD BASE RSC /l 0x409 /d "NDEBUG"
# ADD RSC /l 0x409 /d "NDEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo
# ADD LIB32 /nologo /out:"$(OutDir)\freetype.lib"

!ELSEIF  "$(CFG)" == "freetype - Win32 Debug Static"

# PROP BASE Use_MFC 0
# PROP BASE Use_Debug_Libraries 1
# PROP BASE Output_Dir "Debug Static"
# PROP BASE Intermediate_Dir "Debug Static"
# PROP BASE Target_Dir ""
# PROP Use_MFC 0
# PROP Use_Debug_Libraries 1
# PROP Output_Dir "..\..\..\objs\Win32\Debug Static"
# PROP Intermediate_Dir "..\..\..\objs\Win32\Debug Static"
# PROP Target_Dir ""
CPP=cl.exe
# ADD BASE CPP /nologo /MDd /W3 /Gm /ZI /Od /D "WIN32" /D "_DEBUG" /FD /GZ /c
# SUBTRACT BASE CPP /YX /Yc /Yu
# ADD CPP /nologo /Za /MDd /W3 /Gm /ZI /Od /I "..\..\..\include" /D "WIN32" /D "_CRT_SECURE_NO_WARNINGS" /D "_DEBUG" /D "FT_DEBUG_LEVEL_ERROR" /D "FT_DEBUG_LEVEL_TRACE" /D "FT2_BUILD_LIBRARY" /FR /FD /GZ /c
# SUBTRACT CPP /YX /Yc /Yu
RSC=rc.exe
# ADD BASE RSC /l 0x409 /d "_DEBUG"
# ADD RSC /l 0x409 /d "_DEBUG"
BSC32=bscmake.exe
# ADD BASE BSC32 /nologo
# ADD BSC32 /nologo
LIB32=link.exe -lib
# ADD BASE LIB32 /nologo
# ADD LIB32 /nologo /out:"$(OutDir)\freetype.lib"

!ENDIF

# Begin Target

# Name "freetype - Win32 Release"
# Name "freetype - Win32 Debug"
# Name "freetype - Win32 Release Static"
# Name "freetype - Win32 Debug Static"
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

SOURCE=..\..\..\src\base\ftpatent.c
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
