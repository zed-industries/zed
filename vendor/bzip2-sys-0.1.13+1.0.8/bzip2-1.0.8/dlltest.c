/*
   minibz2
      libbz2.dll test program.
      by Yoshioka Tsuneo (tsuneo@rr.iij4u.or.jp)
      This file is Public Domain.  Welcome any email to me.

   usage: minibz2 [-d] [-{1,2,..9}] [[srcfilename] destfilename]
*/

#define BZ_IMPORT
#include <stdio.h>
#include <stdlib.h>
#include "bzlib.h"
#ifdef _WIN32
#include <io.h>
#endif


#ifdef _WIN32

#define BZ2_LIBNAME "libbz2-1.0.2.DLL" 

#include <windows.h>
static int BZ2DLLLoaded = 0;
static HINSTANCE BZ2DLLhLib;
int BZ2DLLLoadLibrary(void)
{
   HINSTANCE hLib;

   if(BZ2DLLLoaded==1){return 0;}
   hLib=LoadLibrary(BZ2_LIBNAME);
   if(hLib == NULL){
      fprintf(stderr,"Can't load %s\n",BZ2_LIBNAME);
      return -1;
   }
   BZ2_bzlibVersion=GetProcAddress(hLib,"BZ2_bzlibVersion");
   BZ2_bzopen=GetProcAddress(hLib,"BZ2_bzopen");
   BZ2_bzdopen=GetProcAddress(hLib,"BZ2_bzdopen");
   BZ2_bzread=GetProcAddress(hLib,"BZ2_bzread");
   BZ2_bzwrite=GetProcAddress(hLib,"BZ2_bzwrite");
   BZ2_bzflush=GetProcAddress(hLib,"BZ2_bzflush");
   BZ2_bzclose=GetProcAddress(hLib,"BZ2_bzclose");
   BZ2_bzerror=GetProcAddress(hLib,"BZ2_bzerror");

   if (!BZ2_bzlibVersion || !BZ2_bzopen || !BZ2_bzdopen
       || !BZ2_bzread || !BZ2_bzwrite || !BZ2_bzflush
       || !BZ2_bzclose || !BZ2_bzerror) {
      fprintf(stderr,"GetProcAddress failed.\n");
      return -1;
   }
   BZ2DLLLoaded=1;
   BZ2DLLhLib=hLib;
   return 0;

}
int BZ2DLLFreeLibrary(void)
{
   if(BZ2DLLLoaded==0){return 0;}
   FreeLibrary(BZ2DLLhLib);
   BZ2DLLLoaded=0;
}
#endif /* WIN32 */

void usage(void)
{
   puts("usage: minibz2 [-d] [-{1,2,..9}] [[srcfilename] destfilename]");
}

int main(int argc,char *argv[])
{
   int decompress = 0;
   int level = 9;
   char *fn_r = NULL;
   char *fn_w = NULL;

#ifdef _WIN32
   if(BZ2DLLLoadLibrary()<0){
      fprintf(stderr,"Loading of %s failed.  Giving up.\n", BZ2_LIBNAME);
      exit(1);
   }
   printf("Loading of %s succeeded.  Library version is %s.\n",
          BZ2_LIBNAME, BZ2_bzlibVersion() );
#endif
   while(++argv,--argc){
      if(**argv =='-' || **argv=='/'){
         char *p;

         for(p=*argv+1;*p;p++){
            if(*p=='d'){
               decompress = 1;
            }else if('1'<=*p && *p<='9'){
               level = *p - '0';
            }else{
               usage();
               exit(1);
            }
         }
      }else{
         break;
      }
   }
   if(argc>=1){
      fn_r = *argv;
      argc--;argv++;
   }else{
      fn_r = NULL;
   }
   if(argc>=1){
      fn_w = *argv;
      argc--;argv++;
   }else{
      fn_w = NULL;
   }
   {
      int len;
      char buff[0x1000];
      char mode[10];

      if(decompress){
         BZFILE *BZ2fp_r = NULL;
         FILE *fp_w = NULL;

         if(fn_w){
            if((fp_w = fopen(fn_w,"wb"))==NULL){
               printf("can't open [%s]\n",fn_w);
               perror("reason:");
               exit(1);
            }
         }else{
            fp_w = stdout;
         }
         if((fn_r == NULL && (BZ2fp_r = BZ2_bzdopen(fileno(stdin),"rb"))==NULL)
            || (fn_r != NULL && (BZ2fp_r = BZ2_bzopen(fn_r,"rb"))==NULL)){
            printf("can't bz2openstream\n");
            exit(1);
         }
         while((len=BZ2_bzread(BZ2fp_r,buff,0x1000))>0){
            fwrite(buff,1,len,fp_w);
         }
         BZ2_bzclose(BZ2fp_r);
         if(fp_w != stdout) fclose(fp_w);
      }else{
         BZFILE *BZ2fp_w = NULL;
         FILE *fp_r = NULL;

         if(fn_r){
            if((fp_r = fopen(fn_r,"rb"))==NULL){
               printf("can't open [%s]\n",fn_r);
               perror("reason:");
               exit(1);
            }
         }else{
            fp_r = stdin;
         }
         mode[0]='w';
         mode[1] = '0' + level;
         mode[2] = '\0';

         if((fn_w == NULL && (BZ2fp_w = BZ2_bzdopen(fileno(stdout),mode))==NULL)
            || (fn_w !=NULL && (BZ2fp_w = BZ2_bzopen(fn_w,mode))==NULL)){
            printf("can't bz2openstream\n");
            exit(1);
         }
         while((len=fread(buff,1,0x1000,fp_r))>0){
            BZ2_bzwrite(BZ2fp_w,buff,len);
         }
         BZ2_bzclose(BZ2fp_w);
         if(fp_r!=stdin)fclose(fp_r);
      }
   }
#ifdef _WIN32
   BZ2DLLFreeLibrary();
#endif
   return 0;
}
