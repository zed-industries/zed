#ifndef __XCB_ATOM_H__
#define __XCB_ATOM_H__

#include <xcb/xcb.h>

#ifdef __cplusplus
extern "C" {
#endif

char *xcb_atom_name_by_screen(const char *base, uint8_t screen);
char *xcb_atom_name_by_resource(const char *base, uint32_t resource);
char *xcb_atom_name_unique(const char *base, uint32_t id);

#ifdef __cplusplus
}
#endif

#endif /* __XCB_ATOM_H__ */
