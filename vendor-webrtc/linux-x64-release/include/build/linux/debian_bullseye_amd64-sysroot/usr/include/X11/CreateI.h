#ifndef _XtcreateI_h
#define _XtcreateI_h

_XFUNCPROTOBEGIN

extern Widget _XtCreateWidget(String name, WidgetClass widget_class,
			      Widget parent, ArgList args, Cardinal num_args,
			      XtTypedArgList typed_args,
			      Cardinal num_typed_args);
extern Widget _XtCreatePopupShell(String name, WidgetClass widget_class,
				  Widget parent, ArgList args,
				  Cardinal num_args, XtTypedArgList typed_args,
				  Cardinal num_typed_args);
extern Widget _XtAppCreateShell(String name, String class,
				WidgetClass widget_class, Display *display,
				ArgList args, Cardinal num_args,
				XtTypedArgList typed_args,
				Cardinal num_typed_args);
extern Widget _XtCreateHookObj(Screen *screen);

_XFUNCPROTOEND

#include <stdarg.h>

_XFUNCPROTOBEGIN

/* VarCreate.c */
extern Widget _XtVaOpenApplication(XtAppContext *app_context_return,
			_Xconst char* application_class,
			XrmOptionDescList options, Cardinal num_options,
			int *argc_in_out, _XtString *argv_in_out,
			String *fallback_resources, WidgetClass widget_class,
			va_list var_args);
extern Widget _XtVaAppInitialize(XtAppContext *app_context_return,
			_Xconst char* application_class,
			XrmOptionDescList options, Cardinal num_options,
			int *argc_in_out, _XtString *argv_in_out,
			String *fallback_resources, va_list var_args);

_XFUNCPROTOEND

#endif /* _XtcreateI_h */
